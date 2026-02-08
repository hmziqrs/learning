//! Request validation module for reqforge-core.
//!
//! Provides comprehensive validation for HTTP requests before execution.

use crate::models::request::{RequestDefinition, HttpMethod, BodyType, RawContentType};
use std::collections::HashSet;

/// Try to parse a URL, returning an error if it fails
fn parse_url(url: &str) -> Result<url::Url, String> {
    url::Url::parse(url).map_err(|e| e.to_string())
}

/// Validation errors that can occur when validating a request.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ValidationError {
    #[error("URL validation error: {message}")]
    InvalidUrl { message: String },

    #[error("Header validation error: {message}")]
    InvalidHeader { message: String },

    #[error("Body validation error: {message}")]
    InvalidBody { message: String },

    #[error("Method validation error: {message}")]
    InvalidMethod { message: String },

    #[error("Request validation failed:\n  - {0}")]
    MultipleErrors(String),
}

impl ValidationError {
    /// Create a new URL validation error.
    pub fn invalid_url(message: impl Into<String>) -> Self {
        Self::InvalidUrl {
            message: message.into(),
        }
    }

    /// Create a new header validation error.
    pub fn invalid_header(message: impl Into<String>) -> Self {
        Self::InvalidHeader {
            message: message.into(),
        }
    }

    /// Create a new body validation error.
    pub fn invalid_body(message: impl Into<String>) -> Self {
        Self::InvalidBody {
            message: message.into(),
        }
    }

    /// Create a new method validation error.
    pub fn invalid_method(message: impl Into<String>) -> Self {
        Self::InvalidMethod {
            message: message.into(),
        }
    }

    /// Combine multiple validation errors into a single error.
    pub fn multiple_errors(errors: Vec<String>) -> Self {
        Self::MultipleErrors(errors.join("\n  - "))
    }
}

/// Result type for validation operations.
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Validate a URL string.
///
/// Checks:
/// - URL is not empty
/// - URL can be parsed
/// - URL has a valid scheme (http or https)
/// - URL has a valid host
///
/// # Examples
///
/// ```
/// use reqforge_core::validation::validate_url;
///
/// assert!(validate_url("https://example.com").is_ok());
/// assert!(validate_url("invalid-url").is_err());
/// ```
pub fn validate_url(url: &str) -> ValidationResult<()> {
    // Check if URL is empty
    if url.trim().is_empty() {
        return Err(ValidationError::invalid_url("URL cannot be empty"));
    }

    // Try to parse the URL
    let parsed = parse_url(url)
        .map_err(|e| ValidationError::invalid_url(format!("Failed to parse URL: {}", e)))?;

    // Check if scheme is http or https
    let scheme = parsed.scheme();
    if scheme != "http" && scheme != "https" {
        return Err(ValidationError::invalid_url(format!(
            "Invalid scheme '{}'. Only 'http' and 'https' are supported",
            scheme
        )));
    }

    // Check if host exists
    if parsed.host().is_none() {
        return Err(ValidationError::invalid_url("URL must have a valid host"));
    }

    // Check if host_str is not empty
    if parsed.host_str().map_or(true, |h: &str| h.is_empty()) {
        return Err(ValidationError::invalid_url("URL host cannot be empty"));
    }

    Ok(())
}

/// Validate a single header key-value pair.
///
/// Checks:
/// - Header name is not empty
/// - Header name contains valid characters
/// - Header value is valid (most characters allowed)
///
/// # Examples
///
/// ```
/// use reqforge_core::validation::validate_header_pair;
///
/// assert!(validate_header_pair("Content-Type", "application/json").is_ok());
/// assert!(validate_header_pair("", "value").is_err());
/// ```
pub fn validate_header_pair(key: &str, value: &str) -> ValidationResult<()> {
    // Check if key is empty
    if key.trim().is_empty() {
        return Err(ValidationError::invalid_header("Header name cannot be empty"));
    }

    // Validate header name characters (RFC 7230)
    // Header names should be printable ASCII characters excluding delimiters
    for ch in key.chars() {
        let is_valid = ch.is_ascii()
            && !matches!(
                ch,
                '(' | ')' | '<' | '>' | '@' | ',' | ';' | ':' | '\\' | '"' | '/' | '[' | ']' | '?' | '=' | '{' | '}' | ' ' | '\t'
            );
        if !is_valid {
            return Err(ValidationError::invalid_header(format!(
                "Header name contains invalid character '{}': {}",
                ch, key
            )));
        }
    }

    // Header values can contain most characters but should not contain control characters
    for ch in value.chars() {
        if ch.is_ascii_control() {
            return Err(ValidationError::invalid_header(format!(
                "Header value contains control character in header '{}'",
                key
            )));
        }
    }

    Ok(())
}

