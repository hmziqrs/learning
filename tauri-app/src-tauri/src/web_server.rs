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
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: Option<u16>,
    pub host: Option<String>,
    pub static_dir: Option<String>,
    pub cors: Option<bool>,
    pub directory_listing: Option<bool>,
    pub enable_logging: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
