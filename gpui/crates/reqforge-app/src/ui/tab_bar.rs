//! Tab bar component - displays open request tabs with GPUI.
//!
//! This module implements the tab bar using gpui-component.
//! It displays all open requests with method badges, names, dirty indicators,
//! and close buttons. Handles tab switching and closing.

use crate::app_state::AppState;
use gpui::{
    App, Context, Entity, InteractiveElement, IntoElement, KeyBinding, MouseButton, ParentElement,
    Render, Styled, Window, actions, div, px,
};
use gpui_component::{ActiveTheme, Icon, IconName, StyledExt, h_flex};
use reqforge_core::models::request::HttpMethod;

/// Context key for tab bar keyboard shortcuts
const TAB_CONTEXT: &str = "RequestTabBar";

/// Actions for tab management
actions!(tab_bar, [CloseTab, NextTab, PreviousTab]);

/// Initialize tab bar keyboard bindings.
pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("cmd-w", CloseTab, Some(TAB_CONTEXT)),
        KeyBinding::new("ctrl-tab", NextTab, Some(TAB_CONTEXT)),
        KeyBinding::new("ctrl-shift-tab", PreviousTab, Some(TAB_CONTEXT)),
    ]);
}

/// Tab bar component that displays open request tabs.
///
/// This component manages the display and interaction of request tabs,
/// including switching between tabs, closing tabs, and displaying dirty
/// indicators and method badges.
pub struct RequestTabBar {
    /// The application state entity
    app_state: Entity<AppState>,
    /// Hovered tab index for close button visibility
    hovered_tab_index: Option<usize>,
    /// Scroll offset for tab overflow
    scroll_offset: f32,
}

