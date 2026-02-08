//! Request editor component - main editor for HTTP requests.
//!
//! Provides the main editing interface for HTTP requests including:
//! - Method selector dropdown and URL input with Send button
//! - Sub-tabs for Params/Headers/Body
//! - Integration with AppState for request execution

use crate::app_state::AppState;
use crate::ui::key_value_editor::{EditorType, KeyValueEditor};
use gpui::{div, px, App, AppContext, Context, Entity, InteractiveElement, IntoElement, MouseButton, ParentElement, Render, Styled, Window};
use gpui_component::{h_flex, v_flex, ActiveTheme, Icon, IconName, button::Button, input::Input};
use reqforge_core::models::request::{HttpMethod, KeyValuePair, BodyType};
use reqforge_core::models::response::HttpResponse;

/// Sub-tabs within the request editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestSubTab {
    /// Query parameters
    Params,
    /// HTTP headers
    Headers,
    /// Request body
    Body,
}

impl RequestSubTab {
    /// Get all sub-tabs in order.
    pub fn all() -> &'static [RequestSubTab] {
        &[RequestSubTab::Params, RequestSubTab::Headers, RequestSubTab::Body]
    }

    /// Get the display name for this sub-tab.
    pub fn display_name(&self) -> &str {
        match self {
            RequestSubTab::Params => "Params",
            RequestSubTab::Headers => "Headers",
            RequestSubTab::Body => "Body",
        }
    }
}

/// HTTP methods available in the dropdown.
const HTTP_METHODS: &[HttpMethod] = &[
    HttpMethod::GET,
    HttpMethod::POST,
    HttpMethod::PUT,
    HttpMethod::PATCH,
    HttpMethod::DELETE,
    HttpMethod::HEAD,
    HttpMethod::OPTIONS,
];

/// Request editor component.
///
/// Renders the main request editing interface with method selector dropdown,
/// URL input, sub-tabs for Params/Headers/Body, and Send button.
pub struct RequestEditor {
    /// The application state entity
    app_state: Entity<AppState>,
    /// Current sub-tab
    active_sub_tab: RequestSubTab,
    /// Selected HTTP method
    selected_method: HttpMethod,
    /// Whether the method dropdown is open
    method_dropdown_open: bool,
    /// Key-value editor for query parameters
    params_editor: Option<Entity<KeyValueEditor>>,
    /// Key-value editor for headers
    headers_editor: Option<Entity<KeyValueEditor>>,
}

