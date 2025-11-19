use axum::{
    Router,
    routing::get_service,
    http::{StatusCode, Uri, Request},
    response::{IntoResponse, Response},
    body::Body,
};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tower_http::LatencyUnit;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;
use std::time::Duration;
use std::path::Path;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub port: Option<u16>,
    pub host: Option<String>,
    pub static_dir: Option<String>,
    pub cors: Option<bool>,
    pub directory_listing: Option<bool>,
    pub enable_logging: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub id: String,
    pub url: String,
    pub port: u16,
    pub running: bool,
    pub static_dir: Option<String>,
    pub directory_listing: bool,
    pub logging_enabled: bool,
}

struct RunningServer {
    info: ServerInfo,
    shutdown_tx: tokio::sync::oneshot::Sender<()>,
}

pub struct ServerManager {
    servers: Arc<Mutex<HashMap<String, RunningServer>>>,
}

impl ServerManager {
    pub fn new() -> Self {
        Self {
            servers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn start_server(&self, config: ServerConfig) -> Result<ServerInfo, String> {
        let port = config.port.unwrap_or(0);
        let host = config.host.unwrap_or_else(|| "127.0.0.1".to_string());
        let enable_cors = config.cors.unwrap_or(true);
        let enable_directory_listing = config.directory_listing.unwrap_or(false);
        let enable_logging = config.enable_logging.unwrap_or(false);
        let static_dir = config.static_dir.clone();

        // Validate static directory if provided
        if let Some(ref dir) = static_dir {
            let path = Path::new(dir);

            if !path.exists() {
                return Err(format!("Directory '{}' does not exist. Please create it first or use a different path.", dir));
            }

            if !path.is_dir() {
                return Err(format!("Path '{}' is not a directory. Please specify a valid directory path.", dir));
            }

            // Check if directory is readable
            if std::fs::read_dir(path).is_err() {
                return Err(format!("Directory '{}' is not accessible. Please check permissions.", dir));
            }
        }

        // Create the address
        let addr_str = format!("{}:{}", host, port);
        let addr: SocketAddr = addr_str.parse()
            .map_err(|e| format!("Invalid address {}: {}", addr_str, e))?;

        // Try to bind to get the actual port
        let listener = TcpListener::bind(addr)
            .map_err(|e| format!("Failed to bind to {}: {}", addr_str, e))?;

        let actual_addr = listener.local_addr()
            .map_err(|e| format!("Failed to get local address: {}", e))?;

        let actual_port = actual_addr.port();

        // Convert to tokio listener
        listener.set_nonblocking(true)
            .map_err(|e| format!("Failed to set non-blocking: {}", e))?;

        let tokio_listener = tokio::net::TcpListener::from_std(listener)
            .map_err(|e| format!("Failed to convert listener: {}", e))?;

        // Create the router
        let mut app = Router::new();

        // Add static file serving if directory is provided
        if let Some(ref dir) = static_dir {
            let mut serve_dir = ServeDir::new(dir);

            // Enable directory listing if requested
            if enable_directory_listing {
                serve_dir = serve_dir.precompressed_br()
                    .precompressed_gzip()
                    .precompressed_deflate();
            }

            app = app.fallback_service(get_service(serve_dir));
        } else {
            // Fallback to a simple 404 handler
            app = app.fallback(|| async {
                (StatusCode::NOT_FOUND, "Static directory not configured")
            });
        }

        // Add request logging if enabled
        if enable_logging {
            app = app.layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new()
                        .level(tracing::Level::INFO)
                        .include_headers(true))
                    .on_response(DefaultOnResponse::new()
                        .level(tracing::Level::INFO)
                        .latency_unit(LatencyUnit::Millis))
            );
        }

        // Add CORS if enabled
        if enable_cors {
            app = app.layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any)
            );
        }

        // Create server info
        let server_id = Uuid::new_v4().to_string();
        let server_info = ServerInfo {
            id: server_id.clone(),
            url: format!("http://{}:{}", host, actual_port),
            port: actual_port,
            running: true,
            static_dir: static_dir.clone(),
            directory_listing: enable_directory_listing,
            logging_enabled: enable_logging,
        };

        // Create shutdown channel
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

        // Spawn the server
        tokio::spawn(async move {
            let server = axum::serve(tokio_listener, app)
                .with_graceful_shutdown(async {
                    shutdown_rx.await.ok();
                });

            if let Err(e) = server.await {
                eprintln!("Server error: {}", e);
            }
        });

        // Store the server
        let running_server = RunningServer {
            info: server_info.clone(),
            shutdown_tx,
        };

        self.servers.lock().await.insert(server_id, running_server);

        Ok(server_info)
    }

    pub async fn stop_server(&self, server_id: &str) -> Result<(), String> {
        let mut servers = self.servers.lock().await;

        if let Some(server) = servers.remove(server_id) {
            // Send shutdown signal
            let _ = server.shutdown_tx.send(());
            Ok(())
        } else {
            Err(format!("Server with id {} not found", server_id))
        }
    }

    pub async fn get_server_info(&self, server_id: &str) -> Result<ServerInfo, String> {
        let servers = self.servers.lock().await;

        servers.get(server_id)
            .map(|s| s.info.clone())
            .ok_or_else(|| format!("Server with id {} not found", server_id))
    }

    pub async fn list_servers(&self) -> Vec<ServerInfo> {
        let servers = self.servers.lock().await;
        servers.values().map(|s| s.info.clone()).collect()
    }

    pub async fn stop_all_servers(&self) -> Result<(), String> {
        let mut servers = self.servers.lock().await;

        for (_, server) in servers.drain() {
            let _ = server.shutdown_tx.send(());
        }

        Ok(())
    }
}

