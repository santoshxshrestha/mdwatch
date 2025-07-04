use actix_web::App;
use actix_web::HttpServer;
#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static"))
            .service(hello)
    })
}
