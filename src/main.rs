#![allow(unused)]
use actix_web::web;
use notify::event::RemoveKind;
use notify_debouncer_full::DebouncedEvent;
use notify_debouncer_full::{DebounceEventResult, new_debouncer, notify::*};
use pulldown_cmark::Options;
use std::fs;
use std::path::Path;
use tokio::time::Duration;
mod args;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;
use actix_web::{HttpRequest, HttpResponse};
use ammonia::clean;
use args::MdwatchArgs;
use askama::Template;
use clap::Parser;

use notify::{Event, RecursiveMode, Result, Watcher, event::ModifyKind};
use rust_embed::Embed;
use tokio::sync::mpsc;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/static"]
#[prefix = "static/"]
struct Static;

impl Static {
    fn get_styles() -> String {
        match Static::get("static/global.css") {
            Some(file) => match std::str::from_utf8(&file.data) {
                Ok(css) => css.to_string(),
                Err(e) => {
                    eprintln!("Failed to read CSS file: {e}");
                    String::new()
                }
            },
            None => {
                eprintln!("CSS file not found in embedded assets.");
                String::new()
            }
        }
    }

    fn get_scripts() -> String {
        match Static::get("static/client.js") {
            Some(file) => match std::str::from_utf8(&file.data) {
                Ok(js) => js.to_string(),
                Err(e) => {
                    eprintln!("Failed to read JS file: {e}");
                    String::new()
                }
            },
            None => {
                eprintln!("JS file not found in embedded assets.");
                String::new()
            }
        }
    }
}

async fn ws_handler(
    req: HttpRequest,
    body: web::Payload,
    file: web::Data<String>,
) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut _msg_stream) = actix_ws::handle(&req, body)?;
    let file_path = file.as_str().to_string();
    let (watch_tx, mut notify_rx) = mpsc::unbounded_channel::<DebouncedEvent>();

    let mut debouncer = new_debouncer(
        Duration::from_secs(2),
        None,
        move |result: DebounceEventResult| match result {
            Ok(events) => events.into_iter().for_each(|event| {
                let _ = watch_tx.send(event);
            }),
            Err(errors) => errors
                .iter()
                .for_each(|error| eprintln!("watch error: {error:?}")),
        },
    )
    .map_err(actix_web::error::ErrorInternalServerError)?;

    debouncer
        .watch(&file_path, RecursiveMode::NonRecursive)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    actix_web::rt::spawn(async move {
        // Keep the watcher alive in this async task to keep the msg_stream alive
        let _watcher = debouncer;

        while let Some(event) = notify_rx.recv().await {
            if matches!(event.kind, EventKind::Remove(RemoveKind::File)) {
                eprintln!("File removed: {}", file_path);
                break;
            }
            if matches!(event.kind, EventKind::Modify(ModifyKind::Data(_))) {
                println!("File modified");
                let latest_markdown = match get_markdown(&file_path) {
                    Ok(md) => md,
                    Err(e) => {
                        eprintln!("Error reading markdown file: {e}");
                        continue;
                    }
                };
                if session.text(latest_markdown).await.is_err() {
                    break;
                }
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}

pub fn get_markdown(file_path: &String) -> std::io::Result<String> {
    let markdown_input: String = fs::read_to_string(file_path)?;
    let options = Options::all();
    let parser = pulldown_cmark::Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output = clean(&html_output);
    Ok(html_output)
}

#[derive(Template)]
#[template(path = "main.html")]
pub struct Mdwatch {
    pub content: String,
    pub title: String,
    pub style: String,
    pub script: String,
}

#[get("/")]
async fn home(file: web::Data<String>) -> actix_web::Result<HttpResponse> {
    let file_path = Path::new(file.as_str());

    let file_name = match file_path.file_name() {
        Some(name) => name,
        None => {
            return Err(actix_web::error::ErrorInternalServerError(
                "Failed to get file name",
            ));
        }
    };

    if let Some(extension) = file_path.extension()
        && extension != "md"
    {
        eprintln!(
            "Warning: Unsupported file type: .{}",
            extension.to_string_lossy()
        );
        return Err(actix_web::error::ErrorInternalServerError(
            "Unsupported file type. Please provide a markdown (.md) file.",
        ));
    };

    let html_output = match get_markdown(&file.as_str().to_string()) {
        Ok(html) => html,
        Err(e) => {
            eprintln!("Error processing markdown file: {e}");
            return Err(actix_web::error::ErrorInternalServerError(
                "Failed to process markdown file",
            ));
        }
    };

    let template = Mdwatch {
        content: html_output,
        title: file_name.to_string_lossy().to_string(),
        style: Static::get_styles(),
        script: Static::get_scripts(),
    };

    match template.render() {
        Ok(rendered) => Ok(HttpResponse::Ok().content_type("text/html").body(rendered)),
        Err(e) => {
            eprintln!("Template rendering error: {e}");

            Ok(HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body("Failed to render template"))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = MdwatchArgs::parse();

    let MdwatchArgs { file, ip, port } = args;

    if ip == "0.0.0.0" {
        eprintln!("  Warning: Binding to 0.0.0.0 exposes your server to the entire network!");
        eprintln!("         Make sure you trust your network or firewall settings.");
    }

    println!("Server running at:");
    println!(" - http://{}:{}/", ip, port);

    if let Err(e) = webbrowser::open(&format!("http://localhost:{}/", port)) {
        eprintln!("Failed to open browser: {e}");
    }

    HttpServer::new(move || {
        App::new()
            .route("/ws", web::get().to(ws_handler))
            .service(home)
            .app_data(web::Data::new(file.clone()))
    })
    .bind(format!("{}:{}", ip, port))?
    .run()
    .await
}
