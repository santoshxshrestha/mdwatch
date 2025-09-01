mod template;
use actix_web::web;
use pulldown_cmark;
use pulldown_cmark::Options;
use std::fs;
use std::path::Path;
use std::sync::atomic::AtomicU64;
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
    let locked_file = match file.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let file_path = locked_file.clone();
    let file = match Path::new(&file_path).file_name() {
        Some(name) => name,
        None => {
            return Err(actix_web::error::ErrorInternalServerError(
                "Failed to get file name",
            ));
        }
    };

    let markdown_input: String = fs::read_to_string(file_path.clone())
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let options = Options::all();
    let parser = pulldown_cmark::Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output = clean(&html_output);

    let template = Home {
        content: html_output,
        last_modified: last_modified.load(Ordering::SeqCst),
        title: file.to_string_lossy().to_string(),
    };

    match template.render() {
        Ok(rendered) => Ok(HttpResponse::Ok().content_type("text/html").body(rendered)),
        Err(e) => {
            eprintln!("Template rendering error: {}", e);

            Ok(HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body("Failed to render template"))
        }
    }
}

#[get("/api/check-update")]
async fn check_update(file: web::Data<Arc<Mutex<String>>>) -> actix_web::Result<HttpResponse> {
    let locked_file = match file.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let file_path = locked_file.clone();

    match fs::metadata(&file_path) {
        Ok(metadata) => match metadata.modified() {
            Ok(modified_time) => {
                let timestamp = modified_time
                    .duration_since(UNIX_EPOCH)
                    .map_err(|_| {
                        eprintln!("Warning: System time is before UNIX epoch");
                        actix_web::error::ErrorInternalServerError("Invalid system time")
                    })?
                    .as_secs();

                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "last_modified": timestamp
                })))
            }
            Err(e) => {
                eprintln!("Error: Failed to get modification time: {}", e);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to read file modification time"
                })))
            }
        },
        Err(e) => {
            eprintln!("Warning: File not found or inaccessible: {}", e);
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "last_modified": 0
            })))
        }
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
            match file.lock() {
                Ok(mut guard) => *guard = f,
                Err(poisoned) => *poisoned.into_inner() = f,
            }
            match ip.lock() {
                Ok(mut guard) => *guard = i,
                Err(poisoned) => *poisoned.into_inner() = i,
            }
            port.store(p, Ordering::SeqCst);
        }
    }

    let ip_clone = Arc::clone(&ip);
    let port_clone = Arc::clone(&port);
    let last_modified_clone = Arc::clone(&last_modified);

    match ip.lock() {
        Ok(guard) => {
            if *guard == "0.0.0.0" {
                eprintln!(
                    "⚠️ Warning: Binding to 0.0.0.0 exposes your server to the entire network!"
                );
                eprintln!("         Make sure you trust your network or firewall settings.");
            }
        }
        Err(e) => {
            eprintln!("Failed to acquire IP lock: {}", e);
        }
    }

    println!("Server running at:");
    println!(
        " - localhost: http://{}:{}/",
        match ip.lock() {
            Ok(guard) => guard.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        },
        port.load(Ordering::SeqCst)
    );

    if let Err(e) =
        webbrowser::open(format!("http://localhost:{}/", port.load(Ordering::SeqCst)).as_str())
    {
        eprintln!("Failed to open browser: {}", e);
    }

    HttpServer::new(move || {
        App::new()
            .service(home)
            .service(check_update)
            .app_data(web::Data::new(last_modified_clone.clone()))
            .app_data(web::Data::new(Arc::clone(&file)))
    })
    .bind((
        ip_clone
            .lock()
            .map(|guard| guard.clone())
            .unwrap_or_else(|poisoned| {
                eprintln!("Failed to acquire IP lock {poisoned}, defaulting to 127.0.0.1");
                "127.0.0.1".to_string()
            }),
        port_clone.load(Ordering::SeqCst),
    ))?
    .run()
    .await
}
