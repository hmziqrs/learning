//! Bridge layer for Core <-> UI type conversion.
//!
//! This module provides functions to convert between UI state types (TabState, KeyValueRow)
//! and Core domain types (RequestDefinition, KeyValuePair, BodyType).
//!
//! This is the single ownership boundary where we allocate Strings when transitioning
//! between UI and Core layers. All other code should use borrowed data (Cow<str>, &str)
//! to avoid unnecessary allocations.
//!
//! # Zero-Copy Design (with Entity<InputState>)
//!
//! - `TabState::to_request_definition(cx)` reads from UI state and allocates Strings at the bridge boundary
//! - `KeyValueRow::to_kv_pair(cx)` converts individual rows with context
//! - InputState is read via `.read(cx).text().to_string()` — single allocation point

use gpui::{App, AppContext, Context, Window, Entity};
use gpui_component::input::InputState;
use reqforge_core::models::request::{
    RequestDefinition, KeyValuePair, BodyType, RawContentType, HttpMethod,
};
use crate::app_state::{AppState, TabState, KeyValueRow};
use uuid::Uuid;

/// Build a RequestDefinition from a TabState.
///
/// This function is now a thin wrapper around `TabState::to_request_definition()`.
/// The conversion logic lives on TabState since it requires GPUI context to
/// read from Entity<InputState> fields.
///
/// # Examples
///
/// ```ignore
/// use reqforge_app::bridge::build_request_from_tab;
///
/// let request = build_request_from_tab(&tab, cx);
/// ```
pub fn build_request_from_tab(tab: &TabState, cx: &mut Context<AppState>) -> RequestDefinition {
    tab.to_request_definition(cx)
}

/// Build a RequestDefinition from individual UI components.
///
/// This is an alternative to `build_request_from_tab` that builds a request
/// from individual component values. This is useful when you have direct
/// access to the input values rather than a full TabState.
///
/// # Arguments
///
/// * `request_id` - The UUID of the request
/// * `name` - Request name
/// * `method` - HTTP method
/// * `url` - Request URL (as allocated String from UI input)
/// * `headers` - Vector of key-value pairs from UI headers editor
/// * `params` - Vector of key-value pairs from UI params editor
/// * `body` - Body type from UI body editor
///
/// # Returns
///
/// A RequestDefinition with all provided values.
pub fn build_request_from_components(
    request_id: Uuid,
    name: String,
    method: HttpMethod,
    url: String,
    headers: Vec<KeyValuePair>,
    params: Vec<KeyValuePair>,
    body: BodyType,
) -> RequestDefinition {
    let now = chrono::Utc::now();

    RequestDefinition {
        id: request_id,
        name,
        method,
        url,
        headers,
        query_params: params,
        body,
        created_at: now,
        updated_at: now,
    }
}

