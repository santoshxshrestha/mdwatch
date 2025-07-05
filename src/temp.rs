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
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::Ordering;
use std::sync::atomic::{AtomicU16, AtomicU64};
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
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
:root {
    --bg-color: #181A1B;
    --text-color: #D3CFC9;
    --heading-color: #E0E0E0;
    --code-bg: #2D2F30;
    --code-color: #F8F8F2;
    --quote-bg: #222426;
    --quote-border: #555;
    --quote-text: #AAA;
    --link-color: #569CD6;
    --button-bg: #2D2F30;
    --button-color: #F8F8F2;
    --button-hover: #3D3F40;
}

[data-theme=\"light\"] {
    --bg-color: #ffffff;
    --text-color: #333333;
    --heading-color: #222222;
    --code-bg: #f5f5f5;
    --code-color: #333333;
    --quote-bg: #f9f9f9;
    --quote-border: #ccc;
    --quote-text: #666;
    --link-color: #0066cc;
    --button-bg: #f0f0f0;
    --button-color: #333333;
    --button-hover: #e0e0e0;
}

body {
    max-width: 720px;
    margin: auto;
    padding: 2rem;
    font-family: system-ui, sans-serif;
    line-height: 1.6;
    background-color: var(--bg-color);
    color: var(--text-color);
    transition: background-color 0.3s ease, color 0.3s ease;
}

h1, h2, h3 {
    color: var(--heading-color);
}

code {
    background: var(--code-bg);
    padding: 0.2em 0.4em;
    border-radius: 4px;
    font-family: monospace;
    color: var(--code-color);
}

pre {
    background: var(--code-bg);
    padding: 1em;
    overflow-x: auto;
    border-radius: 4px;
    color: var(--code-color);
}

blockquote {
    border-left: 4px solid var(--quote-border);
    padding-left: 1em;
    color: var(--quote-text);
    background-color: var(--quote-bg);
    border-radius: 4px;
}

a {
    color: var(--link-color);
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}

.theme-toggle {
    position: fixed;
    top: 20px;
    right: 20px;
    background: var(--button-bg);
    color: var(--button-color);
    border: none;
    border-radius: 50%;
    width: 50px;
    height: 50px;
    font-size: 20px;
    cursor: pointer;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
    transition: all 0.3s ease;
}

.theme-toggle:hover {
    background: var(--button-hover);
    transform: scale(1.1);
}

.theme-toggle:active {
    transform: scale(0.95);
}

.reload-indicator {
    position: fixed;
    top: 80px;
    right: 20px;
    background: var(--button-bg);
    color: var(--button-color);
    border: none;
    border-radius: 20px;
    padding: 8px 12px;
    font-size: 12px;
    opacity: 0;
    transition: opacity 0.3s ease;
    pointer-events: none;
}

.reload-indicator.visible {
    opacity: 0.9;
}
    </style>
</head>
<body>
    <button class=\"theme-toggle\" onclick=\"toggleTheme()\" title=\"Toggle theme\">
        🌙
    </button>
    <div class=\"reload-indicator\" id=\"reloadIndicator\">
        🔄 Reloading...
    </div>
    <article id=\"content\">{{content | safe}}</article>
    
    <script>
        let lastModified = {{last_modified}};
        
        function toggleTheme() {
            const html = document.documentElement;
            const button = document.querySelector('.theme-toggle');
            
            if (html.getAttribute('data-theme') === 'light') {
                html.removeAttribute('data-theme');
                button.textContent = '🌙';
                localStorage.setItem('theme', 'dark');
            } else {
                html.setAttribute('data-theme', 'light');
                button.textContent = '☀️';
                localStorage.setItem('theme', 'light');
            }
        }
        
        // Load saved theme on page load
        document.addEventListener('DOMContentLoaded', function() {
            const savedTheme = localStorage.getItem('theme');
            const html = document.documentElement;
            const button = document.querySelector('.theme-toggle');
            
            if (savedTheme === 'light') {
                html.setAttribute('data-theme', 'light');
                button.textContent = '☀️';
            } else {
                button.textContent = '🌙';
            }
            
            // Start checking for file changes
            checkForUpdates();
        });
        
        function checkForUpdates() {
            fetch('/api/check-update')
                .then(response => response.json())
                .then(data => {
                    if (data.last_modified > lastModified) {
                        lastModified = data.last_modified;
                        
                        // Show reload indicator
                        const indicator = document.getElementById('reloadIndicator');
                        indicator.classList.add('visible');
                        
                        // Store current scroll position
                        const scrollPercentage = window.pageYOffset / (document.body.scrollHeight - window.innerHeight);
                        localStorage.setItem('scrollPosition', scrollPercentage);
                        
                        // Reload after a short delay
                        setTimeout(() => {
                            window.location.reload();
                        }, 200);
                    }
                })
                .catch(error => {
                    // Silently handle errors - server might be restarting
                    console.log('Connection lost, retrying...');
                });
            
            // Check every 500ms
            setTimeout(checkForUpdates, 500);
        }
        
        // Restore scroll position after reload
        window.addEventListener('load', function() {
            const savedScrollPosition = localStorage.getItem('scrollPosition');
            if (savedScrollPosition) {
                setTimeout(() => {
                    const scrollY = parseFloat(savedScrollPosition) * (document.body.scrollHeight - window.innerHeight);
                    window.scrollTo(0, scrollY);
                    localStorage.removeItem('scrollPosition');
                }, 100);
            }
        });
    </script>
