use actix_web::web;
use pulldown_cmark;
use std::fs;
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

#[derive(Template)]
#[template(
    ext = "html",
    source = "
<!doctype html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
    <title>mdserve</title>
    <style>
body {
    max-width: 720px;
    margin: auto;
    padding: 2rem;
    font-family: system-ui, sans-serif;
    line-height: 1.6;
    background-color: #181A1B;
    color: #D3CFC9;
}

h1, h2, h3 {
    color: #E0E0E0; 
}

code {
    background: #2D2F30;
    padding: 0.2em 0.4em;
    border-radius: 4px;
    font-family: monospace;
    color: #F8F8F2;
}

pre {
    background: #2D2F30;
    padding: 1em;
    overflow-x: auto;
    border-radius: 4px;
    color: #F8F8F2;
}

blockquote {
    border-left: 4px solid #555;
    padding-left: 1em;
    color: #AAA;
    background-color: #222426;
    border-radius: 4px;
}

a {
    color: #569CD6;
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}

    </style>
</head>
<body>
    <article id=\"content\">{{content | safe}}</article>
</body>
</html>
"
)]
struct Home {
    content: String,
}

#[get("/")]
async fn home(file: web::Data<Arc<Mutex<String>>>) -> actix_web::Result<HttpResponse> {
    let locked_file = file.lock().unwrap();
    let file_path = locked_file.clone();
    let markdown_input: String =
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
            .app_data(web::Data::new(Arc::clone(&file)))
    })
    .bind((
        ip_clone.lock().unwrap().clone(),
        port_clone.load(Ordering::SeqCst),
    ))?
    .run()
    .await
}