/// Validate all headers in a list.
///
/// Collects all validation errors for headers and returns them together.
///
/// # Examples
///
/// ```
/// use reqforge_core::validation::validate_headers;
/// use reqforge_core::models::request::KeyValuePair;
///
/// let headers = vec![
///     KeyValuePair { key: "Content-Type".into(), value: "application/json".into(), enabled: true, description: None },
/// ];
/// assert!(validate_headers(&headers).is_ok());
/// ```
pub fn validate_headers(headers: &[crate::models::request::KeyValuePair]) -> ValidationResult<()> {
    let mut errors = Vec::new();

    // Track duplicate header names (only for enabled headers)
    let mut seen_headers = HashSet::new();

    for header in headers.iter().filter(|h| h.enabled) {
        // Validate the header pair
        if let Err(e) = validate_header_pair(&header.key, &header.value) {
            errors.push(e.to_string());
            continue;
        }

        // Check for duplicate headers (case-insensitive)
        let key_lower = header.key.to_lowercase();
        if seen_headers.contains(&key_lower) {
            errors.push(format!("Duplicate header detected: '{}'", header.key));
        } else {
            seen_headers.insert(key_lower);
        }
    }

    if !errors.is_empty() {
        return Err(ValidationError::invalid_header(format!(
            "Multiple header validation errors:\n  - {}",
            errors.join("\n  - ")
        )));
    }

    Ok(())
}

/// Validate that the body matches the content-type and HTTP method.
///
/// Checks:
/// - Body is appropriate for the HTTP method
/// - JSON body is valid JSON (when content-type is JSON)
/// - XML body is well-formed (when content-type is XML)
/// - Form data has valid key-value pairs
///
/// # Examples
///
/// ```
/// use reqforge_core::validation::validate_body;
/// use reqforge_core::models::request::{BodyType, RawContentType, HttpMethod};
///
/// // JSON body with valid JSON
/// assert!(validate_body(
///     &HttpMethod::POST,
///     &BodyType::Raw { content: "{\"key\":\"value\"}".into(), content_type: RawContentType::Json }
/// ).is_ok());
/// ```
pub fn validate_body(method: &HttpMethod, body: &BodyType) -> ValidationResult<()> {
    match body {
        BodyType::None => Ok(()),

        BodyType::Raw { content, content_type } => {
            // Check if body is allowed for this method
            if !method_supports_body(method) {
                return Err(ValidationError::invalid_body(format!(
                    "HTTP method {} does not support a request body",
                    method
                )));
            }

            // Check if content is not empty (unless it's intentional)
            if content.trim().is_empty() {
                return Err(ValidationError::invalid_body(
                    "Request body content is empty",
                ));
            }

            // Validate content based on content type
            match content_type {
                RawContentType::Json => {
                    // Validate JSON
                    serde_json::from_str::<serde_json::Value>(content)
                        .map_err(|e| {
                            ValidationError::invalid_body(format!(
                                "Invalid JSON body: {}",
                                e.to_string().lines().next().unwrap_or(&e.to_string())
                            ))
                        })?;
                }
                RawContentType::Xml => {
                    // Basic XML validation - check for well-formedness
                    if !content.trim().starts_with('<') || !content.trim().ends_with('>') {
                        return Err(ValidationError::invalid_body(
                            "XML body must be well-formed (should start with '<' and end with '>')",
                        ));
                    }
                }
                RawContentType::Text | RawContentType::Html => {
                    // Text and HTML are always valid
                }
            }

            Ok(())
        }

        BodyType::FormUrlEncoded(pairs) => {
            // Check if body is allowed for this method
            if !method_supports_body(method) {
                return Err(ValidationError::invalid_body(format!(
                    "HTTP method {} does not support a request body",
                    method
                )));
            }

            // Check if there's at least one pair
            if pairs.is_empty() {
                return Err(ValidationError::invalid_body(
                    "Form-encoded body must contain at least one field",
                ));
            }

            // Validate each form field
            for pair in pairs.iter().filter(|p| p.enabled) {
                if pair.key.trim().is_empty() {
                    return Err(ValidationError::invalid_body(
                        "Form field name cannot be empty",
                    ));
                }
            }

            Ok(())
        }
    }
}

/// Check if the HTTP method supports a request body.
fn method_supports_body(method: &HttpMethod) -> bool {
    matches!(
        method,
        HttpMethod::POST | HttpMethod::PUT | HttpMethod::PATCH
    )
}