impl RequestTabBar {
    /// Create a new RequestTabBar.
    pub fn new(app_state: Entity<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            hovered_tab_index: None,
            scroll_offset: 0.0,
        }
    }

    /// Handle clicking on a tab to switch to it.
    fn on_tab_click(&mut self, tab_index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        let app_state = self.app_state.clone();
        app_state.update(cx, |state, cx| {
            if tab_index < state.tabs.len() {
                state.active_tab = Some(tab_index);
                cx.notify();
            }
        });
    }

    /// Handle clicking the close button on a tab.
    fn on_tab_close(&mut self, close_index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        let app_state = self.app_state.clone();
        app_state.update(cx, |state, cx| {
            if close_index < state.tabs.len() {
                state.tabs.remove(close_index);
                // Update active tab index
                if state.tabs.is_empty() {
                    state.active_tab = None;
                } else if state.active_tab == Some(close_index) {
                    // Closed the active tab, select previous or first
                    state.active_tab = if close_index > 0 {
                        Some(close_index - 1)
                    } else {
                        Some(0)
                    };
                } else if state.active_tab.map_or(false, |a| a > close_index) {
                    // Active tab is after closed tab, adjust index
                    state.active_tab = state.active_tab.map(|a| a - 1);
                }
                cx.notify();
            }
        });
    }

    /// Handle the CloseTab action (cmd-w).
    fn on_action_close_tab(&mut self, _: &CloseTab, _window: &mut Window, cx: &mut Context<Self>) {
        let app_state = self.app_state.clone();
        app_state.update(cx, |state, cx| {
            if let Some(index) = state.active_tab {
                state.tabs.remove(index);
                // Update active tab index
                if state.tabs.is_empty() {
                    state.active_tab = None;
                } else if index >= state.tabs.len() {
                    state.active_tab = Some(state.tabs.len() - 1);
                } else {
                    state.active_tab = Some(index);
                }
                cx.notify();
            }
        });
    }

    /// Handle the NextTab action (ctrl-tab).
    fn on_action_next_tab(&mut self, _: &NextTab, _window: &mut Window, cx: &mut Context<Self>) {
        let app_state = self.app_state.clone();
        app_state.update(cx, |state, cx| {
            if state.tabs.is_empty() {
                return;
            }
            let next_index = match state.active_tab {
                Some(index) => (index + 1) % state.tabs.len(),
                None => 0,
            };
            state.active_tab = Some(next_index);
            cx.notify();
        });
    }

    /// Handle the PreviousTab action (ctrl-shift-tab).
    fn on_action_previous_tab(
        &mut self,
        _: &PreviousTab,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let app_state = self.app_state.clone();
        app_state.update(cx, |state, cx| {
            if state.tabs.is_empty() {
                return;
            }
            let prev_index = match state.active_tab {
                Some(0) => state.tabs.len() - 1,
                Some(index) => index - 1,
                None => 0,
            };
            state.active_tab = Some(prev_index);
            cx.notify();
        });
    }

    /// Render a method badge for the tab.
    fn render_method_badge(&self, method: &HttpMethod, cx: &mut Context<Self>) -> impl IntoElement {
        let method_str = method.to_string();

        div()
            .px(px(4.0))
            .py(px(2.0))
            .rounded_sm()
            .text_color(cx.theme().foreground)
            .child(method_str)
    }

    /// Render a single tab with method badge, name, dirty indicator, and close button.
    fn render_tab(&self, index: usize, cx: &mut Context<Self>) -> impl IntoElement {
        // Extract just the fields we need - no cloning of TabState
        let (method, name, is_active, is_dirty, is_loading) = {
            let app_state = self.app_state.read(cx);
            let tab = app_state.tabs.get(index);
            let is_active = app_state.active_tab == Some(index);
            if let Some(tab) = tab {
                (
                    tab.draft.method.clone(),
                    tab.draft.name.clone(),
                    is_active,
                    tab.is_dirty,
                    tab.is_loading,
                )
            } else {
                // Return early if no tab
                return div().h(px(32.0)).w(px(120.0));
            }
        };

        let tab_index = index;

        // Build the tab content
        let mut tab_content = h_flex()
            .gap_2()
            .items_center()
            .h(px(32.0))
            .min_w(px(120.0))
            .max_w(px(200.0))
            .px_3()
            .rounded_lg()
            .border_1()
            .border_color(cx.theme().border);

        // Set background and border based on active state
        if is_active {
            tab_content = tab_content
                .bg(cx.theme().background)
                .border_b_0()
                .rounded_bl_none()
                .rounded_br_none();
        } else {
            tab_content = tab_content.bg(cx.theme().muted);
        }

        // Method badge
        tab_content = tab_content.child(self.render_method_badge(&method, cx));

        // Tab name
        tab_content = tab_content.child(
            div()
                .flex_1()
                .text_sm()
                .font_medium()
                .text_color(cx.theme().foreground)
                .overflow_hidden()
                .child(name),
        );

        // Dirty indicator
        if is_dirty {
            tab_content =
                tab_content.child(div().size(px(6.0)).rounded_full().bg(cx.theme().accent));
        }

        // Loading indicator
        if is_loading {
            tab_content = tab_content.child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("⟳"),
            );
        }

        // Close button
        let close_button = div()
            .size(px(20.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .cursor_pointer()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    this.on_tab_close(tab_index, window, cx);
                }),
            )
            .child(
                Icon::new(IconName::Close)
                    .size(px(12.0))
                    .text_color(cx.theme().muted_foreground),
            );

        tab_content = tab_content.child(close_button);

        // Add mouse events for tab selection
        tab_content.on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _, window, cx| {
                this.on_tab_click(tab_index, window, cx);
            }),
        )
    }

    /// Render all tabs horizontally.
    fn render_tabs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let tab_count = {
            let app_state = self.app_state.read(cx);
            app_state.tabs.len()
        };

        if tab_count == 0 {
            // Empty state - no tabs open
            return h_flex()
                .id("tab-bar-empty")
                .w_full()
                .h(px(40.0))
                .px_3()
                .items_center()
                .border_b_1()
                .border_color(cx.theme().border)
                .bg(cx.theme().background)
                .child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child("No tabs open - select a request from the sidebar"),
                );
        }

        // Render tabs in a scrollable container
        let mut tabs_container = h_flex()
            .id("tab-bar")
            .w_full()
            .h(px(40.0))
            .px_2()
            .gap_1()
            .items_center()
            .border_b_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().muted)
            .overflow_x_hidden();

        // Add each tab
        for i in 0..tab_count {
            let tab_element = self.render_tab(i, cx);
            tabs_container = tabs_container.child(tab_element);
        }

        // Add spacer and "new tab" button
        tabs_container = tabs_container.child(div().flex_1()).child(
            div()
                .size(px(28.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded_md()
                .cursor_pointer()
                .child(
                    Icon::new(IconName::Plus)
                        .size(px(14.0))
                        .text_color(cx.theme().muted_foreground),
                ),
        );

        tabs_container
    }
}

impl Render for RequestTabBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("request-tab-bar")
            .key_context(TAB_CONTEXT)
            .on_action(cx.listener(Self::on_action_close_tab))
            .on_action(cx.listener(Self::on_action_next_tab))
            .on_action(cx.listener(Self::on_action_previous_tab))
            .child(self.render_tabs(cx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqforge_core::ReqForgeCore;

    #[gpui::test]
    fn test_tab_bar_creation(cx: &mut gpui::TestAppContext) {
        let core = ReqForgeCore::new();
        let app_state = cx.new(|_| AppState::new(core));
        let tab_bar = cx.new(|cx| RequestTabBar::new(app_state.clone(), cx));

        // Verify tab bar was created
        let tab_bar_read = tab_bar.read(cx);
        assert_eq!(tab_bar_read.hovered_tab_index, None);
    }
}