</body>
</html>
"
)]
struct Home {
    content: String,
    last_modified: u64,
}

#[get("/")]
async fn home(
    file: web::Data<Arc<Mutex<String>>>,
    last_modified: web::Data<Arc<AtomicU64>>,
) -> actix_web::Result<HttpResponse> {
    let locked_file = file.lock().unwrap();
    let file_path = locked_file.clone();
    let markdown_input: String =
        fs::read_to_string(&file_path).map_err(actix_web::error::ErrorInternalServerError)?;
    let parser = pulldown_cmark::Parser::new(&markdown_input);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output = clean(&html_output);

    let template = Home {
        content: html_output,
        last_modified: last_modified.load(Ordering::SeqCst),
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

#[get("/api/check-update")]
async fn check_update(
    file: web::Data<Arc<Mutex<String>>>,
    last_modified: web::Data<Arc<AtomicU64>>,
) -> actix_web::Result<HttpResponse> {
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

fn start_file_watcher(file_path: String, last_modified: Arc<AtomicU64>) {
    thread::spawn(move || {
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            },
            notify::Config::default(),
        )
        .unwrap();

        if let Some(parent) = Path::new(&file_path).parent() {
            watcher.watch(parent, RecursiveMode::NonRecursive).unwrap();
        }

        loop {
            match rx.recv() {
                Ok(event) => {
                    if let EventKind::Modify(_) = event.kind {
                        for path in event.paths {
                            if path.to_string_lossy() == file_path {
                                if let Ok(metadata) = fs::metadata(&file_path) {
                                    let modified_time = metadata
                                        .modified()
                                        .unwrap_or(SystemTime::UNIX_EPOCH)
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs();

                                    last_modified.store(modified_time, Ordering::SeqCst);
                                    println!("📝 File changed: {}", file_path);
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("File watcher error: {:?}", e),
            }
        }
    });
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
            *file.lock().unwrap() = f.clone();
            *ip.lock().unwrap() = i;
            port.store(p, Ordering::SeqCst);

            // Get initial modification time
            if let Ok(metadata) = fs::metadata(&f) {
                let modified_time = metadata
                    .modified()
                    .unwrap_or(SystemTime::UNIX_EPOCH)
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                last_modified.store(modified_time, Ordering::SeqCst);
            }

            // Start file watcher
            start_file_watcher(f, Arc::clone(&last_modified));
        }
    }

    let ip_clone = Arc::clone(&ip);
    let port_clone = Arc::clone(&port);

    if ip.lock().unwrap().as_str() == "0.0.0.0" {
        eprintln!("⚠️ Warning: Binding to 0.0.0.0 exposes your server to the entire network!");
        eprintln!("         Make sure you trust your network or firewall settings.");
    }

    println!("🚀 mdserve starting...");
    println!(
        "📍 Server running at: http://{}:{}/",
        ip.lock().unwrap(),
        port.load(Ordering::SeqCst)
    );
    println!("📁 Watching: {}", file.lock().unwrap());
    println!("🔄 Auto-reload: enabled");

    let _ = webbrowser::open(format!("http://localhost:{}/", port.load(Ordering::SeqCst)).as_str());

    HttpServer::new(move || {
        App::new()
            .service(home)
            .service(check_update)
            .app_data(web::Data::new(Arc::clone(&file)))
            .app_data(web::Data::new(Arc::clone(&last_modified)))
    })
    .bind((
        ip_clone.lock().unwrap().clone(),
        port_clone.load(Ordering::SeqCst),
    ))?
    .run()
    .await
}
