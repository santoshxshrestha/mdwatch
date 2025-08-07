#![allow(unused)]
mod template;
use actix_web::web;
use pulldown_cmark;
use pulldown_cmark::Options;
use std::fs;
use std::path::Path;
use std::sync::atomic::AtomicU64;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use template::Home;
mod args;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::get;
use ammonia::clean;
use args::MdwatchArgs;
use askama::Template;
use clap::Parser;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use webbrowser;

#[get("/")]
async fn home(
    file: web::Data<Arc<Mutex<String>>>,
    last_modified: web::Data<Arc<AtomicU64>>,
) -> actix_web::Result<HttpResponse> {
    let locked_file = file.lock().unwrap();
    let file_path = locked_file.clone();
    let file = Path::new(&file_path).file_name().unwrap();
    let markdown_input: String = fs::read_to_string(file_path.clone())
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let mut options = Options::all();
    let parser = pulldown_cmark::Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output = clean(&html_output);

    let template = Home {
        content: html_output,
        last_modified: last_modified.load(Ordering::SeqCst),
        title: file.to_string_lossy().to_string(),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

#[get("/api/check-update")]
async fn check_update(file: web::Data<Arc<Mutex<String>>>) -> actix_web::Result<HttpResponse> {
    let locked_file = file.lock().unwrap();
    let file_path = locked_file.clone();

    match fs::metadata(&file_path) {
        Ok(metadata) => {
            let modified_time = metadata
                .modified()
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            Ok(HttpResponse::Ok().json(serde_json::json!({
                "last_modified": modified_time
            })))
        }
        Err(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "last_modified": 0
        }))),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = MdwatchArgs::parse();

    let file = Arc::new(Mutex::new(String::new()));
    let port = Arc::new(AtomicU16::new(0));
    let ip = Arc::new(Mutex::new(String::new()));
    let last_modified = Arc::new(AtomicU64::new(0));

    match args {
        MdwatchArgs {
            file: f,
            ip: i,
            port: p,
        } => {
            *file.lock().unwrap() = f;
            *ip.lock().unwrap() = i;
            port.store(p, Ordering::SeqCst);
        }
    }
    let ip_clone = Arc::clone(&ip);
    let port_clone = Arc::clone(&port);
    let last_modified_clone = Arc::clone(&last_modified);

    if ip.lock().unwrap().as_str() == "0.0.0.0" {
        eprintln!("⚠️ Warning: Binding to 0.0.0.0 exposes your server to the entire network!");
        eprintln!("         Make sure you trust your network or firewall settings.");
    }

    println!("Server running at:");
    println!(
        " - localhost: http://{}:{}/",
        ip.lock().unwrap(),
        port.load(Ordering::SeqCst)
    );

    let _ = webbrowser::open(format!("http://localhost:{}/", port.load(Ordering::SeqCst)).as_str());

    HttpServer::new(move || {
        App::new()
            .service(home)
            .service(check_update)
            .app_data(web::Data::new(last_modified_clone.clone()))
            .app_data(web::Data::new(Arc::clone(&file)))
    })
    .bind((
        ip_clone.lock().unwrap().clone(),
        port_clone.load(Ordering::SeqCst),
    ))?
    .run()
    .await
}