/// Populate a TabState from a RequestDefinition.
///
/// This function creates UI state (TabState) from a Core RequestDefinition.
/// It initializes all the UI components with values from the request.
///
/// This function requires GPUI context to create Entity<InputState> for each
/// text field (URL, body, headers, params).
///
/// # Arguments
///
/// * `req` - The request definition to populate from
/// * `collection_id` - The ID of the collection this request belongs to
/// * `window` - The window context for creating entities
/// * `cx` - The Context for creating entities (works with Context<AppState> or App)
///
/// # Examples
///
/// ```ignore
/// use reqforge_app::bridge::populate_tab_from_request;
///
/// let tab = populate_tab_from_request(&request, collection_id, window, cx);
/// ```
pub fn populate_tab_from_request(
    req: &RequestDefinition,
    collection_id: Uuid,
    window: &mut Window,
    cx: &mut Context<AppState>,
) -> TabState {
    // Create URL input state
    let url_input = cx.new(|cx| {
        InputState::new(window, cx).default_value(req.url.clone())
    });

    // Create body input state based on BodyType
    let body_content = match &req.body {
        BodyType::Raw { content, .. } => content.clone(),
        BodyType::FormUrlEncoded(_) => {
            // Convert form data to URL-encoded string
            body_type_to_string(&req.body).unwrap_or_default()
        }
        BodyType::None => String::new(),
    };
    let body_input = cx.new(|cx| {
        InputState::new(window, cx).default_value(body_content)
    });

    // Create header rows from KeyValuePair vector
    let headers: Vec<KeyValueRow> = req
        .headers
        .iter()
        .map(|pair| {
            let key_input = cx.new(|cx| {
                InputState::new(window, cx).default_value(pair.key.clone())
            });
            let value_input = cx.new(|cx| {
                InputState::new(window, cx).default_value(pair.value.clone())
            });
            KeyValueRow::with_enabled(key_input, value_input, pair.enabled)
        })
        .collect();

    // Create param rows from KeyValuePair vector
    let params: Vec<KeyValueRow> = req
        .query_params
        .iter()
        .map(|pair| {
            let key_input = cx.new(|cx| {
                InputState::new(window, cx).default_value(pair.key.clone())
            });
            let value_input = cx.new(|cx| {
                InputState::new(window, cx).default_value(pair.value.clone())
            });
            KeyValueRow::with_enabled(key_input, value_input, pair.enabled)
        })
        .collect();

    TabState::new(
        req.id,
        collection_id,
        req.name.clone(),
        req.method.clone(),
        url_input,
        body_input,
        headers,
        params,
    )
}

/// Populate a TabState from a RequestDefinition using App context.
///
/// This is a convenience wrapper that converts App context to Context<AppState>
/// for entity creation. This is useful when called from UI components that have
/// App context available (e.g., Tree render callbacks).
///
/// # Arguments
///
/// * `req` - The request definition to populate from
/// * `collection_id` - The ID of the collection this request belongs to
/// * `app_state` - The AppState entity to create context from
/// * `window` - The window context for creating entities
/// * `cx` - The App context for creating entities
///
/// # Examples
///
/// ```ignore
/// use reqforge_app::bridge::populate_tab_from_request_with_app;
///
/// let tab = populate_tab_from_request_with_app(&request, collection_id, app_state, window, cx);
/// ```
pub fn populate_tab_from_request_with_app(
    req: &RequestDefinition,
    collection_id: Uuid,
    app_state: &Entity<AppState>,
    window: &mut Window,
    cx: &mut Context<AppState>,
) -> TabState {
    populate_tab_from_request(req, collection_id, window, cx)
}

/// Convert a UI KeyValueRow to a Core KeyValuePair.
///
/// This function is now a thin wrapper around `KeyValueRow::to_kv_pair()`.
/// The conversion logic lives on KeyValueRow since it requires GPUI context to
/// read from Entity<InputState> fields.
///
/// # Examples
///
/// ```ignore
/// use reqforge_app::bridge::key_value_row_to_pair;
///
/// let pair = key_value_row_to_pair(&row, cx);
/// ```
pub fn key_value_row_to_pair(row: &KeyValueRow, cx: &mut Context<AppState>) -> KeyValuePair {
    row.to_kv_pair(cx)
}

/// Convert multiple UI KeyValueRows to Core KeyValuePairs.
///
/// # Examples
///
/// ```ignore
/// use reqforge_app::bridge::key_value_rows_to_pairs;
///
/// let pairs = key_value_rows_to_pairs(&rows, cx);
/// ```
pub fn key_value_rows_to_pairs(rows: &[KeyValueRow], cx: &mut Context<AppState>) -> Vec<KeyValuePair> {
    // Collect valid rows first to avoid borrow checker issues with cx
    let valid_rows: Vec<_> = rows
        .iter()
        .filter(|row| row.enabled && row.is_valid(cx))
        .collect();

    // Then map to pairs
    valid_rows
        .into_iter()
        .map(|row| row.to_kv_pair(cx))
        .collect()
}

