//! Request editor component - main editor for HTTP requests.
//!
//! Provides the main editing interface for HTTP requests including:
//! - Method selector and URL input with Send button
//! - Sub-tabs for Params/Headers/Body
//! - Integration with AppState for request execution

use crate::app_state::AppState;
use gpui::{div, px, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, FontWeight};
use gpui_component::{h_flex, v_flex, ActiveTheme};
use reqforge_core::models::request::{HttpMethod, KeyValuePair, BodyType};

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

/// Request editor component.
///
/// Renders the main request editing interface with method selector, URL input,
/// sub-tabs for Params/Headers/Body, and Send button.
pub struct RequestEditor {
    /// The application state entity
    app_state: Entity<AppState>,
    /// Current sub-tab
    active_sub_tab: RequestSubTab,
    /// URL input value
    url_input: String,
    /// Selected HTTP method
    selected_method: HttpMethod,
    /// Whether the method dropdown is open
    method_dropdown_open: bool,
}

impl RequestEditor {
    /// Create a new request editor.
    pub fn new(app_state: Entity<AppState>) -> Self {
        Self {
            app_state,
            active_sub_tab: RequestSubTab::Params,
            url_input: String::new(),
            selected_method: HttpMethod::GET,
            method_dropdown_open: false,
        }
    }

    /// Initialize the editor state from the active tab.
    pub fn init_from_active_tab(&mut self, cx: &mut Context<Self>) {
        if let Some(tab) = self.app_state.read(cx).active_tab() {
            self.selected_method = tab.draft.method.clone();
            self.url_input = tab.draft.url.clone();
        }
    }

    /// Render the method selector dropdown.
    fn render_method_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected_method.clone();

        div()
            .min_w(px(80.0))
            .h(px(32.0))
            .px_2()
            .py_1()
            .rounded(px(4.0))
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .child(format!("{:?}", selected))
    }

    /// Render the URL input field (placeholder div).
    fn render_url_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let display_text = if self.url_input.is_empty() {
            "https://example.com/api/endpoint".to_string()
        } else {
            self.url_input.clone()
        };

        let text_div = div().child(display_text);
        if self.url_input.is_empty() {
            text_div.text_color(cx.theme().muted_foreground)
        } else {
            text_div
        }
    }

    /// Render the Send button.
    fn render_send_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_loading = self
            .app_state
            .read(cx)
            .active_tab()
            .map(|tab| tab.is_loading)
            .unwrap_or(false);

        div()
            .min_w(px(80.0))
            .h(px(32.0))
            .px_4()
            .py_1()
            .rounded(px(4.0))
            .bg(cx.theme().primary)
            .text_color(cx.theme().primary_foreground)
            .font_weight(FontWeight::BOLD)
            .items_center()
            .justify_center()
            .child(if is_loading { "Sending..." } else { "Send" })
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
                .rounded(px(4.0))
                .child(display_name);
            if is_active {
                tab_div.bg(cx.theme().muted)
            } else {
                tab_div
            }
        }).collect();

        h_flex()
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
            .map(|tab| tab.draft.query_params.clone())
            .unwrap_or_default();

        div()
            .flex_1()
            .p_4()
            .child(
                div()
                    .text_sm()
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
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(cx.theme().border)
                            .child("Add Parameter"),
                    ),
            )
    }

    /// Render the Headers sub-tab content.
    fn render_headers_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let headers = self
            .app_state
            .read(cx)
            .active_tab()
            .map(|tab| tab.draft.headers.clone())
            .unwrap_or_default();

        div()
            .flex_1()
            .p_4()
            .child(
                div()
                    .text_sm()
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
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(cx.theme().border)
                            .child("Add Header"),
                    ),
            )
    }

    /// Render the Body sub-tab content.
    fn render_body_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let (body_content, content_type) = self
            .app_state
            .read(cx)
            .active_tab()
            .and_then(|tab| match &tab.draft.body {
                BodyType::Raw { content, .. } => {
                    Some((content.clone(), "Raw".to_string()))
                }
                _ => Some((String::new(), "None".to_string())),
            })
            .unwrap_or((String::new(), "None".to_string()));

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
            .flex_1()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child("Request Body"),
            )
            .child(
                div()
                    .min_w(px(150.0))
                    .h(px(32.0))
                    .px_3()
                    .py_1()
                    .rounded(px(4.0))
                    .border_1()
                    .border_color(cx.theme().border)
                    .child(content_type),
            )
            .child(
                div()
                    .flex_1()
                    .min_h(px(200.0))
                    .p_3()
                    .rounded(px(4.0))
                    .border_1()
                    .border_color(cx.theme().border)
                    .child(content_div)
            )
    }

    /// Render a key-value list (for params/headers).
    fn render_key_value_list(
        &self,
        pairs: &[KeyValuePair],
        key_placeholder: &str,
        value_placeholder: &str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let rows: Vec<_> = pairs.iter().enumerate().map(|(_index, pair)| {
            let enabled = pair.enabled;
            let key = pair.key.clone();
            let value = pair.value.clone();

            let key_display = if key.is_empty() { key_placeholder.to_string() } else { key.clone() };
            let value_display = if value.is_empty() { value_placeholder.to_string() } else { value.clone() };

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
                .gap_1()
                .child(
                    div()
                        .w(px(24.0))
                        .h(px(28.0))
                        .items_center()
                        .justify_center()
                        .child(if enabled { "☑" } else { "☐" }),
                )
                .child(
                    div()
                        .flex_1()
                        .h(px(28.0))
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .border_1()
                        .border_color(cx.theme().border)
                        .child(key_div)
                )
                .child(
                    div()
                        .flex_1()
                        .h(px(28.0))
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .border_1()
                        .border_color(cx.theme().border)
                        .child(value_div)
                )
                .child(
                    div()
                        .w(px(24.0))
                        .h(px(28.0))
                        .items_center()
                        .justify_center()
                        .text_color(cx.theme().red)
                        .child("×"),
                )
        }).collect();

        div()
            .flex()
            .flex_col()
            .gap_1()
            .children(rows)
    }
}

impl Render for RequestEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .flex_1()
            .h_full()
            .bg(cx.theme().background)
            .border_r_1()
            .border_color(cx.theme().border)
            // URL bar row
            .child(
                h_flex()
                    .p_2()
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(self.render_method_selector(cx))
                    .child(self.render_url_input(cx))
                    .child(self.render_send_button(cx)),
            )
            // Sub-tab bar
            .child(self.render_sub_tab_bar(cx))
            // Sub-tab content
            .child(match self.active_sub_tab {
                RequestSubTab::Params => div().child(self.render_params_tab(cx)),
                RequestSubTab::Headers => div().child(self.render_headers_tab(cx)),
                RequestSubTab::Body => div().child(self.render_body_tab(cx)),
            })
    }
}
