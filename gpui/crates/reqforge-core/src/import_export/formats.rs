//! Support for external formats (Postman, OpenAPI, etc.)

pub mod postman;
pub mod openapi;
pub mod manifest;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::{HttpMethod, KeyValuePair, BodyType, RawContentType};
use super::error::{ImportError, ImportErrorKind};
use super::ImportResult;

/// Workspace manifest for full exports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceManifest {
    pub version: String,
    pub exported_at: chrono::DateTime<chrono::Utc>,
    pub collection_count: usize,
    pub environment_count: usize,
}

/// Generic helper to parse HTTP method strings
fn parse_http_method(method: &str) -> ImportResult<HttpMethod> {
    match method.to_uppercase().as_str() {
        "GET" => Ok(HttpMethod::GET),
        "POST" => Ok(HttpMethod::POST),
        "PUT" => Ok(HttpMethod::PUT),
        "PATCH" => Ok(HttpMethod::PATCH),
        "DELETE" => Ok(HttpMethod::DELETE),
        "HEAD" => Ok(HttpMethod::HEAD),
        "OPTIONS" => Ok(HttpMethod::OPTIONS),
        _ => Err(ImportError::new(
            ImportErrorKind::Validation,
            &format!("Unknown HTTP method: {}", method),
        )),
    }
}

/// Helper to extract headers from various formats
fn extract_headers(headers_value: &Value) -> Vec<KeyValuePair> {
    headers_value
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| {
                    if let Some(obj) = v.as_object() {
                        let key = obj.get("key").and_then(|k| k.as_str()).unwrap_or("");
                        let value = obj.get("value").and_then(|v| v.as_str()).unwrap_or("");
                        let enabled = obj.get("disabled").and_then(|d| d.as_bool()).map(|d| !d).unwrap_or(true);
                        let description = obj.get("description").and_then(|d| d.as_str()).map(String::from);

                        if !key.is_empty() {
                            Some(KeyValuePair {
                                key: key.to_string(),
                                value: value.to_string(),
                                enabled,
                                description,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Helper to extract query parameters from various formats
fn extract_query_params(query_params_value: &Value) -> Vec<KeyValuePair> {
    query_params_value
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| {
                    if let Some(obj) = v.as_object() {
                        let key = obj.get("key").and_then(|k| k.as_str()).unwrap_or("");
                        let value = obj.get("value").and_then(|v| v.as_str()).unwrap_or("");
                        let enabled = obj.get("disabled").and_then(|d| d.as_bool()).map(|d| !d).unwrap_or(true);
                        let description = obj.get("description").and_then(|d| d.as_str()).map(String::from);

                        if !key.is_empty() {
                            Some(KeyValuePair {
                                key: key.to_string(),
                                value: value.to_string(),
                                enabled,
                                description,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Helper to extract body from various formats
fn extract_body(body_value: &Value, body_mode: Option<&str>) -> ImportResult<BodyType> {
    match body_mode {
        Some("raw") => {
            let content = body_value
                .get("raw")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let content_type = match body_value
                .get("options")
                .and_then(|o| o.get("raw"))
                .and_then(|r| r.get("language"))
                .and_then(|l| l.as_str())
            {
                Some("json") => RawContentType::Json,
                Some("xml") => RawContentType::Xml,
                Some("html") => RawContentType::Html,
                _ => RawContentType::Text,
            };

            Ok(BodyType::Raw { content, content_type })
        }
        Some("urlencoded") => {
            let fields = body_value
                .get("urlencoded")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| {
                            if let Some(obj) = v.as_object() {
                                let key = obj.get("key").and_then(|k| k.as_str()).unwrap_or("");
                                let value = obj.get("value").and_then(|v| v.as_str()).unwrap_or("");
                                let enabled = obj.get("disabled").and_then(|d| d.as_bool()).map(|d| !d).unwrap_or(true);
                                let description = obj.get("description").and_then(|d| d.as_str()).map(String::from);

                                if !key.is_empty() {
                                    Some(KeyValuePair {
                                        key: key.to_string(),
                                        value: value.to_string(),
                                        enabled,
                                        description,
                                    })
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();

            Ok(BodyType::FormUrlEncoded(fields))
        }
        Some("formdata") => {
            // For now, treat formdata as urlencoded (simplified)
            let fields = body_value
                .get("formdata")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| {
                            if let Some(obj) = v.as_object() {
                                let key = obj.get("key").and_then(|k| k.as_str()).unwrap_or("");
                                let value = obj.get("value").and_then(|v| v.as_str()).unwrap_or("");
                                let enabled = obj.get("disabled").and_then(|d| d.as_bool()).map(|d| !d).unwrap_or(true);
                                let description = obj.get("description").and_then(|d| d.as_str()).map(String::from);

                                if !key.is_empty() && obj.get("type").and_then(|t| t.as_str()) != Some("file") {
                                    Some(KeyValuePair {
                                        key: key.to_string(),
                                        value: value.to_string(),
                                        enabled,
                                        description,
                                    })
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();

            Ok(BodyType::FormUrlEncoded(fields))
        }
        _ => Ok(BodyType::None),
    }
}