/// Convert a body content string to a BodyType.
///
/// # Arguments
///
/// * `content` - Body content string
/// * `content_type` - The raw content type (JSON, XML, Text, HTML)
///
/// # Returns
///
/// A BodyType::Raw variant with the specified content.
pub fn body_string_to_body_type(
    content: String,
    content_type: RawContentType,
) -> BodyType {
    if content.is_empty() {
        BodyType::None
    } else {
        BodyType::Raw {
            content,
            content_type,
        }
    }
}

/// Get the body content string from a BodyType.
///
/// # Arguments
///
/// * `body` - The BodyType to extract content from
///
/// # Returns
///
/// Option with the body content string, or None if BodyType is None.
pub fn body_type_to_string(body: &BodyType) -> Option<String> {
    match body {
        BodyType::None => None,
        BodyType::Raw { content, .. } => Some(content.clone()),
        BodyType::FormUrlEncoded(pairs) => {
            // Convert form data to URL-encoded string
            let encoded: Vec<String> = pairs
                .iter()
                .filter(|p| p.enabled)
                .map(|p| {
                    format!(
                        "{}={}",
                        urlencoding::encode(&p.key),
                        urlencoding::encode(&p.value)
                    )
                })
                .collect();
            if encoded.is_empty() {
                None
            } else {
                Some(encoded.join("&"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test body_string_to_body_type with empty content
    #[test]
    fn test_body_string_to_body_type_empty() {
        let body = body_string_to_body_type("".to_string(), RawContentType::Json);
        assert!(matches!(body, BodyType::None));
    }

    /// Test body_string_to_body_type with content
    #[test]
    fn test_body_string_to_body_type_with_content() {
        let body = body_string_to_body_type(
            "{\"key\":\"value\"}".to_string(),
            RawContentType::Json,
        );
        assert!(matches!(body, BodyType::Raw { .. }));

        if let BodyType::Raw { content, content_type } = body {
            assert_eq!(content, "{\"key\":\"value\"}");
            assert!(matches!(content_type, RawContentType::Json));
        }
    }

    /// Test body_type_to_string with None
    #[test]
    fn test_body_type_to_string_none() {
        let result = body_type_to_string(&BodyType::None);
        assert!(result.is_none());
    }

    /// Test body_type_to_string with Raw body
    #[test]
    fn test_body_type_to_string_raw() {
        let body = BodyType::Raw {
            content: "test content".to_string(),
            content_type: RawContentType::Text,
        };

        let result = body_type_to_string(&body);
        assert_eq!(result, Some("test content".to_string()));
    }

    /// Test body_type_to_string with FormUrlEncoded
    #[test]
    fn test_body_type_to_string_form() {
        let pairs = vec![
            KeyValuePair {
                key: "name".to_string(),
                value: "John Doe".to_string(),
                enabled: true,
                description: None,
            },
            KeyValuePair {
                key: "age".to_string(),
                value: "30".to_string(),
                enabled: true,
                description: None,
            },
        ];

        let body = BodyType::FormUrlEncoded(pairs);
        let result = body_type_to_string(&body);

        assert!(result.is_some());
        let result_str = result.unwrap();
        // URL encoding varies, so just check both keys are present
        assert!(result_str.contains("name="));
        assert!(result_str.contains("age="));
    }

    /// Test build_request_from_components
    #[test]
    fn test_build_request_from_components() {
        let request_id = Uuid::new_v4();
        let headers = vec![
            KeyValuePair {
                key: "Accept".to_string(),
                value: "application/json".to_string(),
                enabled: true,
                description: None,
            },
        ];
        let params = vec![
            KeyValuePair {
                key: "page".to_string(),
                value: "1".to_string(),
                enabled: true,
                description: None,
            },
        ];

        let request = build_request_from_components(
            request_id,
            "Test Request".to_string(),
            HttpMethod::GET,
            "https://api.example.com".to_string(),
            headers,
            params,
            BodyType::None,
        );

        assert_eq!(request.id, request_id);
        assert_eq!(request.name, "Test Request");
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.url, "https://api.example.com");
        assert_eq!(request.headers.len(), 1);
        assert_eq!(request.query_params.len(), 1);
    }
}
