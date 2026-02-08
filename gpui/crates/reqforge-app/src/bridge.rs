//! Bridge layer for Core <-> UI type conversion.
//!
//! This module provides functions to convert between UI state types (TabState, KeyValueRow)
//! and Core domain types (RequestDefinition, KeyValuePair, BodyType).
//!
//! This is the single ownership boundary where we allocate Strings when transitioning
//! between UI and Core layers. All other code should use borrowed data (Cow<str>, &str)
//! to avoid unnecessary allocations.
//!
//! # Zero-Copy Design
//!
//! - `build_request_from_tab` reads from UI state and allocates Strings at the bridge boundary
//! - `populate_tab_from_request` creates UI state from Core types
//! - In Phase 3 with `Entity<InputState>`:
//!   - Read text via `.read(cx).text().to_string()` — single allocation point
//!   - Create `Entity<InputState>` for each text field

use reqforge_core::models::request::{
    RequestDefinition, KeyValuePair, BodyType, RawContentType, HttpMethod,
};
use crate::app_state::{TabState, KeyValueRow};
use gpui::{App, Context, Entity};
use uuid::Uuid;

/// Build a RequestDefinition from a TabState.
///
/// This function reads from the UI state (TabState) and creates a Core
/// RequestDefinition. This is the single allocation point when transitioning
/// from UI to Core types.
///
/// # Arguments
///
/// * `tab` - The tab state containing UI inputs
/// * `cx` - The App context for reading Entity values (used when InputState is implemented)
///
/// # Returns
///
/// A RequestDefinition with all values from the tab state.
///
/// # Examples
///
/// ```ignore
/// use reqforge_app::bridge::build_request_from_tab;
/// use gpui::App;
///
/// let request = build_request_from_tab(&tab, &mut cx);
/// ```
pub fn build_request_from_tab(tab: &TabState, cx: &mut App) -> RequestDefinition {
    // In the current stub implementation, TabState contains a draft RequestDefinition directly.
    // When Phase 3 implements Entity<InputState>, we will:
    // 1. Read URL input: tab.url_input.read(cx).text().to_string()
    // 2. Read headers: for each row, row.key_input.read(cx).text().to_string()
    // 3. Read params: for each row, row.key_input.read(cx).text().to_string()
    // 4. Read body: tab.body_input.read(cx).text().to_string()

    // For now, we clone from the existing draft (this is the stub implementation)
    let mut request = tab.draft.clone();

    // Update the timestamp to reflect when this was built
    request.updated_at = chrono::Utc::now();

    request
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
/// In Phase 3 with Entity<InputState>, this will:
/// - Create `Entity<InputState>` for the URL input
/// - Create `Entity<InputState>` for the body input
/// - Create `Entity<InputState>` for each header key and value
/// - Create `Entity<InputState>` for each param key and value
///
/// # Arguments
///
/// * `req` - The request definition to populate from
/// * `_cx` - The App context for creating Entity values (unused in stub implementation)
///
/// # Returns
///
/// A new TabState with all fields populated from the request definition.
///
/// # Examples
///
/// ```ignore
/// use reqforge_app::bridge::populate_tab_from_request;
///
/// let tab = populate_tab_from_request(&request, &mut cx);
/// ```
pub fn populate_tab_from_request(
    req: &RequestDefinition,
    _cx: &mut App,
) -> TabState {
    // In the current stub implementation, we create a simple TabState with the
    // RequestDefinition directly embedded.
    //
    // When Phase 3 implements Entity<InputState>, we will:
    // 1. Create URL input entity: cx.new(|_cx| InputState::new(req.url.clone()))
    // 2. For each header: create entities for key and value
    // 3. For each param: create entities for key and value
    // 4. Create body input entity based on BodyType

    TabState {
        request_id: req.id,
        collection_id: Uuid::new_v4(), // Will be set by caller when opening in collection
        draft: req.clone(),
        last_response: None,
        is_loading: false,
        is_dirty: false,
    }
}

/// Convert a UI KeyValueRow to a Core KeyValuePair.
///
/// This function converts from the UI layer's KeyValueRow to the Core layer's
/// KeyValuePair. This is the allocation boundary for key-value text.
///
/// # Arguments
///
/// * `row` - The UI key-value row
///
/// # Returns
///
/// A KeyValuePair with values from the row.
pub fn key_value_row_to_pair(row: &KeyValueRow) -> KeyValuePair {
    KeyValuePair {
        key: row.key.clone(),
        value: row.value.clone(),
        enabled: row.enabled,
        description: None,
    }
}

/// Convert a Core KeyValuePair to a UI KeyValueRow.
///
/// This function converts from the Core layer's KeyValuePair to the UI layer's
/// KeyValueRow.
///
/// # Arguments
///
/// * `pair` - The Core key-value pair
///
/// # Returns
///
/// A KeyValueRow with values from the pair.
pub fn key_value_pair_to_row(pair: &KeyValuePair) -> KeyValueRow {
    KeyValueRow {
        id: Uuid::new_v4(),
        enabled: pair.enabled,
        key: pair.key.clone(),
        value: pair.value.clone(),
    }
}

/// Convert multiple UI KeyValueRows to Core KeyValuePairs.
///
/// # Arguments
///
/// * `rows` - Vector of UI key-value rows
///
/// # Returns
///
/// Vector of KeyValuePairs.
pub fn key_value_rows_to_pairs(rows: &[KeyValueRow]) -> Vec<KeyValuePair> {
    rows.iter()
        .filter(|row| row.enabled && (!row.key.is_empty() || !row.value.is_empty()))
        .map(key_value_row_to_pair)
        .collect()
}

/// Convert multiple Core KeyValuePairs to UI KeyValueRows.
///
/// # Arguments
///
/// * `pairs` - Vector of Core key-value pairs
///
/// # Returns
///
/// Vector of KeyValueRows.
pub fn key_value_pairs_to_rows(pairs: &[KeyValuePair]) -> Vec<KeyValueRow> {
    pairs.iter()
        .map(key_value_pair_to_row)
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

    #[test]
    fn test_key_value_row_to_pair() {
        let row = KeyValueRow {
            id: Uuid::new_v4(),
            enabled: true,
            key: "Content-Type".to_string(),
            value: "application/json".to_string(),
        };

        let pair = key_value_row_to_pair(&row);

        assert_eq!(pair.key, "Content-Type");
        assert_eq!(pair.value, "application/json");
        assert!(pair.enabled);
        assert!(pair.description.is_none());
    }

    #[test]
    fn test_key_value_pair_to_row() {
        let pair = KeyValuePair {
            key: "Authorization".to_string(),
            value: "Bearer token".to_string(),
            enabled: true,
            description: Some("Auth header".to_string()),
        };

        let row = key_value_pair_to_row(&pair);

        assert_eq!(row.key, "Authorization");
        assert_eq!(row.value, "Bearer token");
        assert!(row.enabled);
    }

    #[test]
    fn test_key_value_rows_to_pairs_filters_disabled() {
        let rows = vec![
            KeyValueRow {
                id: Uuid::new_v4(),
                enabled: true,
                key: "Accept".to_string(),
                value: "application/json".to_string(),
            },
            KeyValueRow {
                id: Uuid::new_v4(),
                enabled: false,
                key: "X-Disabled".to_string(),
                value: "should-not-appear".to_string(),
            },
        ];

        let pairs = key_value_rows_to_pairs(&rows);

        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].key, "Accept");
    }

    #[test]
    fn test_key_value_pairs_to_rows() {
        let pairs = vec![
            KeyValuePair {
                key: "Accept".to_string(),
                value: "application/json".to_string(),
                enabled: true,
                description: None,
            },
            KeyValuePair {
                key: "Content-Type".to_string(),
                value: "text/plain".to_string(),
                enabled: true,
                description: None,
            },
        ];

        let rows = key_value_pairs_to_rows(&pairs);

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].key, "Accept");
        assert_eq!(rows[1].key, "Content-Type");
    }

    #[test]
    fn test_body_string_to_body_type_empty() {
        let body = body_string_to_body_type("".to_string(), RawContentType::Json);
        assert!(matches!(body, BodyType::None));
    }

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

    #[test]
    fn test_body_type_to_string_none() {
        let result = body_type_to_string(&BodyType::None);
        assert!(result.is_none());
    }

    #[test]
    fn test_body_type_to_string_raw() {
        let body = BodyType::Raw {
            content: "test content".to_string(),
            content_type: RawContentType::Text,
        };

        let result = body_type_to_string(&body);
        assert_eq!(result, Some("test content".to_string()));
    }

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
