//! Response viewer component - displays HTTP response data.
//!
//! This is a stub implementation that demonstrates the component structure
//! without requiring GPUI rendering.

use reqforge_core::models::request::HttpMethod;
use reqforge_core::models::response::HttpResponse;
use serde_json::Value as JsonValue;
use std::time::Duration;

/// Sub-tabs within the response viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseSubTab {
    /// Response body
    Body,
    /// Response headers
    Headers,
    /// Response cookies
    Cookies,
    /// Timing information
    Timing,
    /// Request metadata
    Request,
}

impl ResponseSubTab {
    /// Get all sub-tabs in order.
    pub fn all() -> &'static [ResponseSubTab] {
        &[ResponseSubTab::Body, ResponseSubTab::Headers, ResponseSubTab::Cookies, ResponseSubTab::Timing, ResponseSubTab::Request]
    }

    /// Get the display name for this sub-tab.
    pub fn display_name(&self) -> &str {
        match self {
            ResponseSubTab::Body => "Body",
            ResponseSubTab::Headers => "Headers",
            ResponseSubTab::Cookies => "Cookies",
            ResponseSubTab::Timing => "Timing",
            ResponseSubTab::Request => "Request",
        }
    }
}

/// HTTP response metadata.
#[derive(Debug, Clone)]
pub struct ResponseMetadata {
    /// HTTP status code
    pub status_code: u16,
    /// Status reason phrase
    pub status_text: String,
    /// Response time
    pub response_time: Duration,
    /// Response size in bytes
    pub size: u64,
    /// Response content type
    pub content_type: Option<String>,
}

impl ResponseMetadata {
    /// Create a new response metadata.
    pub fn new(status_code: u16, status_text: String) -> Self {
        Self {
            status_code,
            status_text,
            response_time: Duration::from_millis(0),
            size: 0,
            content_type: None,
        }
    }

    /// Get the status category (1xx, 2xx, 3xx, 4xx, 5xx).
    pub fn status_category(&self) -> u16 {
        self.status_code / 100
    }

    /// Check if the response was successful (2xx or 3xx).
    pub fn is_success(&self) -> bool {
        let cat = self.status_category();
        cat == 2 || cat == 3
    }

    /// Format the response time for display.
    pub fn format_time(&self) -> String {
        if self.response_time.as_millis() < 1 {
            format!("{}μs", self.response_time.as_micros())
        } else if self.response_time.as_secs() < 1 {
            format!("{}ms", self.response_time.as_millis())
        } else {
            format!("{:.1}s", self.response_time.as_secs_f64())
        }
    }

    /// Format the size for display.
    pub fn format_size(&self) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;

        if self.size < KB {
            format!("{} B", self.size)
        } else if self.size < MB {
            format!("{:.1} KB", self.size as f64 / KB as f64)
        } else {
            format!("{:.1} MB", self.size as f64 / MB as f64)
        }
    }
}

/// Response viewer component.
///
/// Displays the HTTP response including status, timing, size,
/// and the response body/headers in sub-tabs.
pub struct ResponseViewer {
    /// Response metadata
    pub metadata: ResponseMetadata,
    /// Response body as bytes
    pub body_bytes: Vec<u8>,
    /// Parsed JSON body (if applicable)
    pub json_body: Option<JsonValue>,
    /// Response headers
    pub headers: Vec<(String, String)>,
    /// Current sub-tab
    pub active_sub_tab: ResponseSubTab,
    /// Whether a response is currently loaded
    pub has_response: bool,
    /// Original request details
    pub request_method: HttpMethod,
    pub request_url: String,
}

impl ResponseViewer {
    /// Create a new response viewer (empty state).
    pub fn new() -> Self {
        Self {
            metadata: ResponseMetadata::new(0, "No Response".to_string()),
            body_bytes: Vec::new(),
            json_body: None,
            headers: Vec::new(),
            active_sub_tab: ResponseSubTab::Body,
            has_response: false,
            request_method: HttpMethod::GET,
            request_url: String::new(),
        }
    }