/// Validate a complete request definition.
///
/// This performs all validation checks:
/// - URL validation
/// - Header validation
/// - Body validation
/// - Method validation (check body compatibility)
///
/// Returns a detailed error with all validation issues.
///
/// # Examples
///
/// ```
/// use reqforge_core::validation::validate_request;
/// use reqforge_core::models::request::{RequestDefinition, HttpMethod, BodyType};
///
/// let req = RequestDefinition::new("Test", HttpMethod::GET, "https://api.example.com");
/// assert!(validate_request(&req).is_ok());
/// ```
pub fn validate_request(req: &RequestDefinition) -> ValidationResult<()> {
    let mut errors = Vec::new();

    // Validate URL
    if let Err(e) = validate_url(&req.url) {
        errors.push(e.to_string());
    }

    // Validate headers
    if let Err(e) = validate_headers(&req.headers) {
        errors.push(e.to_string());
    }

    // Validate body
    if let Err(e) = validate_body(&req.method, &req.body) {
        errors.push(e.to_string());
    }

    // Additional check: body methods with no body
    if method_supports_body(&req.method) && matches!(req.body, BodyType::None) {
        // This is a warning, not an error - some APIs expect empty POST/PUT/PATCH
    }

    if !errors.is_empty() {
        return Err(ValidationError::multiple_errors(errors));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::request::{KeyValuePair, HttpMethod};

    // URL validation tests
    #[test]
    fn test_validate_url_valid_https() {
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("https://api.example.com/v1/users").is_ok());
        assert!(validate_url("https://example.com:8080/path").is_ok());
    }

    #[test]
    fn test_validate_url_valid_http() {
        assert!(validate_url("http://example.com").is_ok());
        assert!(validate_url("http://localhost:3000").is_ok());
    }

    #[test]
    fn test_validate_url_empty() {
        let result = validate_url("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_url_invalid_scheme() {
        let result = validate_url("ftp://example.com");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("scheme"));
    }

    #[test]
    fn test_validate_url_no_scheme() {
        let result = validate_url("example.com");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to parse"));
    }

    #[test]
    fn test_validate_url_no_host() {
        let result = validate_url("https://");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("host"));
    }

    // Header validation tests
    #[test]
    fn test_validate_header_pair_valid() {
        assert!(validate_header_pair("Content-Type", "application/json").is_ok());
        assert!(validate_header_pair("Authorization", "Bearer token123").is_ok());
        assert!(validate_header_pair("X-Custom-Header", "custom value").is_ok());
    }

    #[test]
    fn test_validate_header_pair_empty_key() {
        let result = validate_header_pair("", "value");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_header_pair_invalid_characters() {
        let result = validate_header_pair("Invalid:Header", "value");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid character"));
    }

    #[test]
    fn test_validate_header_pair_control_characters() {
        let result = validate_header_pair("Content-Type", "value\nwith\nnewlines");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("control character"));
    }

    #[test]
    fn test_validate_headers_valid() {
        let headers = vec![
            KeyValuePair {
                key: "Content-Type".into(),
                value: "application/json".into(),
                enabled: true,
                description: None,
            },
            KeyValuePair {
                key: "Authorization".into(),
                value: "Bearer token".into(),
                enabled: true,
                description: None,
            },
        ];
        assert!(validate_headers(&headers).is_ok());
    }

    #[test]
    fn test_validate_headers_empty() {
        let headers: Vec<KeyValuePair> = vec![];
        assert!(validate_headers(&headers).is_ok());
    }

    #[test]
    fn test_validate_headers_duplicate() {
        let headers = vec![
            KeyValuePair {
                key: "Content-Type".into(),
                value: "application/json".into(),
                enabled: true,
                description: None,
            },
            KeyValuePair {
                key: "content-type".into(),
                value: "text/plain".into(),
                enabled: true,
                description: None,
            },
        ];
        let result = validate_headers(&headers);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Duplicate"));
    }

    #[test]
    fn test_validate_headers_disabled_not_checked() {
        let headers = vec![
            KeyValuePair {
                key: "Content-Type".into(),
                value: "application/json".into(),
                enabled: true,
                description: None,
            },
            KeyValuePair {
                key: "".into(),
                value: "value".into(),
                enabled: false,
                description: None,
            },
        ];
        assert!(validate_headers(&headers).is_ok());
    }

    // Body validation tests
    #[test]
    fn test_validate_body_none() {
        assert!(validate_body(&HttpMethod::GET, &BodyType::None).is_ok());
        assert!(validate_body(&HttpMethod::POST, &BodyType::None).is_ok());
    }

    #[test]
    fn test_validate_body_json_valid() {
        let body = BodyType::Raw {
            content: "{\"key\":\"value\"}".into(),
            content_type: RawContentType::Json,
        };
        assert!(validate_body(&HttpMethod::POST, &body).is_ok());
    }

    #[test]
    fn test_validate_body_json_invalid() {
        let body = BodyType::Raw {
            content: "{invalid json}".into(),
            content_type: RawContentType::Json,
        };
        let result = validate_body(&HttpMethod::POST, &body);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid JSON"));
    }

    #[test]
    fn test_validate_body_xml_valid() {
        let body = BodyType::Raw {
            content: "<root><element>value</element></root>".into(),
            content_type: RawContentType::Xml,
        };
        assert!(validate_body(&HttpMethod::POST, &body).is_ok());
    }

    #[test]
    fn test_validate_body_xml_invalid() {
        let body = BodyType::Raw {
            content: "not xml".into(),
            content_type: RawContentType::Xml,
        };
        let result = validate_body(&HttpMethod::POST, &body);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("well-formed"));
    }

    #[test]
    fn test_validate_body_empty_content() {
        let body = BodyType::Raw {
            content: "".into(),
            content_type: RawContentType::Text,
        };
        let result = validate_body(&HttpMethod::POST, &body);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_validate_body_get_with_body() {
        let body = BodyType::Raw {
            content: "data".into(),
            content_type: RawContentType::Text,
        };
        let result = validate_body(&HttpMethod::GET, &body);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not support"));
    }

    #[test]
    fn test_validate_body_form_valid() {
        let pairs = vec![
            KeyValuePair {
                key: "field1".into(),
                value: "value1".into(),
                enabled: true,
                description: None,
            },
            KeyValuePair {
                key: "field2".into(),
                value: "value2".into(),
                enabled: true,
                description: None,
            },
        ];
        let body = BodyType::FormUrlEncoded(pairs);
        assert!(validate_body(&HttpMethod::POST, &body).is_ok());
    }

    #[test]
    fn test_validate_body_form_empty_key() {
        let pairs = vec![KeyValuePair {
            key: "".into(),
            value: "value".into(),
            enabled: true,
            description: None,
        }];
        let body = BodyType::FormUrlEncoded(pairs);
        let result = validate_body(&HttpMethod::POST, &body);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_body_form_no_pairs() {
        let pairs = vec![];
        let body = BodyType::FormUrlEncoded(pairs);
        let result = validate_body(&HttpMethod::POST, &body);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at least one"));
    }

    // Request validation tests
    #[test]
    fn test_validate_request_valid_get() {
        let req = RequestDefinition::new("Test", HttpMethod::GET, "https://api.example.com");
        assert!(validate_request(&req).is_ok());
    }

    #[test]
    fn test_validate_request_valid_post() {
        let mut req = RequestDefinition::new("Test", HttpMethod::POST, "https://api.example.com");
        req.body = BodyType::Raw {
            content: "{\"key\":\"value\"}".into(),
            content_type: RawContentType::Json,
        };
        assert!(validate_request(&req).is_ok());
    }

    #[test]
    fn test_validate_request_invalid_url() {
        let req = RequestDefinition::new("Test", HttpMethod::GET, "not a url");
        let result = validate_request(&req);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_request_invalid_header() {
        let mut req = RequestDefinition::new("Test", HttpMethod::GET, "https://api.example.com");
        req.headers.push(KeyValuePair {
            key: "Bad:Header".into(),
            value: "value".into(),
            enabled: true,
            description: None,
        });
        let result = validate_request(&req);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_request_invalid_json_body() {
        let mut req = RequestDefinition::new("Test", HttpMethod::POST, "https://api.example.com");
        req.body = BodyType::Raw {
            content: "{bad json}".into(),
            content_type: RawContentType::Json,
        };
        let result = validate_request(&req);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_request_multiple_errors() {
        let mut req = RequestDefinition::new("Test", HttpMethod::POST, "not a url");
        req.body = BodyType::Raw {
            content: "{bad json}".into(),
            content_type: RawContentType::Json,
        };
        let result = validate_request(&req);
        assert!(result.is_err());

        if let ValidationError::MultipleErrors(errors) = result.unwrap_err() {
            assert!(errors.len() >= 2);
        } else {
            panic!("Expected MultipleErrors variant");
        }
    }

    // Method supports body tests
    #[test]
    fn test_method_supports_body() {
        assert!(method_supports_body(&HttpMethod::POST));
        assert!(method_supports_body(&HttpMethod::PUT));
        assert!(method_supports_body(&HttpMethod::PATCH));
        assert!(!method_supports_body(&HttpMethod::GET));
        assert!(!method_supports_body(&HttpMethod::DELETE));
        assert!(!method_supports_body(&HttpMethod::HEAD));
        assert!(!method_supports_body(&HttpMethod::OPTIONS));
    }

    // ValidationError helper methods tests
    #[test]
    fn test_validation_error_helpers() {
        let err = ValidationError::invalid_url("test message");
        assert!(err.to_string().contains("test message"));

        let err = ValidationError::invalid_header("test header");
        assert!(err.to_string().contains("test header"));

        let err = ValidationError::invalid_body("test body");
        assert!(err.to_string().contains("test body"));

        let err = ValidationError::invalid_method("test method");
        assert!(err.to_string().contains("test method"));
    }
}
