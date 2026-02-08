//! Request editor component - main editor for HTTP requests.
//!
//! This is a stub implementation that demonstrates the component structure
//! without requiring GPUI rendering.

use reqforge_core::models::request::{HttpMethod, RequestDefinition};

/// Sub-tabs within the request editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestSubTab {
    /// Query parameters
    Params,
    /// HTTP headers
    Headers,
    /// Request body
    Body,
    /// Authentication
    Auth,
    /// Settings/options
    Settings,
}

impl RequestSubTab {
    /// Get all sub-tabs in order.
    pub fn all() -> &'static [RequestSubTab] {
        &[RequestSubTab::Params, RequestSubTab::Headers, RequestSubTab::Body, RequestSubTab::Auth, RequestSubTab::Settings]
    }

    /// Get the display name for this sub-tab.
    pub fn display_name(&self) -> &str {
        match self {
            RequestSubTab::Params => "Params",
            RequestSubTab::Headers => "Headers",
            RequestSubTab::Body => "Body",
            RequestSubTab::Auth => "Auth",
            RequestSubTab::Settings => "Settings",
        }
    }
}

/// Request editor component.
///
/// Provides the main editing interface for HTTP requests including:
/// - Method selector and URL input
/// - Sub-tabs for Params/Headers/Body/etc.
/// - Send button
pub struct RequestEditor {
    /// Current request being edited
    pub request: RequestDefinition,
    /// Current sub-tab
    pub active_sub_tab: RequestSubTab,
    /// URL input value (may differ from request.url while editing)
    pub url_input: String,
    /// Method dropdown state
    pub method_dropdown_open: bool,
    /// Focused field for UI feedback
    pub focused_field: Option<String>,
}

impl RequestEditor {
    /// Create a new request editor.
    pub fn new(request: RequestDefinition) -> Self {
        let url_input = request.url.clone();
        Self {
            request,
            active_sub_tab: RequestSubTab::Params,
            url_input,
            method_dropdown_open: false,
            focused_field: None,
        }
    }

    /// Render the request editor to console (stub implementation).
    pub fn render(&self) {
        println!();
        println!("┌────────────────────────────────────────────────────────────┐");

        // Method and URL bar
        self.render_method_url_bar();

        println!("├────────────────────────────────────────────────────────────┤");

        // Sub-tabs
        self.render_sub_tabs();

        println!("├────────────────────────────────────────────────────────────┤");

        // Active sub-tab content
        self.render_sub_tab_content();

        println!("├────────────────────────────────────────────────────────────┤");

        // Footer with send button
        self.render_footer();

        println!("└────────────────────────────────────────────────────────────┘");
    }

    /// Render the method dropdown and URL input bar.
    fn render_method_url_bar(&self) {
        print!("│ ");

        // Method selector
        let methods = [HttpMethod::GET, HttpMethod::POST, HttpMethod::PUT, HttpMethod::DELETE, HttpMethod::PATCH];
        print!("[");
        for (i, method) in methods.iter().enumerate() {
            let marker = if *method == self.request.method {
                "●"
            } else {
                " "
            };
            print!("{}{}", marker, method);
            if i < methods.len() - 1 {
                print!(" ");
            }
        }
        print!("]");

        // URL input
        let url_display = if self.url_input.len() > 45 {
            format!("{}...", &self.url_input[..42])
        } else {
            self.url_input.clone()
        };
        print!(" URL: {}", url_display);

        println!("                                       │");
    }

    /// Render the sub-tabs.
    fn render_sub_tabs(&self) {
        print!("│");
        for tab in RequestSubTab::all() {
            let is_active = *tab == self.active_sub_tab;
            let marker = if is_active { "[" } else { " " };
            let end_marker = if is_active { "]" } else { " " };
            print!(" {}{}{} ", marker, tab.display_name(), end_marker);
        }
        println!("                                             │");
    }

    /// Render the content of the active sub-tab.
    fn render_sub_tab_content(&self) {
        match self.active_sub_tab {
            RequestSubTab::Params => {
                self.render_params_tab();
            }
            RequestSubTab::Headers => {
                self.render_headers_tab();
            }
            RequestSubTab::Body => {
                self.render_body_tab();
            }
            RequestSubTab::Auth => {
                self.render_auth_tab();
            }
            RequestSubTab::Settings => {
                self.render_settings_tab();
            }
        }
    }

    /// Render the Params sub-tab.
    fn render_params_tab(&self) {
        println!("│ Query Parameters:                                      │");
        println!("│ ┌────────────────────────────────────────────────────┐  │");
        println!("│ │ Key           │ Value              │              │  │");
        println!("│ ├────────────────────────────────────────────────────┤  │");
        println!("│ │                │                    │ [+] Add      │  │");
        println!("│ └────────────────────────────────────────────────────┘  │");
    }

    /// Render the Headers sub-tab.
    fn render_headers_tab(&self) {
        println!("│ Headers:                                               │");
        println!("│ ┌────────────────────────────────────────────────────┐  │");
        println!("│ │ Key           │ Value              │              │  │");
        println!("│ ├────────────────────────────────────────────────────┤  │");

        // Show common headers
        let common_headers = [
            ("Content-Type", "application/json"),
            ("User-Agent", "ReqForge/1.0"),
            ("Accept", "*/*"),
        ];

        for (key, value) in &common_headers {
            println!("│ │ {:<13} │ {:<18} │              │  │", key, value);
        }

        println!("│ │                │                    │ [+] Add      │  │");
        println!("│ └────────────────────────────────────────────────────┘  │");
    }