    /// Load a response into the viewer.
    pub fn load_response(
        &mut self,
        status_code: u16,
        status_text: String,
        headers: Vec<(String, String)>,
        body: Vec<u8>,
        response_time: Duration,
        method: HttpMethod,
        url: String,
    ) {
        self.metadata = ResponseMetadata {
            status_code,
            status_text,
            response_time,
            size: body.len() as u64,
            content_type: headers
                .iter()
                .find(|(k, _)| k.eq_ignore_ascii_case("content-type"))
                .map(|(_, v)| v.clone()),
        };

        self.headers = headers;
        self.body_bytes = body;
        self.json_body = self.try_parse_json();
        self.has_response = true;
        self.request_method = method;
        self.request_url = url;
    }

    /// Try to parse the body as JSON.
    fn try_parse_json(&self) -> Option<JsonValue> {
        if self.metadata.content_type.as_deref().map_or(false, |ct| {
            ct.contains("application/json") || ct.contains("text/json")
        }) {
            serde_json::from_slice(&self.body_bytes).ok()
        } else {
            None
        }
    }

    /// Clear the response.
    pub fn clear(&mut self) {
        self.metadata = ResponseMetadata::new(0, "No Response".to_string());
        self.body_bytes.clear();
        self.json_body = None;
        self.headers.clear();
        self.has_response = false;
    }

    /// Load a response from core HttpResponse into the viewer.
    pub fn load_from_core_response(&mut self, response: &HttpResponse, method: HttpMethod, url: String) {
        // Convert HashMap headers to Vec<(String, String)>
        let headers: Vec<(String, String)> = response.headers
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        self.load_response(
            response.status,
            response.status_text.clone(),
            headers,
            response.body.clone(),
            response.elapsed,
            method,
            url,
        );
    }

    /// Render the response viewer to console (stub implementation).
    pub fn render(&self) {
        println!();
        println!("┌────────────────────────────────────────────────────────────┐");

        // Status bar
        self.render_status_bar();

        println!("├────────────────────────────────────────────────────────────┤");

        // Sub-tabs
        self.render_sub_tabs();

        println!("├────────────────────────────────────────────────────────────┤");

        // Content
        if self.has_response {
            self.render_content();
        } else {
            self.render_empty_state();
        }

        println!("└────────────────────────────────────────────────────────────┘");
    }

    /// Render the status bar with code, time, and size.
    fn render_status_bar(&self) {
        if !self.has_response {
            println!("│ No Response                                            │");
            return;
        }

        let status_indicator = if self.metadata.is_success() {
            "✓"
        } else {
            "✗"
        };

        let status_text = format!("{} {} {}", status_indicator, self.metadata.status_code, self.metadata.status_text);
        let time_text = format!("Time: {}", self.metadata.format_time());
        let size_text = format!("Size: {}", self.metadata.format_size());

        println!("│ {}  {}  {}                              │",
            status_text, time_text, size_text);
    }

    /// Render the sub-tabs.
    fn render_sub_tabs(&self) {
        print!("│");
        for tab in ResponseSubTab::all() {
            let is_active = *tab == self.active_sub_tab;
            let marker = if is_active { "[" } else { " " };
            let end_marker = if is_active { "]" } else { " " };
            print!(" {}{}{} ", marker, tab.display_name(), end_marker);
        }
        println!("                                             │");
    }

    /// Render the content based on active sub-tab.
    fn render_content(&self) {
        match self.active_sub_tab {
            ResponseSubTab::Body => self.render_body(),
            ResponseSubTab::Headers => self.render_headers(),
            ResponseSubTab::Cookies => self.render_cookies(),
            ResponseSubTab::Timing => self.render_timing(),
            ResponseSubTab::Request => self.render_request(),
        }
    }

    /// Render empty state.
    fn render_empty_state(&self) {
        println!("│                                                       │");
        println!("│   Send a request to see the response here            │");
        println!("│                                                       │");
        println!("│                                                       │");
        println!("│                                                       │");
        println!("│                                                       │");
        println!("│                                                       │");
    }

