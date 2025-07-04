#![allow(unused)]
use actix_web::web;
use pulldown_cmark;
mod args;
use actix_files::Files;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;
use args::MdwatchArgs;
use askama::Template;
use clap::Parser;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use webbrowser::open;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    content: String,
}

#[get("/")]
async fn home() -> impl Responder {
    let markdown_input = "hello world";
    let parser = pulldown_cmark::Parser::new(markdown_input);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let template = Home {
        content: html_output,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
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
