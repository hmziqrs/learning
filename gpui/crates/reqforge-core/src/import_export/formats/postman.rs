//! Postman collection format support

use serde_json::Value;
use uuid::Uuid;
use chrono::Utc;

use crate::models::{Collection, RequestDefinition, HttpMethod, BodyType, CollectionItem, Folder};
use super::super::error::{ImportError, ImportErrorKind};
use super::super::ImportResult;
use super::{parse_http_method, extract_headers, extract_query_params, extract_body};

/// Parse a Postman collection v2.1 into a ReqForge collection
pub fn parse_postman_collection(postman: &Value) -> ImportResult<Collection> {
    // Get collection info
    let info = postman
        .get("info")
        .ok_or_else(|| ImportError::new(
            ImportErrorKind::PostmanFormat,
            "Missing 'info' field in Postman collection"
        ))?;

    let name = info
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ImportError::new(
            ImportErrorKind::PostmanFormat,
            "Missing 'name' field in Postman collection info"
        ))?;

    let mut collection = Collection::new(name);

    // Get items (folders and requests)
    let items = postman
        .get("item")
        .and_then(|v| v.as_array())
        .ok_or_else(|| ImportError::new(
            ImportErrorKind::PostmanFormat,
            "Missing 'item' field in Postman collection"
        ))?;

    // Process items recursively
    let mut requests = Vec::new();
    for item in items {
        process_postman_item(item, &mut requests, None)?;
    }

    // Add all requests to the collection
    for (req, parent_folder_id) in requests {
        collection.add_request(req, parent_folder_id);
    }

    Ok(collection)
}

/// Process a Postman item (folder or request) recursively
fn process_postman_item(
    item: &Value,
    requests: &mut Vec<(RequestDefinition, Option<Uuid>)>,
    parent_folder_id: Option<Uuid>,
) -> ImportResult<Option<Uuid>> {
    let name = item
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unnamed");

    // Check if this is a folder or request
    if item.get("request").is_some() {
        // This is a request
        let request = parse_postman_request(item)?;
        requests.push((request, parent_folder_id));
        Ok(None)
    } else if item.get("item").is_some() {
        // This is a folder
        let folder_id = Some(Uuid::new_v4());

        // Process children
        let children = item
            .get("item")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ImportError::new(
                ImportErrorKind::PostmanFormat,
                "Invalid 'item' field in Postman folder"
            ))?;

        for child in children {
            process_postman_item(child, requests, folder_id)?;
        }

        Ok(folder_id)
    } else {
        // Unknown item type, skip
        Ok(None)
    }
}

/// Parse a Postman request
fn parse_postman_request(item: &Value) -> ImportResult<RequestDefinition> {
    let name = item
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unnamed Request")
        .to_string();

    let request_value = item
        .get("request")
        .ok_or_else(|| ImportError::new(
            ImportErrorKind::PostmanFormat,
            "Missing 'request' field in Postman item"
        ))?;

    // Handle case where request is a string (reference to another request)
    let request_obj = if request_value.is_string() {
        // For now, create a minimal request since we can't resolve references
        return Ok(RequestDefinition::new(name, HttpMethod::GET, ""));
    } else {
        request_value
    };

    // Get method
    let method_str = request_value
        .get("method")
        .and_then(|v| v.as_str())
        .unwrap_or("GET");

    let method = parse_http_method(method_str)?;

    // Get URL
    let url = parse_postman_url(request_value.get("url"))?;

    // Get headers
    let headers = request_value
        .get("header")
        .map_or_else(Vec::new, extract_headers);

    // Get query parameters (from URL if present, otherwise from request)
    let query_params = if let Some(url_obj) = request_value.get("url") {
        if let Some(query) = url_obj.get("query") {
            extract_query_params(query)
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    // Get body
    let body = if let Some(body_obj) = request_value.get("body") {
        let body_mode = body_obj.get("mode").and_then(|v| v.as_str());
        extract_body(body_obj, body_mode)?
    } else {
        BodyType::None
    };

    Ok(RequestDefinition {
        id: Uuid::new_v4(),
        name,
        method,
        url,
        headers,
        query_params,
        body,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}

/// Parse a Postman URL (can be string or object)
fn parse_postman_url(url_value: Option<&Value>) -> ImportResult<String> {
    match url_value {
        None => Ok(String::new()),
        Some(Value::String(s)) => Ok(s.clone()),
        Some(Value::Object(obj)) => {
            // Get the raw URL if available
            if let Some(raw) = obj.get("raw").and_then(|v| v.as_str()) {
                return Ok(raw.to_string());
            }

            // Otherwise construct from protocol, host, and path
            let protocol = obj.get("protocol").and_then(|v| v.as_str()).unwrap_or("https");
            let host = obj.get("host").and_then(|v| v.as_array());

            let host_str = match host {
                Some(host_arr) => {
                    let parts: Vec<&str> = host_arr
                        .iter()
                        .filter_map(|v| v.as_str())
                        .collect();
                    parts.join(".")
                }
                None => "localhost".to_string(),
            };

            let path = obj.get("path").and_then(|v| v.as_array());
            let path_str = match path {
                Some(path_arr) => {
                    let parts: Vec<&str> = path_arr
                        .iter()
                        .filter_map(|v| v.as_str())
                        .collect();
                    if parts.is_empty() {
                        String::new()
                    } else {
                        "/".to_string() + &parts.join("/")
                    }
                }
                None => String::new(),
            };

            Ok(format!("{}://{}{}", protocol, host_str, path_str))
        }
        Some(_) => Ok(String::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_postman_collection() {
        let postman_json = serde_json::json!({
            "info": {
                "name": "Test Collection",
                "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
            },
            "item": [
                {
                    "name": "Get Users",
                    "request": {
                        "method": "GET",
                        "url": {
                            "raw": "https://api.example.com/users",
                            "protocol": "https",
                            "host": ["api", "example", "com"],
                            "path": ["users"]
                        },
                        "header": []
                    }
                }
            ]
        });

        let result = parse_postman_collection(&postman_json);
        assert!(result.is_ok());

        let collection = result.unwrap();
        assert_eq!(collection.name, "Test Collection");
        assert_eq!(collection.requests.len(), 1);
    }

    #[test]
    fn test_parse_postman_request_with_body() {
        let postman_json = serde_json::json!({
            "info": {
                "name": "Test Collection",
                "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
            },
            "item": [
                {
                    "name": "Create User",
                    "request": {
                        "method": "POST",
                        "url": "https://api.example.com/users",
                        "header": [
                            {
                                "key": "Content-Type",
                                "value": "application/json"
                            }
                        ],
                        "body": {
                            "mode": "raw",
                            "raw": "{\"name\":\"John Doe\"}"
                        }
                    }
                }
            ]
        });

        let result = parse_postman_collection(&postman_json);
        assert!(result.is_ok());

        let collection = result.unwrap();
        assert_eq!(collection.requests.len(), 1);

        let request = collection.requests.values().next().unwrap();
        assert_eq!(request.name, "Create User");
        assert_eq!(request.method, HttpMethod::POST);
        assert_eq!(request.headers.len(), 1);
        assert!(matches!(request.body, BodyType::Raw { .. }));
    }
}
