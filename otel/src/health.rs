use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get},
    Router,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::cache::RedisCache;
use crate::handlers::AppState;
use crate::config::Config;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Debug, Serialize)]
pub struct ServiceHealthResponse {
    service: String,
    status: String,
    message: Option<String>,
}

#[instrument(skip(state))]
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "0.1.0".to_string(),
    })
}

#[instrument(skip(state))]
pub async fn quickwit_health_check(
    State(state): State<AppState>,
) -> Result<Json<ServiceHealthResponse>, StatusCode> {
    let span = tracing::info_span!("quickwit_health_check");
    let _enter = span.enter();

    let client = reqwest::Client::new();
    let url = format!("{}/api/v1/catalog", state.config.quickwit_http_endpoint);

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                info!("QuickWit health check successful");
                Ok(Json(ServiceHealthResponse {
                    service: "quickwit".to_string(),
                    status: "healthy".to_string(),
                    message: None,
                }))
            } else {
                let status = response.status();
                error!("QuickWit health check failed with status: {}", status);
                Ok(Json(ServiceHealthResponse {
                    service: "quickwit".to_string(),
                    status: "unhealthy".to_string(),
                    message: Some(format!("HTTP status: {}", status)),
                }))
            }
        }
        Err(e) => {
            error!("QuickWit health check error: {:?}", e);
            Ok(Json(ServiceHealthResponse {
                service: "quickwit".to_string(),
                status: "unhealthy".to_string(),
                message: Some(e.to_string()),
            }))
        }
    }
}

#[instrument(skip(state))]
pub async fn prometheus_health_check(
    State(state): State<AppState>,
) -> Result<Json<ServiceHealthResponse>, StatusCode> {
    let span = tracing::info_span!("prometheus_health_check");
    let _enter = span.enter();

    let client = reqwest::Client::new();
    let url = format!("{}/-/healthy", state.config.prometheus_endpoint);

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                info!("Prometheus health check successful");
                Ok(Json(ServiceHealthResponse {
                    service: "prometheus".to_string(),
                    status: "healthy".to_string(),
                    message: None,
                }))
            } else {
                let status = response.status();
                error!("Prometheus health check failed with status: {}", status);
                Ok(Json(ServiceHealthResponse {
                    service: "prometheus".to_string(),
                    status: "unhealthy".to_string(),
                    message: Some(format!("HTTP status: {}", status)),
                }))
            }
        }
        Err(e) => {
            error!("Prometheus health check error: {:?}", e);
            Ok(Json(ServiceHealthResponse {
                service: "prometheus".to_string(),
                status: "unhealthy".to_string(),
                message: Some(e.to_string()),
            }))
        }
    }
}

#[instrument(skip(state))]
pub async fn delete_cache_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ServiceHealthResponse>, StatusCode> {
    let span = tracing::info_span!("delete_cache_entry", id = %id);
    let _enter = span.enter();

    match state.cache.delete_todo(&id) {
        Ok(true) => {
            info!("Successfully deleted cache entry for todo ID: {}", id);
            Ok(Json(ServiceHealthResponse {
                service: "redis_cache".to_string(),
                status: "deleted".to_string(),
                message: Some(format!("Cache entry for todo {} deleted", id)),
            }))
        }
        Ok(false) => {
            info!("No cache entry found for todo ID: {}", id);
            Ok(Json(ServiceHealthResponse {
                service: "redis_cache".to_string(),
                status: "not_found".to_string(),
                message: Some(format!("No cache entry found for todo {}", id)),
            }))
        }
        Err(e) => {
            error!("Failed to delete cache entry: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QuickwitSearchQuery {
    q: String,
    index: Option<String>,
}

#[instrument(skip(state))]
pub async fn quickwit_search(
    State(state): State<AppState>,
    Json(query): Json<QuickwitSearchQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let span = tracing::info_span!("quickwit_search", query = %query.q);
    let _enter = span.enter();

    let client = reqwest::Client::new();
    let index = query.index.as_deref().unwrap_or("otel-log-v0");
    let url = format!("{}/api/v1/{}/search", state.config.quickwit_http_endpoint, index);

    let search_body = serde_json::json!({
        "query": query.q,
        "max_hits": 10
    });

    match client.post(&url).json(&search_body).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json().await {
                    Ok(result) => {
                        info!("QuickWit search successful");
                        Ok(Json(result))
                    }
                    Err(e) => {
                        error!("Failed to parse QuickWit search response: {:?}", e);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            } else {
                let status = response.status();
                error!("QuickWit search failed with status: {}", status);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(e) => {
            error!("QuickWit search error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/health/quickwit", get(quickwit_health_check))
        .route("/health/prometheus", get(prometheus_health_check))
        .route("/cache/todo/:id", delete(delete_cache_entry))
        .route("/quickwit/search", get(quickwit_search))
}