pub fn is_port_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

pub fn create_test_directory(path: &str) -> Result<String, String> {
    use std::fs;

    let dir_path = Path::new(path);

    // Check if directory already exists
    if dir_path.exists() {
        return Err(format!("Directory '{}' already exists. Please choose a different name or delete the existing directory.", path));
    }

    // Create the directory
    fs::create_dir_all(dir_path)
        .map_err(|e| format!("Failed to create directory '{}': {}", path, e))?;

    // Create sample index.html
    let index_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Local Web Server Test Page</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 40px 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }
        .container {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border-radius: 20px;
            padding: 40px;
            box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
        }
        h1 {
            margin-top: 0;
            font-size: 2.5em;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.2);
        }
        p {
            font-size: 1.2em;
            line-height: 1.6;
        }
        .success {
            background: rgba(16, 185, 129, 0.2);
            border-left: 4px solid #10b981;
            padding: 15px;
            border-radius: 8px;
            margin: 20px 0;
        }
        .feature-list {
            list-style: none;
            padding: 0;
        }
        .feature-list li {
            padding: 10px 0;
            padding-left: 30px;
            position: relative;
        }
        .feature-list li:before {
            content: "✓";
            position: absolute;
            left: 0;
            color: #10b981;
            font-weight: bold;
            font-size: 1.2em;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎉 Success!</h1>
        <div class="success">
            <strong>Your local web server is running!</strong>
        </div>
        <p>
            This is a test page created by the Tauri Local Web Server module.
            You can now serve static files from this directory.
        </p>
        <h2>What you can do:</h2>
        <ul class="feature-list">
            <li>Add HTML, CSS, and JavaScript files to this directory</li>
            <li>Test your web applications locally</li>
            <li>Serve images, videos, and other static assets</li>
            <li>Enable directory listing to browse files</li>
            <li>Use CORS for development</li>
            <li>Monitor requests with logging</li>
        </ul>
        <p style="margin-top: 30px; opacity: 0.8; font-size: 0.9em;">
            📁 Directory: <code>""#.to_string() + path + r#"</code><br>
            🚀 Powered by Tauri & Axum
        </p>
    </div>
</body>
</html>"#;

    let index_path = dir_path.join("index.html");
    fs::write(&index_path, index_html)
        .map_err(|e| format!("Failed to create index.html: {}", e))?;

    // Create a simple CSS file
    let style_css = r#"/* Add your custom styles here */
body {
    margin: 0;
    padding: 0;
}
"#;

    let css_path = dir_path.join("style.css");
    fs::write(&css_path, style_css)
        .map_err(|e| format!("Failed to create style.css: {}", e))?;

    Ok(format!("Test directory created successfully at '{}'", path))
}
