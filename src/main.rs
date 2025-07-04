#![allow(unused)]
use actix_files::Files;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;
use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
struct Home;

#[get("/")]
async fn home() -> impl Responder {
    let template = Home;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(Files::new("/static", "./static"))
            .service(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
