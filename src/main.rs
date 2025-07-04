#![allow(unused)]
use actix_web::web;
use pulldown_cmark;
use std::error::Error;
use std::fs;
mod args;
use actix_files::Files;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;
use ammonia::clean;
use args::MdwatchArgs;
use askama::Template;
use clap::Parser;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use webbrowser;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    content: String,
}

#[get("/")]
async fn home(file: web::Data<Arc<Mutex<String>>>) -> actix_web::Result<HttpResponse> {
    let locked_file = file.lock().unwrap();
    let file_path = locked_file.clone();
    let mut markdown_input: String =
        fs::read_to_string(file_path).map_err(actix_web::error::ErrorInternalServerError)?;
    let parser = pulldown_cmark::Parser::new(&markdown_input);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output = clean(&html_output);

    let template = Home {
        content: html_output,
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = MdwatchArgs::parse();

    let file = Arc::new(Mutex::new(String::new()));
    let port = Arc::new(AtomicU16::new(0));
    let ip = Arc::new(Mutex::new(String::new()));

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

    if ip.lock().unwrap().as_str() == "0.0.0.0" {
        eprintln!("⚠️ Warning: Binding to 0.0.0.0 exposes your server to the entire network!");
        eprintln!("         Make sure you trust your network or firewall settings.");
    } else {
        if webbrowser::open(format!("http://localhost:{}/", port.load(Ordering::SeqCst)).as_str())
            .is_ok()
        {}
    }

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/static", "./static"))
            .service(home)
            .app_data(web::Data::new(Arc::clone(&file)))
    })
    .bind((
        ip_clone.lock().unwrap().clone(),
        port_clone.load(Ordering::SeqCst),
    ))?
    .run()
    .await
}
