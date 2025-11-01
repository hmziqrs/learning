mod cache;
mod config;
mod entity;
mod handlers;
mod health;
mod telemetry;

use axum::{
    extract::DefaultBodyLimit,
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use config::Config;
use sea_orm::{Database, DatabaseConnection};
use std::net::SocketAddr;
use telemetry::init_telemetry;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

use handlers::{AppState, todo_routes};
use health::health_routes;
use migration::Migrator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    
    let _prometheus_exporter = init_telemetry(&config.quickwit_otlp_endpoint);

    let db: DatabaseConnection = Database::connect(&config.database_url).await?;
    
    let cache = cache::RedisCache::new(&config.redis_url)?;

    Migrator::up(&db, None).await?;

    let app_state = AppState {
        db,
        cache,
        config: config.clone(),
    };

    let app = Router::new()
        .merge(todo_routes())
        .merge(health_routes())
        .route("/metrics", get(metrics_handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(HeaderValue::from_static("http://localhost:3000"))
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers(Any)
                        .allow_origin(Any),
                )
                .layer(DefaultBodyLimit::max(1024 * 1024 * 10)), // 10MB limit
        )
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.service_port));
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn metrics_handler() -> Result<String, axum::http::StatusCode> {
    use prometheus::Encoder;
    
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    
    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        tracing::error!("Failed to encode metrics: {:?}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    match String::from_utf8(buffer) {
        Ok(metrics) => Ok(metrics),
        Err(e) => {
            tracing::error!("Failed to convert metrics to string: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
