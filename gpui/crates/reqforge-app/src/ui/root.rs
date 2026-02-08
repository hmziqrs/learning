//! Root view for the ReqForge application.
//!
//! This module defines the main UI layout with sidebar, request editor,
//! response viewer, and environment selector.

use crate::app_state::AppState;
use gpui::{div, px, Context, EventEmitter, Render, Window, IntoElement, Styled, ParentElement, Entity};
use gpui_component::{h_flex, v_flex, ActiveTheme, Icon, IconName};

/// Root view of the ReqForge application.
///
/// Renders the three-panel layout:
/// - Left: Collection/Request tree sidebar
/// - Center: Request editor + Response viewer
/// - Top: Environment selector
pub struct RootView {
    /// The application state entity
    app_state: Entity<AppState>,
}

impl RootView {
    /// Create a new RootView.
    pub fn new(app_state: Entity<AppState>, _cx: &mut Context<Self>) -> Self {
        Self { app_state }
    }

    /// Render the environment selector at the top right.
    fn render_env_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = self.app_state.read(cx);
        let active_env_id = app_state.active_env_id;
        let environments = &app_state.core.environments;

        // Find the active environment name
        let active_env_name = active_env_id
            .and_then(|id| environments.iter().find(|e| e.id == id))
            .map(|e| e.name.clone())
            .unwrap_or_else(|| "No Environment".to_string());

        // Create environment info
        let env_count = environments.len();

        div()
            .h(px(32.0))
            .px_3()
            .py_1()
            .rounded(px(4.0))
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .cursor_pointer()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        Icon::new(IconName::Globe)
                            .size(px(14.0))
                            .text_color(cx.theme().muted_foreground),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().foreground)
                            .child(active_env_name),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(if env_count > 0 {
                                format!("({} env{})", env_count, if env_count == 1 { "" } else { "s" })
                            } else {
                                "No envs".to_string()
                            }),
                    )
                    .child(
                        Icon::new(IconName::ChevronDown)
                            .size(px(12.0))
                            .text_color(cx.theme().muted_foreground),
                    ),
            )
    }

    /// Render the sidebar panel.
    fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = self.app_state.read(cx);

        // Render sidebar with collection info
        let collection_count = app_state.core.collections.len();
        let request_count: usize = app_state.core.collections.iter()
            .map(|c| c.requests.len())
            .sum();

        div()
            .w(px(300.0))
            .h_full()
            .border_r_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().sidebar)
            .child(
                v_flex()
                    .p_4()
                    .gap_4()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(cx.theme().foreground)
                            .child("Collections"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("{} collections, {} requests", collection_count, request_count)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child("Sidebar Panel - Click collections to expand"),
                    ),
            )
    }

    /// Render the request editor area.
    fn render_request_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = self.app_state.read(cx);

        if let Some(tab) = app_state.active_tab() {
            // Show active request info
            v_flex()
                .flex_1()
                .border_b_1()
                .border_color(cx.theme().border)
                .bg(cx.theme().background)
                .child(
                    h_flex()
                        .p_2()
                        .gap_2()
                        .border_b_1()
                        .border_color(cx.theme().border)
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(cx.theme().muted)
                                .text_sm()
                                .font_weight(gpui::FontWeight::BOLD)
                                .child(format!("{:?}", tab.method)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .px_2()
                                .py_1()
                                .text_sm()
                                .child(tab.url_input.read(cx).text().to_string()),
                        )
                        .child(
                            div()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(cx.theme().primary)
                                .text_color(cx.theme().primary_foreground)
                                .text_sm()
                                .font_weight(gpui::FontWeight::BOLD)
                                .child("Send"),
                        ),
                )
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().muted_foreground)
                                .child("Request Editor - Use gpui-component RequestEditor"),
                        ),
                )
        } else {
            // Empty state
            div()
                .flex_1()
                .border_b_1()
                .border_color(cx.theme().border)
                .bg(cx.theme().background)
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().muted_foreground)
                                .child("No request selected"),
                        ),
                )
        }
    }

    /// Render the response viewer area.
    fn render_response_viewer(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = self.app_state.read(cx);

        if let Some(tab) = app_state.active_tab() {
            if let Some(response) = &tab.last_response {
                // Show response
                v_flex()
                    .flex_1()
                    .bg(cx.theme().background)
                    .child(
                        h_flex()
                            .p_2()
                            .gap_2()
                            .border_b_1()
                            .border_color(cx.theme().border)
                            .items_center()
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded(px(4.0))
                                    .bg(cx.theme().green)
                                    .text_color(gpui::rgb(0xffffff))
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child(format!("{}", response.status)),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(format!("{} · {} bytes", response.status_text, response.size_bytes)),
                            )
                            .child(div().flex_1())
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(format!("{:?}", response.elapsed)),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .p_4()
                            .font_family("Monospace")
                            .text_sm()
                            .child(
                                div().child(format!("Response body ({} bytes)", response.body.len()))
                            ),
                    )
            } else {
                // No response yet
                v_flex()
                    .flex_1()
                    .bg(cx.theme().background)
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("No response yet. Click Send to execute the request."),
                            ),
                    )
            }
        } else {
            // No tab selected
            div()
                .flex_1()
                .bg(cx.theme().background)
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().muted_foreground)
                                .child("No response to display"),
                        ),
                )
        }
    }

    /// Render the main area with request editor and response viewer.
    fn render_main_area(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = self.app_state.read(cx);
        let has_tabs = !app_state.tabs.is_empty();
        drop(app_state);

        // Build the main content
        let main_content = v_flex()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .bg(cx.theme().background)
            .child(
                // Top bar with environment selector
                h_flex()
                    .h(px(40.0))
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .px_2()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(cx.theme().foreground)
                            .child("ReqForge"),
                    )
                    .child(self.render_env_selector(cx)),
            )
            .child(
                // Request editor area
                self.render_request_editor(cx),
            )
            .child(
                // Response viewer area
                self.render_response_viewer(cx),
            );

        // Add empty state overlay when no tabs are open
        if !has_tabs {
            main_content.child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .w_full()
                    .h_full()
                    .bg(cx.theme().background)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        v_flex()
                            .gap_2()
                            .items_center()
                            .child(
                                Icon::new(IconName::FolderOpen)
                                    .size(px(48.0))
                                    .text_color(cx.theme().muted_foreground),
                            )
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .text_color(cx.theme().muted_foreground)
                                    .child("No Request Selected"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("Open a request from the sidebar to get started"),
                            ),
                    ),
            )
        } else {
            main_content
        }
    }
}

impl EventEmitter<()> for RootView {}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(self.render_sidebar(cx))
            .child(self.render_main_area(cx))
    }
}
