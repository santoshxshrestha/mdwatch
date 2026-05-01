mod args;
mod utils;

use actix_files::NamedFile;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};
use ammonia::Builder;
use ammonia::UrlRelative::PassThrough;
use args::MdwatchArgs;
use askama::Template;
use clap::Parser;
use notify::event::RemoveKind;
use notify_debouncer_full::DebouncedEvent;
use notify_debouncer_full::{DebounceEventResult, new_debouncer, notify::*};
use pulldown_cmark::Options;
use regex::Regex;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::fs;

use notify::RecursiveMode;
use notify::event::{ModifyKind, RenameMode};
use rust_embed::Embed;
use tokio::sync::mpsc;

use utils::get_random_port;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/static"]
#[prefix = "static/"]
struct Static;

fn get_embedded_file(file_path: &str) -> String {
    match Static::get(file_path) {
        Some(file) => match std::str::from_utf8(&file.data) {
            Ok(content) => content.to_string(),
            Err(e) => {
                eprintln!("Failed to read embedded file: {e}");
                String::new()
            }
        },
        None => {
            eprintln!("File not found in embedded files.");
            String::new()
        }
    }
}

async fn ws_handler(
    req: HttpRequest,
    body: web::Payload,
    file_info: web::Data<FileInfo>,
) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut _msg_stream) = actix_ws::handle(&req, body)?;
    let file = file_info.file.to_path_buf();

    let file_name = match file.file_name() {
        Some(name) => name.to_os_string(),
        None => {
            return Err(actix_web::error::ErrorInternalServerError(
                "Failed to get file name",
            ));
        }
    };

    let base_dir = file_info.base_dir.to_path_buf();
    let (watch_tx, mut notify_rx) = mpsc::unbounded_channel::<DebouncedEvent>();

    let mut debouncer = new_debouncer(
        Duration::from_millis(200),
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
        .watch(&base_dir, RecursiveMode::Recursive)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    actix_web::rt::spawn(async move {
        // Keep the watcher alive in this async task to keep the msg_stream alive
        let _watcher = debouncer;

        // here we initially set last_sent to 1 second ago to allow the first update to be sent immediately
        let mut last_sent = Instant::now() - Duration::from_secs(1);

        while let Some(event) = notify_rx.recv().await {
            let is_selected_file = event.paths.iter().any(|p| {
                p.file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name == file_name)
                    .unwrap_or(false)
            });

            if is_selected_file {
                if matches!(event.kind, EventKind::Remove(RemoveKind::File)) {
                    eprintln!("File removed: {}", file.display());
                    break;
                }
                let modified_selected_file =
                    matches!(
                        event.kind,
                        EventKind::Modify(ModifyKind::Name(RenameMode::Both))
                    ) || matches!(event.kind, EventKind::Modify(ModifyKind::Data(_)));

                if modified_selected_file && last_sent.elapsed() >= Duration::from_secs(1) {
                    let latest_markdown = match get_markdown(&file).await {
                        Ok(md) => md,
                        Err(e) => {
                            eprintln!("Error reading markdown file: {e}");
                            continue;
                        }
                    };
                    last_sent = Instant::now();
                    if session.text(latest_markdown).await.is_err() {
                        break;
                    }
                }
            } 
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}

/// Rewrite local image `src` attributes to use the `/_local_image/` prefix.
/// Remote images (http://, https://, //, data:) are left untouched.
fn rewrite_image_paths(html: &str) -> String {
    let re = Regex::new(r#"(<img\s[^>]*?src\s*=\s*")([^"]*?)(")"#).expect("invalid regex");
    re.replace_all(html, |caps: &regex::Captures| {
        let prefix = &caps[1];
        let src = &caps[2];
        let suffix = &caps[3];
        // Skip remote URLs and data URIs
        if src.starts_with("http://")
            || src.starts_with("https://")
            || src.starts_with("//")
            || src.starts_with("data:")
        {
            format!("{}{}{}", prefix, src, suffix)
        } else {
            format!("{}/_local_image/{}{}", prefix, src, suffix)
        }
    })
    .to_string()
}

/// Sanitize HTML while preserving relative URLs (needed for /_local_image/ paths).
fn sanitize_html(html: &str) -> String {
    Builder::default()
        .url_relative(PassThrough)
        .add_generic_attributes(&["align"])
        .add_tag_attributes("code", &["class"])
        .clean(html)
        .to_string()
}

fn rewrite_mermaid_tags(html: &str) -> String {
    let re = Regex::new(r#"<pre><code class="language-mermaid">([\s\S]*?)</code></pre>"#)
        .expect("invalid regex");
    let src = r#"<pre class="mermaid">$1</pre>"#;
    re.replace_all(html, src).to_string()
}

async fn get_markdown(file_path: &PathBuf) -> std::io::Result<String> {
    let markdown_input: String = fs::read_to_string(file_path).await?;
    let options = Options::all();
    let parser = pulldown_cmark::Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output = rewrite_image_paths(&html_output);
    html_output = sanitize_html(&html_output);
    html_output = rewrite_mermaid_tags(&html_output);
    Ok(html_output)
}

#[derive(Template)]
#[template(path = "main.html")]
struct Mdwatch {
    content: String,
    title: String,
    style: String,
    script: String,
    lib: Libs,
}

struct Libs {
    hljs_theme_dark: String,
    hljs_theme_light: String,
    hljs_script: String,
    mermaid_script: String,
}

impl Default for Libs {
    fn default() -> Self {
        Self {
            hljs_theme_dark: get_embedded_file("static/lib/github-dark.min.css"),
            hljs_theme_light: get_embedded_file("static/lib/github-light.min.css"),
            hljs_script: get_embedded_file("static/lib/highlight.min.js"),
            mermaid_script: get_embedded_file("static/lib/mermaid.min.js"),
        }
    }
}

#[get("/")]
async fn home(file_info: web::Data<FileInfo>) -> actix_web::Result<HttpResponse> {
    let file = &file_info.file;
    let file_name = match file.file_name() {
        Some(name) => name,
        None => {
            return Err(actix_web::error::ErrorInternalServerError(
                "Failed to get file name",
            ));
        }
    };

    if let Some(extension) = file.extension()
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

    let html_output = match get_markdown(&file.to_path_buf()).await {
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
        style: get_embedded_file("static/global.css"),
        script: get_embedded_file("static/client.js"),
        lib: Libs::default(),
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

/// Serve local image files referenced in the markdown.
/// Resolves the requested path relative to the markdown file's parent directory.
#[get("/_local_image/{path:.*}")]
async fn serve_local_image(
    path: web::Path<PathBuf>,
    file_info: web::Data<FileInfo>,
) -> actix_web::Result<NamedFile> {
    let requested = path.into_inner();
    let base_dir = &file_info.base_dir;
    let resolved = base_dir.join(&requested);

    // Canonicalize to prevent directory traversal attacks (e.g. ../../etc/passwd)
    let canonical = resolved
        .canonicalize()
        .map_err(|_| actix_web::error::ErrorNotFound("Image not found"))?;

    let base_canonical = base_dir
        .canonicalize()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid base directory"))?;

    if !canonical.starts_with(&base_canonical) {
        return Err(actix_web::error::ErrorForbidden(
            "Access denied: path outside base directory",
        ));
    }

    Ok(NamedFile::open(canonical)?)
}

#[derive(Clone)]
struct FileInfo {
    file: PathBuf,
    base_dir: PathBuf,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = MdwatchArgs::parse();

    let file = args.file;
    let ip = args.ip;
    let port = args.port.unwrap_or_else(get_random_port);

    // Resolve the parent directory of the markdown file for serving local images
    let base_dir: PathBuf = file
        .parent()
        .map(|p| {
            if p.as_os_str().is_empty() {
                PathBuf::from(".")
            } else {
                p.to_path_buf()
            }
        })
        .unwrap_or_else(|| PathBuf::from("."));

    let file_info = FileInfo {
        file,
        base_dir,
    };

    if ip == "0.0.0.0" {
        eprintln!("  Warning: Binding to 0.0.0.0 exposes your server to the entire network!");
        eprintln!("         Make sure you trust your network or firewall settings.");
    }

    println!("Server running at:");
    println!(" - http://{}:{}/", ip, port);

    match HttpServer::new(move || {
        App::new()
            .route("/ws", web::get().to(ws_handler))
            .service(home)
            .service(serve_local_image)
            .app_data(web::Data::new(file_info.clone()))
    })
    .bind(format!("{}:{}", ip, port))
    {
        Ok(server) => {
            if let Err(e) = webbrowser::open(&format!("http://localhost:{}/", port)) {
                eprintln!("Failed to open browser: {e}");
            }
            server.run().await
        }
        Err(e) => {
            eprintln!("Failed to start server: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: &'static str,
        expected: &'static str,
    }

    #[test]
    fn test_rewrite_image_paths() {
        let test_cases = [
            TestCase {
                input: r#"<img src="image.png" alt="Image">"#,
                expected: r#"<img src="/_local_image/image.png" alt="Image">"#,
            },
            TestCase {
                input: r#"<img src="http://example.com/image.png" alt="Remote Image">"#,
                expected: r#"<img src="http://example.com/image.png" alt="Remote Image">"#,
            },
            TestCase {
                input: r#"<img src="data:image/png;base64,..." alt="Data URI">"#,
                expected: r#"<img src="data:image/png;base64,..." alt="Data URI">"#,
            },
            TestCase {
                input: r#"<img src="//example.com/image.png" alt="Protocol-relative URL">"#,
                expected: r#"<img src="//example.com/image.png" alt="Protocol-relative URL">"#,
            },
        ];

        for case in test_cases {
            let result = rewrite_image_paths(case.input);
            assert_eq!(
                result, case.expected,
                "Failed to rewrite image paths for input: {}",
                case.input
            );
        }
    }
}