impl RequestEditor {
    /// Create a new request editor.
    pub fn new(app_state: Entity<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            active_sub_tab: RequestSubTab::Params,
            selected_method: HttpMethod::GET,
            method_dropdown_open: false,
            params_editor: None,
            headers_editor: None,
        }
    }

    /// Toggle the method dropdown open/closed.
    fn toggle_method_dropdown(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.method_dropdown_open = !self.method_dropdown_open;
        cx.notify();
    }

    /// Select an HTTP method from the dropdown by index.
    fn select_method_by_index(&mut self, index: usize, cx: &mut Context<Self>) {
        // Use a lookup table for HTTP methods
        match index {
            0 => self.selected_method = HttpMethod::GET,
            1 => self.selected_method = HttpMethod::POST,
            2 => self.selected_method = HttpMethod::PUT,
            3 => self.selected_method = HttpMethod::PATCH,
            4 => self.selected_method = HttpMethod::DELETE,
            5 => self.selected_method = HttpMethod::HEAD,
            6 => self.selected_method = HttpMethod::OPTIONS,
            _ => {}
        }
        self.method_dropdown_open = false;
        cx.notify();
    }

    /// Switch to a specific sub-tab.
    fn switch_sub_tab(&mut self, tab: RequestSubTab, cx: &mut Context<Self>) {
        self.active_sub_tab = tab;
        cx.notify();
    }

    /// Render the method selector dropdown button.
    fn render_method_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let method_name = self.selected_method.to_string();
        let chevron_icon = if self.method_dropdown_open {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        let mut button = div()
            .id("method-selector")
            .min_w(px(80.0))
            .h(px(32.0))
            .px_2()
            .py_1()
            .rounded(px(4.0))
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .cursor_pointer();

        if self.method_dropdown_open {
            button = button.border_color(cx.theme().primary);
        }

        button.child(
            h_flex()
                .gap_1()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .child(method_name),
                )
                .child(
                    Icon::new(chevron_icon)
                        .size(px(12.0))
                        .text_color(cx.theme().muted_foreground),
                ),
        )
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|this, _, window, cx| this.toggle_method_dropdown(window, cx)),
        )
    }

    /// Render the method dropdown menu.
    fn render_method_dropdown(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_index = HTTP_METHODS.iter().position(|m| m == &self.selected_method);

        let items: Vec<_> = HTTP_METHODS
            .iter()
            .enumerate()
            .map(|(index, method)| {
                let is_selected = selected_index == Some(index);
                let method_name = method.to_string();

                let mut item = div()
                    .h(px(32.0))
                    .px_3()
                    .rounded_md()
                    .flex()
                    .items_center()
                    .cursor_pointer();

                if is_selected {
                    item = item.bg(cx.theme().muted);
                }

                item.child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(gpui::FontWeight::BOLD)
                                .child(method_name),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().primary)
                                .child(if is_selected { "✓" } else { "" }),
                        ),
                )
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _window, cx| {
                        this.select_method_by_index(index, cx);
                    }),
                )
            })
            .collect();

        div()
            .absolute()
            .top(px(36.0))
            .left(px(0.0))
            .min_w(px(80.0))
            .max_w(px(120.0))
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border)
            .rounded_md()
            .child(
                v_flex()
                    .gap_1()
                    .p_1()
                    .children(items),
            )
    }

    /// Render the URL input field.
    fn render_url_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let (display_text, is_empty) = self
            .app_state
            .read(cx)
            .active_tab()
            .map(|tab| {
                let text = tab.url_input.read(cx).text().to_string();
                (if text.is_empty() {
                    "https://example.com/api/endpoint".to_string()
                } else {
                    text.clone()
                }, text.is_empty())
            })
            .unwrap_or_else(|| ("https://example.com/api/endpoint".to_string(), true));

        let text_div = div().child(display_text.clone());
        let text_div = if is_empty {
            text_div.text_color(cx.theme().muted_foreground)
        } else {
            text_div
        };

        div()
            .id("url-input-wrapper")
            .flex_1()
            .h(px(32.0))
            .px_3()
            .py_1()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .child(text_div)
    }

    /// Render the Send button.
    fn render_send_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_loading = self
            .app_state
            .read(cx)
            .active_tab()
            .map(|tab| tab.is_loading)
            .unwrap_or(false);

        let button_text = if is_loading { "Sending..." } else { "Send" };

        if is_loading {
            div()
                .min_w(px(80.0))
                .h(px(32.0))
                .px_4()
                .py_1()
                .rounded(px(4.0))
                .bg(gpui::Hsla { h: 0.0, s: 0.0, l: 0.4, a: 1.0 })
                .text_color(cx.theme().primary_foreground)
                .font_weight(gpui::FontWeight::BOLD)
                .items_center()
                .justify_center()
                .child(button_text)
        } else {
            div().child(
                Button::new("send-request")
                    .label(button_text)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.on_send(window, cx);
                    })),
            )
        }
    }

    /// Render the sub-tab bar.
    fn render_sub_tab_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let active_tab = self.active_sub_tab;

        let tabs: Vec<_> = RequestSubTab::all().iter().map(|&tab| {
            let is_active = active_tab == tab;
            let display_name = tab.display_name().to_string();

            let tab_div = div()
                .px_3()
                .py_1()
                .rounded_md()
                .cursor_pointer()
                .child(display_name);

            if is_active {
                tab_div.bg(cx.theme().muted)
            } else {
                tab_div
            }
        }).collect();

        h_flex()
            .id("sub-tab-bar")
            .gap_2()
            .p_2()
            .border_b_1()
            .border_color(cx.theme().border)
            .children(tabs)
    }

    /// Render the Params sub-tab content.
    fn render_params_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let params = self
            .app_state
            .read(cx)
            .active_tab()
            .map(|tab| tab.params.clone())
            .unwrap_or_default();

        div()
            .id("params-tab")
            .flex_1()
            .p_4()
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(cx.theme().muted_foreground)
                            .child("Query Parameters"),
                    )
                    .child(self.render_key_value_list(&params, "Parameter name", "Parameter value", cx))
                    .child(
                        h_flex()
                            .p_2()
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .cursor_pointer()
                                    .child("Add Parameter"),
                            ),
                    ),
            )
    }

    /// Render the Headers sub-tab content.
    fn render_headers_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let headers = self
            .app_state
            .read(cx)
            .active_tab()
            .map(|tab| tab.headers.clone())
            .unwrap_or_default();

        div()
            .id("headers-tab")
            .flex_1()
            .p_4()
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(cx.theme().muted_foreground)
                            .child("Headers"),
                    )
                    .child(self.render_key_value_list(&headers, "Header name", "Header value", cx))
                    .child(
                        h_flex()
                            .p_2()
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .cursor_pointer()
                                    .child("Add Header"),
                            ),
                    ),
            )
    }

    /// Render the Body sub-tab content.
    fn render_body_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let (body_content, content_type) = self
            .app_state
            .read(cx)
            .active_tab()
            .map(|tab| {
                let text = tab.body_input.read(cx).text().to_string();
                (text.clone(), if text.is_empty() { "None" } else { "Raw" }.to_string())
            })
            .unwrap_or_else(|| (String::new(), "None".to_string()));

        let display_body = if body_content.is_empty() {
            "Request body content...".to_string()
        } else {
            body_content.clone()
        };

        let content_div = div().child(display_body);
        let content_div = if body_content.is_empty() {
            content_div.text_color(cx.theme().muted_foreground)
        } else {
            content_div
        };

        div()
            .id("body-tab")
            .flex_1()
            .p_4()
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(cx.theme().muted_foreground)
                            .child("Request Body"),
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .border_1()
                            .border_color(cx.theme().border)
                            .child(content_type),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_h(px(200.0))
                            .p_3()
                            .rounded_md()
                            .border_1()
                            .border_color(cx.theme().border)
                            .font_family("Monospace")
                            .text_sm()
                            .child(content_div),
                    ),
            )
    }

    /// Render a key-value list (for params/headers).
    fn render_key_value_list(
        &self,
        rows: &[crate::app_state::KeyValueRow],
        key_placeholder: &str,
        value_placeholder: &str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        if rows.is_empty() {
            return div()
                .p_4()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(format!("No {} yet. Click below to add one.", if key_placeholder.contains("Parameter") { "parameters" } else { "headers" }));
        }

        let row_divs: Vec<_> = rows
            .iter()
            .map(|row| {
                let enabled = row.enabled;
                let key = row.key_input.read(cx).text().to_string();
                let value = row.value_input.read(cx).text().to_string();

                let key_display = if key.is_empty() {
                    key_placeholder.to_string()
                } else {
                    key.clone()
                };
                let value_display = if value.is_empty() {
                    value_placeholder.to_string()
                } else {
                    value.clone()
                };

                let key_div = div().child(key_display.clone());
                let key_div = if key.is_empty() {
                    key_div.text_color(cx.theme().muted_foreground)
                } else {
                    key_div
                };

                let value_div = div().child(value_display.clone());
                let value_div = if value.is_empty() {
                    value_div.text_color(cx.theme().muted_foreground)
                } else {
                    value_div
                };

                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .w(px(24.0))
                            .h(px(28.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(if enabled {
                                cx.theme().primary
                            } else {
                                cx.theme().muted_foreground
                            })
                            .child(if enabled { "☑" } else { "☐" }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(150.0))
                            .h(px(28.0))
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .border_1()
                            .border_color(cx.theme().border)
                            .child(key_div),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(200.0))
                            .h(px(28.0))
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .border_1()
                            .border_color(cx.theme().border)
                            .child(value_div),
                    )
                    .child(
                        div()
                            .w(px(24.0))
                            .h(px(28.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .text_color(cx.theme().red)
                            .child("×"),
                    )
            })
            .collect();

        div()
            .flex()
            .flex_col()
            .gap_1()
            .children(row_divs)
    }

    /// Handle the Send button click - execute the HTTP request asynchronously.
    ///
    /// This method:
    /// 1. Gets the active tab and validates URL is present
    /// 2. Sets is_loading = true and triggers re-render
    /// 3. Builds the request from the tab state
    /// 4. Spawns an async task using cx.spawn() for execution
    /// 5. On completion: updates last_response, sets is_loading = false
    /// 6. Handles errors gracefully by displaying them in the response viewer
    fn on_send(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        // Get the active tab's draft request to validate URL is present
        let app_state = self.app_state.clone();

        // Read the tab state to get the URL for validation
        let url_opt = app_state.read(cx).active_tab().map(|tab| {
            tab.url_input.read(cx).text().to_string()
        });

        let url = match url_opt {
            Some(u) => u,
            None => {
                log::error!("Cannot send request: no active tab");
                return;
            }
        };

        // Validate that URL is not empty
        if url.trim().is_empty() {
            let error_response = HttpResponse {
                status: 0,
                status_text: "Invalid Request".to_string(),
                headers: std::collections::HashMap::new(),
                body: bytes::Bytes::from("Error: URL cannot be empty. Please enter a valid URL."),
                size_bytes: 0,
                elapsed: std::time::Duration::ZERO,
            };

            app_state.update(cx, |app, cx| {
                if let Some(tab) = app.active_tab_mut() {
                    tab.last_response = Some(error_response);
                    tab.is_loading = false;
                }
                cx.notify();
            });
            return;
        }

        // Set is_loading = true and trigger re-render
        app_state.update(cx, |app, cx| {
            if let Some(tab) = app.active_tab_mut() {
                tab.is_loading = true;
            }
            cx.notify();
        });

        // Clone necessary references for the async task
        let core = app_state.read(cx).core.clone();
        let app_state = app_state.clone();

        // Build the request BEFORE spawning the async task
        let request = {
            let app_state = app_state.read(cx);
            let tab = match app_state.active_tab() {
                Some(t) => t,
                None => {
                    log::error!("No active tab");
                    return;
                }
            };

            let url = tab.url_input.read(cx).text().to_string();
            let body_content = tab.body_input.read(cx).text().to_string();
            let headers: Vec<KeyValuePair> = tab.headers.iter().map(|row| {
                KeyValuePair {
                    key: row.key_input.read(cx).text().to_string(),
                    value: row.value_input.read(cx).text().to_string(),
                    enabled: row.enabled,
                    description: None,
                }
            }).collect();
            let query_params: Vec<KeyValuePair> = tab.params.iter().map(|row| {
                KeyValuePair {
                    key: row.key_input.read(cx).text().to_string(),
                    value: row.value_input.read(cx).text().to_string(),
                    enabled: row.enabled,
                    description: None,
                }
            }).collect();

            let body = if body_content.is_empty() {
                BodyType::None
            } else {
                BodyType::Raw {
                    content: body_content,
                    content_type: reqforge_core::models::request::RawContentType::Json,
                }
            };

            reqforge_core::models::request::RequestDefinition {
                id: tab.request_id,
                name: tab.name.clone(),
                method: tab.method.clone(),
                url,
                headers,
                query_params,
                body,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }
        };

        // Use the executor directly to spawn the async task
        let async_cx = cx.to_async();
        async_cx.spawn(async move |cx| {
            // Execute the request using the core
            let result = core.execute_request(&request).await;

            // Update the app state with the response or error
            app_state.update(cx, |app, cx| {
                if let Some(tab) = app.active_tab_mut() {
                    match result {
                        Ok(response) => {
                            tab.last_response = Some(response);
                        }
                        Err(error) => {
                            let error_body = format!("Request failed: {}", error);
                            tab.last_response = Some(HttpResponse {
                                status: 0,
                                status_text: "Error".to_string(),
                                headers: std::collections::HashMap::new(),
                                body: bytes::Bytes::from(error_body),
                                size_bytes: 0,
                                elapsed: std::time::Duration::ZERO,
                            });
                        }
                    }
                    tab.is_loading = false;
                }
                cx.notify();
            });
        })
        .detach();
    }
}

impl Render for RequestEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Get the current method from the active tab before rendering
        let current_method = {
            let app_state = self.app_state.read(cx);
            app_state.active_tab().map(|tab| tab.method.clone())
        };

        // Update the selected method if there's an active tab
        if let Some(method) = current_method {
            self.selected_method = method;
        }

        // Get data for rendering
        let (url_text, url_is_empty) = {
            let app_state = self.app_state.read(cx);
            app_state.active_tab().map(|tab| {
                let text = tab.url_input.read(cx).text().to_string();
                (if text.is_empty() {
                    "https://example.com/api/endpoint".to_string()
                } else {
                    text.clone()
                }, text.is_empty())
            }).unwrap_or_else(|| ("https://example.com/api/endpoint".to_string(), true))
        };

        let (is_loading, _params, _headers, body_content, body_content_type) = {
            let app_state = self.app_state.read(cx);
            let loading = app_state.active_tab().map(|tab| tab.is_loading).unwrap_or(false);
            let p = app_state.active_tab().map(|tab| tab.params.clone()).unwrap_or_default();
            let h = app_state.active_tab().map(|tab| tab.headers.clone()).unwrap_or_default();
            let (bc, bt) = app_state.active_tab().map(|tab| {
                let text = tab.body_input.read(cx).text().to_string();
                (text.clone(), if text.is_empty() { "None" } else { "Raw" }.to_string())
            }).unwrap_or_else(|| (String::new(), "None".to_string()));
            (loading, p, h, bc, bt)
        };

        // Create or update KeyValueEditor entities for params and headers
        // Note: For now, we'll use a simplified approach and create placeholder editors
        // In a full implementation, these would be cached and updated rather than recreated

        // Extract the count of params/headers to display
        let params_count = {
            let app_state = self.app_state.read(cx);
            app_state.active_tab()
                .map(|tab| tab.params.len())
                .unwrap_or(0)
        };

        let headers_count = {
            let app_state = self.app_state.read(cx);
            app_state.active_tab()
                .map(|tab| tab.headers.len())
                .unwrap_or(0)
        };

        // Create simple placeholder editors for now
        // TODO: Implement proper KeyValueEditor integration with entity caching
        let params_editor_entity = cx.new(|_cx| KeyValueEditor::new(EditorType::Params));
        let headers_editor_entity = cx.new(|_cx| KeyValueEditor::new(EditorType::Headers));

        // Build the method selector
        let method_name = self.selected_method.to_string();
        let chevron_icon = if self.method_dropdown_open {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        let method_selector = div()
            .id("method-selector")
            .min_w(px(80.0))
            .h(px(32.0))
            .px_2()
            .py_1()
            .rounded(px(4.0))
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .cursor_pointer()
            .child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(method_name),
                    )
                    .child(
                        Icon::new(chevron_icon)
                            .size(px(12.0))
                            .text_color(cx.theme().muted_foreground),
                    ),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.toggle_method_dropdown(window, cx)),
            );

        // Build URL input
        let text_div = div().child(url_text.clone());
        let text_div = if url_is_empty {
            text_div.text_color(cx.theme().muted_foreground)
        } else {
            text_div
        };

        let url_input = div()
            .id("url-input-wrapper")
            .flex_1()
            .h(px(32.0))
            .px_3()
            .py_1()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .child(text_div);

        // Build send button
        let button_text = if is_loading { "Sending..." } else { "Send" };
        let send_button = if is_loading {
            div()
                .min_w(px(80.0))
                .h(px(32.0))
                .px_4()
                .py_1()
                .rounded(px(4.0))
                .bg(gpui::Hsla { h: 0.0, s: 0.0, l: 0.4, a: 1.0 })
                .text_color(cx.theme().primary_foreground)
                .font_weight(gpui::FontWeight::BOLD)
                .items_center()
                .justify_center()
                .child(button_text)
        } else {
            div().child(
                Button::new("send-request")
                    .label(button_text)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.on_send(window, cx);
                    })),
            )
        };

        // Build sub-tab bar
        let active_tab = self.active_sub_tab;
        let sub_tabs: Vec<_> = RequestSubTab::all().iter().map(|&tab| {
            let is_active = active_tab == tab;
            let display_name = tab.display_name().to_string();

            let tab_div = div()
                .px_3()
                .py_1()
                .rounded_md()
                .cursor_pointer()
                .child(display_name);

            if is_active {
                tab_div.bg(cx.theme().muted)
            } else {
                tab_div
            }
        }).collect();

        // Build params content
        let params_tab = div()
            .id("params-tab")
            .flex_1()
            .child(params_editor_entity.clone());

        // Build headers content
        let headers_tab = div()
            .id("headers-tab")
            .flex_1()
            .child(headers_editor_entity.clone());

        // Build body content
        let body_input_state = self.app_state.read(cx).active_tab().map(|tab| tab.body_input.clone());

        let body_input = if let Some(input_state) = body_input_state {
            div()
                .flex_1()
                .min_h(px(200.0))
                .child(Input::new(&input_state))
        } else {
            div()
                .flex_1()
                .min_h(px(200.0))
                .p_3()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().border)
                .font_family("Monospace")
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child("No body input available")
        };

        let body_tab = div()
            .id("body-tab")
            .flex_1()
            .p_4()
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(cx.theme().muted_foreground)
                            .child("Request Body"),
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .border_1()
                            .border_color(cx.theme().border)
                            .child(body_content_type),
                    )
                    .child(body_input),
            );

        // Build method dropdown
        let method_dropdown = if self.method_dropdown_open {
            let selected_index = {
                let mut idx = 0;
                for (i, m) in HTTP_METHODS.iter().enumerate() {
                    if m == &self.selected_method {
                        idx = i;
                        break;
                    }
                }
                idx
            };

            let items: Vec<_> = HTTP_METHODS
                .iter()
                .enumerate()
                .map(|(index, method)| {
                    let is_selected = selected_index == index;
                    let method_name = method.to_string();

                    let mut item = div()
                        .h(px(32.0))
                        .px_3()
                        .rounded_md()
                        .flex()
                        .items_center()
                        .cursor_pointer();

                    if is_selected {
                        item = item.bg(cx.theme().muted);
                    }

                    item.child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child(method_name),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().primary)
                                    .child(if is_selected { "✓" } else { "" }),
                            ),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _window, cx| {
                            this.select_method_by_index(index, cx);
                        }),
                    )
                })
                .collect();

            Some(div()
                .absolute()
                .top(px(36.0))
                .left(px(0.0))
                .min_w(px(80.0))
                .max_w(px(120.0))
                .bg(cx.theme().background)
                .border_1()
                .border_color(cx.theme().border)
                .rounded_md()
                .child(
                    v_flex()
                        .gap_1()
                        .p_1()
                        .children(items),
                ))
        } else {
            None
        };

        // Build the final UI
        let sub_tab_content = match self.active_sub_tab {
            RequestSubTab::Params => div().child(params_tab),
            RequestSubTab::Headers => div().child(headers_tab),
            RequestSubTab::Body => div().child(body_tab),
        };

        // Build the result
        let mut result = v_flex()
            .id("request-editor")
            .flex_1()
            .h_full()
            .bg(cx.theme().background)
            .border_r_1()
            .border_color(cx.theme().border)
            // URL bar row
            .child(
                div()
                    .relative()
                    .p_2()
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(method_selector)
                            .child(url_input)
                            .child(send_button),
                    ),
            )
            // Sub-tab bar
            .child(
                h_flex()
                    .id("sub-tab-bar")
                    .gap_2()
                    .p_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .children(sub_tabs),
            )
            // Sub-tab content
            .child(sub_tab_content);

        // Add method dropdown as an overlay if open
        if let Some(dropdown) = method_dropdown {
            result = result.child(dropdown);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqforge_core::ReqForgeCore;

    #[test]
    fn test_sub_tab_display_names() {
        assert_eq!(RequestSubTab::Params.display_name(), "Params");
        assert_eq!(RequestSubTab::Headers.display_name(), "Headers");
        assert_eq!(RequestSubTab::Body.display_name(), "Body");
    }
}
