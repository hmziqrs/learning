//! Root view for the ReqForge application.
//!
//! This module defines the main UI layout with sidebar, request editor,
//! response viewer, and environment selector.

use crate::app_state::AppState;
use gpui::{div, px, AppContext, Context, EventEmitter, Render, Window, IntoElement, Styled, ParentElement, Entity};
use gpui_component::{h_flex, v_flex, ActiveTheme, StyledExt};

// Import the real components
use super::{SidebarPanel, RequestTabBar, RequestEditor, ResponseViewer, EnvSelector};

/// Root view of the ReqForge application.
///
/// Renders the Postman-like layout with sidebar, tab bar,
/// request editor, response viewer, and environment selector.
pub struct RootView {
    /// The application state entity
    app_state: Entity<AppState>,
    /// Sidebar panel showing collection tree
    sidebar: Entity<SidebarPanel>,
    /// Tab bar for open requests
    tab_bar: Entity<RequestTabBar>,
    /// Request editor with URL input and sub-tabs
    request_editor: Entity<RequestEditor>,
    /// Response viewer showing HTTP response
    response_viewer: Entity<ResponseViewer>,
    /// Environment selector dropdown
    env_selector: Entity<EnvSelector>,
}

impl RootView {
    /// Create a new RootView with all component entities.
    pub fn new(app_state: Entity<AppState>, cx: &mut Context<Self>) -> Self {
        let sidebar = cx.new(|cx| SidebarPanel::new(app_state.clone(), cx));
        let tab_bar = cx.new(|cx| RequestTabBar::new(app_state.clone(), cx));
        let request_editor = cx.new(|cx| RequestEditor::new(app_state.clone(), cx));
        let response_viewer = cx.new(|_cx| ResponseViewer::new(app_state.clone()));
        let env_selector = cx.new(|cx| EnvSelector::new(app_state.clone(), cx));
        Self {
            app_state,
            sidebar,
            tab_bar,
            request_editor,
            response_viewer,
            env_selector,
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
            // Left: Sidebar (fixed 300px)
            .child(
                div()
                    .w(px(300.0))
                    .h_full()
                    .border_r_1()
                    .border_color(cx.theme().border)
                    .child(self.sidebar.clone())
            )
            // Right: Main area (flex-1)
            .child(
                v_flex()
                    .flex_1()
                    .h_full()
                    // Top bar with env selector
                    .child(
                        h_flex()
                            .h(px(40.0))
                            .px_2()
                            .items_center()
                            .justify_between()
                            .border_b_1()
                            .border_color(cx.theme().border)
                            .child(div().text_sm().font_semibold().child("ReqForge"))
                            .child(self.env_selector.clone())
                    )
                    // Tab bar
                    .child(self.tab_bar.clone())
                    // Request editor (top half)
                    .child(
                        div()
                            .flex_1()
                            .min_h(px(200.0))
                            .child(self.request_editor.clone())
                    )
                    // Response viewer (bottom half)
                    .child(
                        div()
                            .flex_1()
                            .min_h(px(200.0))
                            .child(self.response_viewer.clone())
                    )
            )
    }
}
