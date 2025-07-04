#![allow(unused)]
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

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    content: String,
}

#[get("/")]
async fn home() -> impl Responder {
    let template = Home {
        content: String::from("Hello there this is the string out there "),
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

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/static", "./static"))
            .service(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