    /// Render the Body sub-tab.
    fn render_body_tab(&self) {
        println!("│ Request Body:                                          │");
        println!("│ ┌────────────────────────────────────────────────────┐  │");
        println!("│ │ [JSON] [Form] [Raw] [None]                         │  │");
        println!("│ ├────────────────────────────────────────────────────┤  │");
        println!("│ │ {{                                                   │  │");
        println!("│ │   \"key\": \"value\"                                   │  │");
        println!("│ │ }}                                                   │  │");
        println!("│ │                                                     │  │");
        println!("│ │                                                     │  │");
        println!("│ └────────────────────────────────────────────────────┘  │");
    }

    /// Render the Auth sub-tab.
    fn render_auth_tab(&self) {
        println!("│ Authentication:                                        │");
        println!("│ ┌────────────────────────────────────────────────────┐  │");
        println!("│ │ [No Auth] [Bearer] [Basic] [API Key]              │  │");
        println!("│ ├────────────────────────────────────────────────────┤  │");
        println!("│ │                                                     │  │");
        println!("│ │ No authentication configured                       │  │");
        println!("│ │                                                     │  │");
        println!("│ └────────────────────────────────────────────────────┘  │");
    }

    /// Render the Settings sub-tab.
    fn render_settings_tab(&self) {
        println!("│ Request Settings:                                      │");
        println!("│ ┌────────────────────────────────────────────────────┐  │");
        println!("│ │ Redirects:    [Follow all ▼]                       │  │");
        println!("│ │ Timeout:      [30s               ]                 │  │");
        println!("│ │                                      │              │  │");
        println!("│ │ [☑] Store response                                 │  │");
        println!("│ │ [☑] Enable cookies                                │  │");
        println!("│ └────────────────────────────────────────────────────┘  │");
    }

    /// Render the footer with Send button.
    fn render_footer(&self) {
        println!("│                                                       │");
        println!("│                                          [Send ▶]     │");
        println!("│                                                       │");
    }

    /// Switch to a different sub-tab.
    pub fn switch_sub_tab(&mut self, tab: RequestSubTab) {
        self.active_sub_tab = tab;
    }

    /// Set the HTTP method.
    pub fn set_method(&mut self, method: HttpMethod) {
        self.request.method = method;
        self.method_dropdown_open = false;
    }

    /// Toggle the method dropdown.
    pub fn toggle_method_dropdown(&mut self) {
        self.method_dropdown_open = !self.method_dropdown_open;
    }

    /// Update the URL input.
    pub fn set_url(&mut self, url: String) {
        self.url_input = url;
    }

    /// Apply the URL input to the request.
    pub fn apply_url(&mut self) {
        self.request.url = self.url_input.clone();
    }

    /// Handle Send button click.
    ///
    /// Returns the request definition that should be sent.
    pub fn on_send(&mut self) -> RequestDefinition {
        // Apply the URL input before sending
        self.apply_url();
        println!();
        println!("▶▶▶ Sending Request");
        println!("   Method: {}", self.request.method);
        println!("   URL: {}", self.url_input);
        self.request.clone()
    }

    /// Handle Save action.
    pub fn on_save(&mut self) {
        println!();
        println!("💾 Saving request...");
        println!("   Name: {}", self.request.name);
        println!("   → Would persist to collection");
    }

    /// Handle URL input focus.
    pub fn focus_url(&mut self) {
        self.focused_field = Some("url".to_string());
    }

    /// Handle URL input blur.
    pub fn blur_url(&mut self) {
        self.focused_field = None;
        self.apply_url();
    }

    /// Get the current request definition.
    pub fn get_request(&self) -> &RequestDefinition {
        &self.request
    }

    /// Get a mutable reference to the current request definition.
    pub fn get_request_mut(&mut self) -> &mut RequestDefinition {
        &mut self.request
    }

    /// Update the request definition.
    pub fn set_request(&mut self, request: RequestDefinition) {
        self.request = request;
        self.url_input = self.request.url.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_creation() {
        let request = RequestDefinition::new(
            "Test Request",
            HttpMethod::GET,
            "https://example.com",
        );
        let editor = RequestEditor::new(request);

        assert_eq!(editor.active_sub_tab, RequestSubTab::Params);
        assert_eq!(editor.url_input, "https://example.com");
    }

    #[test]
    fn test_set_method() {
        let request = RequestDefinition::new(
            "Test Request",
            HttpMethod::GET,
            "https://example.com",
        );
        let mut editor = RequestEditor::new(request);

        editor.set_method(HttpMethod::POST);
        assert_eq!(editor.request.method, HttpMethod::POST);
        assert!(!editor.method_dropdown_open);
    }

    #[test]
    fn test_toggle_dropdown() {
        let request = RequestDefinition::new(
            "Test Request",
            HttpMethod::GET,
            "https://example.com",
        );
        let mut editor = RequestEditor::new(request);

        editor.toggle_method_dropdown();
        assert!(editor.method_dropdown_open);

        editor.toggle_method_dropdown();
        assert!(!editor.method_dropdown_open);
    }

    #[test]
    fn test_sub_tab_switching() {
        let request = RequestDefinition::new(
            "Test Request",
            HttpMethod::GET,
            "https://example.com",
        );
        let mut editor = RequestEditor::new(request);

        editor.switch_sub_tab(RequestSubTab::Body);
        assert_eq!(editor.active_sub_tab, RequestSubTab::Body);

        editor.switch_sub_tab(RequestSubTab::Headers);
        assert_eq!(editor.active_sub_tab, RequestSubTab::Headers);
    }
}