    /// Render the Body sub-tab.
    fn render_body(&self) {
        println!("│ Response Body:                                         │");
        println!("│ ┌────────────────────────────────────────────────────┐  │");
        println!("│ │ [Pretty] [Raw] [Preview]                          │  │");
        println!("│ ├────────────────────────────────────────────────────┤  │");

        if let Some(json) = &self.json_body {
            // Pretty print JSON (truncated)
            let json_str = serde_json::to_string_pretty(json).unwrap_or_default();
            let lines: Vec<&str> = json_str.lines().take(8).collect();
            for line in lines {
                let truncated = if line.len() > 54 {
                    format!("{}...", &line[..51])
                } else {
                    line.to_string()
                };
                println!("│ │ {:<54} │  │", truncated);
            }
            if json_str.lines().count() > 8 {
                println!("│ │ ... (truncated)                                  │  │");
            }
        } else {
            // Show raw body (truncated)
            let body_str = String::from_utf8_lossy(&self.body_bytes);
            for line in body_str.lines().take(8) {
                let truncated = if line.len() > 54 {
                    format!("{}...", &line[..51])
                } else {
                    line.to_string()
                };
                println!("│ │ {:<54} │  │", truncated);
            }
        }

        println!("│ └────────────────────────────────────────────────────┘  │");
    }

    /// Render the Headers sub-tab.
    fn render_headers(&self) {
        println!("│ Response Headers:                                       │");
        println!("│ ┌────────────────────────────────────────────────────┐  │");
        println!("│ │ Header                    │ Value                  │  │");
        println!("│ ├────────────────────────────────────────────────────┤  │");

        for (key, value) in &self.headers {
            let key_truncated = if key.len() > 26 {
                format!("{}...", &key[..23])
            } else {
                key.clone()
            };
            let value_truncated = if value.len() > 22 {
                format!("{}...", &value[..19])
            } else {
                value.clone()
            };
            println!("│ │ {:<26} │ {:<22} │  │", key_truncated, value_truncated);
        }

        println!("│ └────────────────────────────────────────────────────┘  │");
    }

    /// Render the Cookies sub-tab.
    fn render_cookies(&self) {
        println!("│ Cookies:                                               │");
        println!("│ ┌────────────────────────────────────────────────────┐  │");
        println!("│ │ Name        │ Value               │ Domain         │  │");
        println!("│ ├────────────────────────────────────────────────────┤  │");
        println!("│ │             │                     │                │  │");
        println!("│ │ No cookies in response                               │  │");
        println!("│ └────────────────────────────────────────────────────┘  │");
    }

    /// Render the Timing sub-tab.
    fn render_timing(&self) {
        println!("│ Timing:                                                │");
        println!("│ ┌────────────────────────────────────────────────────┐  │");
        println!("│ │ Total:     {:>40} │  │", self.metadata.format_time());
        println!("│ │                                                     │  │");
        println!("│ │ breakdown:                                          │  │");
        println!("│ │   DNS:      10ms                                    │  │");
        println!("│ │   TCP:      25ms                                    │  │");
        println!("│ │   TLS:      45ms                                    │  │");
        println!("│ │   TTFB:     78ms                                    │  │");
        println!("│ │   Download: 12ms                                    │  │");
        println!("│ └────────────────────────────────────────────────────┘  │");
    }

    /// Render the Request sub-tab.
    fn render_request(&self) {
        println!("│ Request Details:                                       │");
        println!("│ ┌────────────────────────────────────────────────────┐  │");
        println!("│ │ Method:    {:<42} │  │", self.request_method);
        println!("│ │ URL:       {:<42} │  │",
            if self.request_url.len() > 42 {
                format!("{}...", &self.request_url[..39])
            } else {
                self.request_url.clone()
            }
        );
        println!("│ │                                                     │  │");
        println!("│ │ Headers: {}                                         │  │", self.headers.len());
        println!("│ │ Body: {} bytes                                      │  │", self.body_bytes.len());
        println!("│ └────────────────────────────────────────────────────┘  │");
    }

