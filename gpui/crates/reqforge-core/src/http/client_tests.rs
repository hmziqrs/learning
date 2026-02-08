#[cfg(test)]
mod tests {
    use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, path, query_param, header}};
    use wiremock::matchers::body_string;
    use std::time::Duration;
    use crate::http::client::HttpEngine;
    use crate::models::request::{RequestDefinition, HttpMethod, BodyType, RawContentType, KeyValuePair};

    /// Helper to create a test request definition
    fn create_test_request(method: HttpMethod, url: &str) -> RequestDefinition {
        RequestDefinition::new("test", method, url)
    }

    /// Helper to add headers to a request
    fn with_headers(mut req: RequestDefinition, headers: Vec<(&str, &str)>) -> RequestDefinition {
        for (key, value) in headers {
            req.headers.push(KeyValuePair {
                key: key.to_string(),
                value: value.to_string(),
                enabled: true,
                description: None,
            });
        }
        req
    }

    /// Helper to add query params to a request
    fn with_query_params(mut req: RequestDefinition, params: Vec<(&str, &str)>) -> RequestDefinition {
        for (key, value) in params {
            req.query_params.push(KeyValuePair {
                key: key.to_string(),
                value: value.to_string(),
                enabled: true,
                description: None,
            });
        }
        req
    }

    /// Helper to add JSON body to a request
    fn with_json_body(mut req: RequestDefinition, content: &str) -> RequestDefinition {
        req.body = BodyType::Raw {
            content: content.to_string(),
            content_type: RawContentType::Json,
        };
        req
    }

    /// Helper to add form-encoded body to a request
    fn with_form_body(mut req: RequestDefinition, pairs: Vec<(&str, &str)>) -> RequestDefinition {
        let form_pairs: Vec<KeyValuePair> = pairs
            .into_iter()
            .map(|(key, value)| KeyValuePair {
                key: key.to_string(),
                value: value.to_string(),
                enabled: true,
                description: None,
            })
            .collect();
        req.body = BodyType::FormUrlEncoded(form_pairs);
        req
    }

    #[tokio::test]
    async fn test_get_request_with_query_parameters() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock to expect GET request with query parameters
        Mock::given(method("GET"))
            .and(path("/api/users"))
            .and(query_param("page", "1"))
            .and(query_param("limit", "10"))
            .and(query_param("sort", "name"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "users": [
                    {"id": 1, "name": "Alice"},
                    {"id": 2, "name": "Bob"}
                ],
                "page": 1,
                "limit": 10
            })))
            .mount(&mock_server)
            .await;

        // Create HTTP engine and request
        let engine = HttpEngine::new();
        let url = format!("{}/api/users", mock_server.uri());
        let request = with_query_params(
            create_test_request(HttpMethod::GET, &url),
            vec![("page", "1"), ("limit", "10"), ("sort", "name")],
        );

        // Execute request
        let response = engine.execute(&request).await.unwrap();

        // Assert response
        assert_eq!(response.status, 200);
        assert_eq!(response.status_text, "OK");
        assert!(response.is_success());
        assert!(response.body_text.is_some());
        let body_text = response.body_text.as_ref().unwrap();
        assert!(body_text.contains("Alice"));
        assert!(body_text.contains("Bob"));
        assert!(response.elapsed >= Duration::ZERO);
    }

    #[tokio::test]
    async fn test_post_request_with_json_body() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock to expect POST request with JSON body
        let expected_body = r#"{"name":"Charlie","email":"charlie@example.com"}"#;
        Mock::given(method("POST"))
            .and(path("/api/users"))
            .and(header("content-type", "application/json"))
            .and(body_string(expected_body))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": 123,
                "name": "Charlie",
                "email": "charlie@example.com",
                "created_at": "2024-01-01T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        // Create HTTP engine and request
        let engine = HttpEngine::new();
        let url = format!("{}/api/users", mock_server.uri());
        let request = with_json_body(
            create_test_request(HttpMethod::POST, &url),
            expected_body,
        );

        // Execute request
        let response = engine.execute(&request).await.unwrap();

        // Assert response
        assert_eq!(response.status, 201);
        assert_eq!(response.status_text, "Created");
        assert!(response.is_success());
        assert!(response.body_text.is_some());
        let body_text = response.body_text.as_ref().unwrap();
        assert!(body_text.contains("Charlie"));
        assert!(body_text.contains("123"));
    }

    #[tokio::test]
    async fn test_post_request_with_form_encoded_data() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock to expect POST request with form-encoded data
        Mock::given(method("POST"))
            .and(path("/api/login"))
            .and(header("content-type", "application/x-www-form-urlencoded"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
                "user": {
                    "id": 456,
                    "username": "testuser"
                }
            })))
            .mount(&mock_server)
            .await;

        // Create HTTP engine and request
        let engine = HttpEngine::new();
        let url = format!("{}/api/login", mock_server.uri());
        let request = with_form_body(
            create_test_request(HttpMethod::POST, &url),
            vec![("username", "testuser"), ("password", "secret123")],
        );

        // Execute request
        let response = engine.execute(&request).await.unwrap();

        // Assert response
        assert_eq!(response.status, 200);
        assert!(response.is_success());
        assert!(response.body_text.is_some());
        let body_text = response.body_text.as_ref().unwrap();
        assert!(body_text.contains("token"));
        assert!(body_text.contains("testuser"));
    }

    #[tokio::test]
    async fn test_headers_handling() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock to expect specific headers
        Mock::given(method("GET"))
            .and(path("/api/protected"))
            .and(header("authorization", "Bearer token123"))
            .and(header("x-api-key", "api-key-456"))
            .and(header("accept", "application/json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("x-request-id", "req-789")
                    .insert_header("content-type", "application/json")
                    .set_body_json(serde_json::json!({
                        "message": "Access granted",
                        "authenticated": true
                    }))
            )
            .mount(&mock_server)
            .await;

        // Create HTTP engine and request with headers
        let engine = HttpEngine::new();
        let url = format!("{}/api/protected", mock_server.uri());
        let request = with_headers(
            create_test_request(HttpMethod::GET, &url),
            vec![
                ("authorization", "Bearer token123"),
                ("x-api-key", "api-key-456"),
                ("accept", "application/json"),
            ],
        );

        // Execute request
        let response = engine.execute(&request).await.unwrap();

        // Assert response
        assert_eq!(response.status, 200);
        assert!(response.is_success());

        // Check response headers
        assert!(response.headers.contains_key("content-type"));
        assert_eq!(response.headers.get("content-type").unwrap(), "application/json");
        assert!(response.headers.contains_key("x-request-id"));
        assert_eq!(response.headers.get("x-request-id").unwrap(), "req-789");
    }

    #[tokio::test]
    async fn test_response_parsing() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock with comprehensive response
        Mock::given(method("GET"))
            .and(path("/api/comprehensive"))
            .respond_with(
                ResponseTemplate::new(202)
                    .insert_header("x-custom-header", "custom-value")
                    .insert_header("cache-control", "no-cache")
                    .set_body_json(serde_json::json!({
                        "data": {
                            "items": ["a", "b", "c"],
                            "count": 3
                        },
                        "metadata": {
                            "timestamp": "2024-01-01T12:00:00Z"
                        }
                    }))
            )
            .mount(&mock_server)
            .await;

        // Create HTTP engine and request
        let engine = HttpEngine::new();
        let url = format!("{}/api/comprehensive", mock_server.uri());
        let request = create_test_request(HttpMethod::GET, &url);

        // Execute request
        let response = engine.execute(&request).await.unwrap();

        // Assert status parsing
        assert_eq!(response.status, 202);
        assert_eq!(response.status_text, "Accepted");

        // Assert headers parsing
        assert!(response.headers.len() >= 3); // at least: content-type, x-custom-header, cache-control
        assert_eq!(response.headers.get("x-custom-header").unwrap(), "custom-value");
        assert_eq!(response.headers.get("cache-control").unwrap(), "no-cache");

        // Assert body parsing
        assert!(!response.body.is_empty());
        assert_eq!(response.size_bytes, response.body.len());
        assert!(response.body_text.is_some());

        // Verify JSON body content
        let body_text = response.body_text.as_ref().unwrap();
        assert!(body_text.contains("items"));
        assert!(body_text.contains("count"));

        // Assert timing
        assert!(response.elapsed >= Duration::ZERO);

        // Test pretty_body method
        let pretty = response.pretty_body();
        assert!(pretty.is_some());
        let pretty_str = pretty.unwrap();
        assert!(pretty_str.contains("items"));
    }

    #[tokio::test]
    async fn test_error_handling_404_not_found() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock for 404 response
        Mock::given(method("GET"))
            .and(path("/api/nonexistent"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({
                        "error": "Not Found",
                        "message": "The requested resource was not found"
                    }))
            )
            .mount(&mock_server)
            .await;

        // Create HTTP engine and request
        let engine = HttpEngine::new();
        let url = format!("{}/api/nonexistent", mock_server.uri());
        let request = create_test_request(HttpMethod::GET, &url);

        // Execute request - should succeed but return 404
        let response = engine.execute(&request).await.unwrap();

        // Assert error status
        assert_eq!(response.status, 404);
        assert_eq!(response.status_text, "Not Found");
        assert!(!response.is_success());
        assert!(response.body_text.is_some());
        let body_text = response.body_text.as_ref().unwrap();
        assert!(body_text.contains("Not Found"));
    }

    #[tokio::test]
    async fn test_error_handling_500_internal_server_error() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock for 500 response
        Mock::given(method("POST"))
            .and(path("/api/error"))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_string("Internal Server Error")
            )
            .mount(&mock_server)
            .await;

        // Create HTTP engine and request
        let engine = HttpEngine::new();
        let url = format!("{}/api/error", mock_server.uri());
        let request = create_test_request(HttpMethod::POST, &url);

        // Execute request - should succeed but return 500
        let response = engine.execute(&request).await.unwrap();

        // Assert error status
        assert_eq!(response.status, 500);
        assert_eq!(response.status_text, "Internal Server Error");
        assert!(!response.is_success());
    }

    #[tokio::test]
    async fn test_error_handling_connection_refused() {
        // Create HTTP engine
        let engine = HttpEngine::new();

        // Use an address that should refuse connection (non-existent server)
        let request = create_test_request(HttpMethod::GET, "http://localhost:59999/api/test");

        // Execute request - should return an error
        let result = engine.execute(&request).await;

        // Assert error
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error, crate::http::client::HttpError::Reqwest(_)));
    }

    #[tokio::test]
    async fn test_error_handling_invalid_url() {
        // Create HTTP engine
        let engine = HttpEngine::new();

        // Use an invalid URL
        let mut request = create_test_request(HttpMethod::GET, "");
        request.url = "not a valid url".to_string();

        // Execute request - should return an error
        let result = engine.execute(&request).await;

        // Assert error - could be URL parse error or request error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_disabled_query_params_are_not_sent() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock - should NOT receive the disabled parameter
        Mock::given(method("GET"))
            .and(path("/api/filter"))
            .and(query_param("active", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "filtered": true
            })))
            .mount(&mock_server)
            .await;

        // Create HTTP engine and request with mixed enabled/disabled params
        let engine = HttpEngine::new();
        let url = format!("{}/api/filter", mock_server.uri());
        let mut request = create_test_request(HttpMethod::GET, &url);

        // Add enabled param
        request.query_params.push(KeyValuePair {
            key: "active".to_string(),
            value: "true".to_string(),
            enabled: true,
            description: None,
        });

        // Add disabled param (should not be sent)
        request.query_params.push(KeyValuePair {
            key: "disabled_param".to_string(),
            value: "should_not_appear".to_string(),
            enabled: false,
            description: None,
        });

        // Execute request - should succeed because disabled param is not sent
        let response = engine.execute(&request).await.unwrap();
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn test_disabled_headers_are_not_sent() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock - should only receive enabled headers
        Mock::given(method("GET"))
            .and(path("/api/headers"))
            .and(header("x-enabled-header", "enabled"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "headers_ok": true
            })))
            .mount(&mock_server)
            .await;

        // Create HTTP engine and request with mixed enabled/disabled headers
        let engine = HttpEngine::new();
        let url = format!("{}/api/headers", mock_server.uri());
        let mut request = create_test_request(HttpMethod::GET, &url);

        // Add enabled header
        request.headers.push(KeyValuePair {
            key: "x-enabled-header".to_string(),
            value: "enabled".to_string(),
            enabled: true,
            description: None,
        });

        // Add disabled header (should not be sent)
        request.headers.push(KeyValuePair {
            key: "x-disabled-header".to_string(),
            value: "should_not_appear".to_string(),
            enabled: false,
            description: None,
        });

        // Execute request - should succeed because disabled header is not sent
        let response = engine.execute(&request).await.unwrap();
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn test_post_xml_body() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock for XML request
        Mock::given(method("POST"))
            .and(path("/api/xml"))
            .and(header("content-type", "application/xml"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<?xml version=\"1.0\" encoding=\"UTF-8\"?><response><status>ok</status></response>")
            )
            .mount(&mock_server)
            .await;

        // Create HTTP engine and request with XML body
        let engine = HttpEngine::new();
        let url = format!("{}/api/xml", mock_server.uri());
        let mut request = create_test_request(HttpMethod::POST, &url);
        request.body = BodyType::Raw {
            content: "<?xml version=\"1.0\" encoding=\"UTF-8\"?><request><data>test</data></request>".to_string(),
            content_type: RawContentType::Xml,
        };

        // Execute request
        let response = engine.execute(&request).await.unwrap();

        // Assert response
        assert_eq!(response.status, 200);
        assert!(response.body_text.is_some());
        let body_text = response.body_text.as_ref().unwrap();
        assert!(body_text.contains("<?xml"));
        assert!(body_text.contains("<status>ok</status>"));
    }

    #[tokio::test]
    async fn test_empty_response_body() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock with empty body (e.g., 204 No Content)
        Mock::given(method("DELETE"))
            .and(path("/api/resource/123"))
            .respond_with(ResponseTemplate::new(204).set_body_string(""))
            .mount(&mock_server)
            .await;

        // Create HTTP engine and request
        let engine = HttpEngine::new();
        let url = format!("{}/api/resource/123", mock_server.uri());
        let request = create_test_request(HttpMethod::DELETE, &url);

        // Execute request
        let response = engine.execute(&request).await.unwrap();

        // Assert empty response
        assert_eq!(response.status, 204);
        assert_eq!(response.status_text, "No Content");
        assert!(response.is_success()); // 2xx range
        assert_eq!(response.size_bytes, 0);
        assert!(response.body.is_empty());
        assert_eq!(response.body_text, Some("".to_string()));
    }

    #[tokio::test]
    async fn test_binary_response_body() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Set up mock with binary response
        let binary_data: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD];
        Mock::given(method("GET"))
            .and(path("/api/binary"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(binary_data.clone())
                    .insert_header("content-type", "application/octet-stream")
            )
            .mount(&mock_server)
            .await;

        // Create HTTP engine and request
        let engine = HttpEngine::new();
        let url = format!("{}/api/binary", mock_server.uri());
        let request = create_test_request(HttpMethod::GET, &url);

        // Execute request
        let response = engine.execute(&request).await.unwrap();

        // Assert binary response
        assert_eq!(response.status, 200);
        assert_eq!(response.body, binary_data);
        assert_eq!(response.size_bytes, binary_data.len());
        // body_text should be None for non-UTF8 data
        assert!(response.body_text.is_none());
    }
}