    /// Switch to a different sub-tab.
    pub fn switch_sub_tab(&mut self, tab: ResponseSubTab) {
        self.active_sub_tab = tab;
    }

    /// Get the body as a string.
    pub fn body_as_string(&self) -> String {
        String::from_utf8_lossy(&self.body_bytes).to_string()
    }

    /// Get the body as JSON, if possible.
    pub fn body_as_json(&self) -> Option<&JsonValue> {
        self.json_body.as_ref()
    }

    /// Get a header value by name.
    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v)
    }
}

impl Default for ResponseViewer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewer_creation() {
        let viewer = ResponseViewer::new();
        assert!(!viewer.has_response);
        assert_eq!(viewer.active_sub_tab, ResponseSubTab::Body);
    }

    #[test]
    fn test_load_response() {
        let mut viewer = ResponseViewer::new();

        viewer.load_response(
            200,
            "OK".to_string(),
            vec![("content-type".to_string(), "application/json".to_string())],
            br#"{"key": "value"}"#.to_vec(),
            Duration::from_millis(100),
            HttpMethod::GET,
            "https://example.com".to_string(),
        );

        assert!(viewer.has_response);
        assert_eq!(viewer.metadata.status_code, 200);
        assert!(viewer.json_body.is_some());
    }

    #[test]
    fn test_status_category() {
        let metadata = ResponseMetadata::new(200, "OK".to_string());
        assert_eq!(metadata.status_category(), 2);
        assert!(metadata.is_success());

        let metadata = ResponseMetadata::new(404, "Not Found".to_string());
        assert_eq!(metadata.status_category(), 4);
        assert!(!metadata.is_success());
    }

    #[test]
    fn test_format_time() {
        let mut metadata = ResponseMetadata::new(200, "OK".to_string());
        metadata.response_time = Duration::from_millis(500);
        assert_eq!(metadata.format_time(), "500ms");

        metadata.response_time = Duration::from_secs(2);
        assert_eq!(metadata.format_time(), "2.0s");
    }

    #[test]
    fn test_format_size() {
        let mut metadata = ResponseMetadata::new(200, "OK".to_string());
        metadata.size = 500;
        assert_eq!(metadata.format_size(), "500 B");

        metadata.size = 2048;
        assert_eq!(metadata.format_size(), "2.0 KB");

        metadata.size = 2 * 1024 * 1024;
        assert_eq!(metadata.format_size(), "2.0 MB");
    }

    #[test]
    fn test_get_header() {
        let mut viewer = ResponseViewer::new();

        viewer.load_response(
            200,
            "OK".to_string(),
            vec![
                ("content-type".to_string(), "application/json".to_string()),
                ("X-Custom-Header".to_string(), "custom-value".to_string()),
            ],
            vec![],
            Duration::from_millis(100),
            HttpMethod::GET,
            "https://example.com".to_string(),
        );

        assert_eq!(viewer.get_header("content-type"), Some(&"application/json".to_string()));
        assert_eq!(viewer.get_header("CONTENT-TYPE"), Some(&"application/json".to_string()));
        assert_eq!(viewer.get_header("X-Custom-Header"), Some(&"custom-value".to_string()));
        assert_eq!(viewer.get_header("non-existent"), None);
    }

    #[test]
    fn test_sub_tab_switching() {
        let mut viewer = ResponseViewer::new();

        viewer.switch_sub_tab(ResponseSubTab::Headers);
        assert_eq!(viewer.active_sub_tab, ResponseSubTab::Headers);

        viewer.switch_sub_tab(ResponseSubTab::Timing);
        assert_eq!(viewer.active_sub_tab, ResponseSubTab::Timing);
    }

    #[test]
    fn test_clear() {
        let mut viewer = ResponseViewer::new();

        viewer.load_response(
            200,
            "OK".to_string(),
            vec![],
            vec![1, 2, 3],
            Duration::from_millis(100),
            HttpMethod::GET,
            "https://example.com".to_string(),
        );

        assert!(viewer.has_response);

        viewer.clear();
        assert!(!viewer.has_response);
        assert!(viewer.body_bytes.is_empty());
    }
}
