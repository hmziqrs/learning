//! Response viewer component - displays HTTP response data.
//!
//! This component renders the HTTP response including status, timing, size,
//! and the response body/headers in sub-tabs using gpui-component.

use crate::app_state::AppState;
use gpui::{div, px, Context, Element, InteractiveElement, Render, Window, IntoElement, Styled, ParentElement};
use gpui_component::{h_flex, v_flex, tab::TabBar, tab::Tab, ActiveTheme};
use std::time::Duration;

/// Sub-tabs within the response viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseSubTab {
    /// Response body
    Body,
    /// Response headers
    Headers,
}

impl ResponseSubTab {
    /// Get the display name for this sub-tab.
    pub fn display_name(&self) -> &str {
        match self {
            ResponseSubTab::Body => "Body",
            ResponseSubTab::Headers => "Headers",
        }
    }

    /// Get the index for this sub-tab.
    pub fn index(&self) -> usize {
        match self {
            ResponseSubTab::Body => 0,
            ResponseSubTab::Headers => 1,
        }
    }
}

/// Response viewer component.
///
/// Displays the HTTP response including status, timing, size,
/// and the response body/headers in sub-tabs.
pub struct ResponseViewer {
    /// The application state entity
    app_state: gpui::Entity<AppState>,
    /// Current sub-tab
    active_sub_tab: ResponseSubTab,
}

impl ResponseViewer {
    /// Create a new ResponseViewer.
    pub fn new(app_state: gpui::Entity<AppState>) -> Self {
        Self {
            app_state,
            active_sub_tab: ResponseSubTab::Body,
        }
    }

    /// Format duration for display.
    fn format_duration(&self, duration: &Duration) -> String {
        if duration.as_millis() < 1 {
            format!("{}μs", duration.as_micros())
        } else if duration.as_secs() < 1 {
            format!("{}ms", duration.as_millis())
        } else {
            format!("{:.1}s", duration.as_secs_f64())
        }
    }

    /// Format size for display.
    fn format_size(&self, size: usize) -> String {
        const KB: usize = 1024;
        const MB: usize = KB * 1024;

        if size < KB {
            format!("{} B", size)
        } else if size < MB {
            format!("{:.1} KB", size as f64 / KB as f64)
        } else {
            format!("{:.1} MB", size as f64 / MB as f64)
        }
    }
}

impl Render for ResponseViewer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Read app state first, dropping the borrow before using cx further
        let (status_info, has_response) = {
            let app_state = self.app_state.read(cx);
            let active_tab = app_state.active_tab();

            if let Some(tab) = active_tab {
                if let Some(response) = &tab.last_response {
                    let time_str = self.format_duration(&response.elapsed);
                    let size_str = self.format_size(response.size_bytes);
                    let status = response.status;
                    let status_text = response.status_text.clone();

                    (
                        Some((status, status_text, time_str, size_str)),
                        true
                    )
                } else {
                    (None, false)
                }
            } else {
                (None, false)
            }
        };

        // Now we can use cx for theme access
        let status_bar = if let Some((status, status_text, time_str, size_str)) = status_info {
            let category = status / 100;
            let status_color = match category {
                2 => cx.theme().green,
                3 => cx.theme().yellow,
                4 => cx.theme().yellow,
                5 => cx.theme().red,
                _ => cx.theme().muted_foreground,
            };

            h_flex()
                .id("response-status-bar")
                .w_full()
                .p_2()
                .gap_2()
                .items_center()
                .border_b_1()
                .border_color(cx.theme().border)
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(4.))
                        .bg(status_color)
                        .text_color(status_color)
                        .font_weight(gpui::FontWeight::BOLD)
                        .child(format!("{}", status))
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child(status_text)
                )
                .child(div().flex_1())
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child(format!("{} · {}", time_str, size_str))
                )
        } else {
            h_flex()
                .id("response-status-bar")
                .w_full()
                .p_2()
                .items_center()
                .border_b_1()
                .border_color(cx.theme().border)
                .child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child("No Response")
                )
        };

        // Tab bar
        let active_index = self.active_sub_tab.index();
        let tab_bar = TabBar::new("response-sub-tabs")
            .selected_index(active_index)
            .on_click(cx.listener(|view, index, _, cx| {
                view.active_sub_tab = match *index {
                    0 => ResponseSubTab::Body,
                    1 => ResponseSubTab::Headers,
                    _ => ResponseSubTab::Body,
                };
                cx.notify();
            }))
            .child(Tab::new().label("Body"))
            .child(Tab::new().label("Headers"));

        // Content panel
        let content_panel = if has_response {
            // Read app state again for content rendering
            let app_state = self.app_state.read(cx);
            let active_tab = app_state.active_tab();
            let tab = active_tab.and_then(|t| t.last_response.as_ref());

            match (self.active_sub_tab, tab) {
                (ResponseSubTab::Body, Some(response)) => {
                    let body_display = if let Some(pretty_json) = response.pretty_body() {
                        pretty_json
                    } else if let Some(raw_text) = response.body_text() {
                        raw_text.to_string()
                    } else {
                        "<Binary Data>".to_string()
                    };

                    div()
                        .id("response-body-content")
                        .flex_1()
                        .p_4()
                        .font_family("Monospace")
                        .text_sm()
                        .child(body_display)
                        .into_any()
                }
                (ResponseSubTab::Headers, Some(response)) => {
                    let headers_vec: Vec<(String, String)> = response.headers
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();

                    let mut header_div = div()
                        .id("response-headers-content")
                        .flex_1()
                        .p_4();

                    for (key, value) in headers_vec {
                        header_div = header_div.child(
                            h_flex()
                                .gap_2()
                                .py_1()
                                .border_b_1()
                                .border_color(cx.theme().border)
                                .child(
                                    div()
                                        .w(px(200.))
                                        .flex_shrink_0()
                                        .font_weight(gpui::FontWeight::MEDIUM)
                                        .text_color(cx.theme().muted_foreground)
                                        .child(key.clone())
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .font_family("Monospace")
                                        .text_sm()
                                        .child(value.clone())
                                )
                        );
                    }

                    header_div.into_any()
                }
                _ => {
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            v_flex()
                                .gap_2()
                                .items_center()
                                .child(
                                    div()
                                        .text_lg()
                                        .text_color(cx.theme().muted_foreground)
                                        .child("No response data available")
                                )
                        )
                        .into_any()
                }
            }
        } else {
            // Empty state
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    v_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .text_lg()
                                .text_color(cx.theme().muted_foreground)
                                .child("Send a request to see the response here")
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().muted_foreground)
                                .child("Click the Send button in the request editor above")
                        )
                )
                .into_any()
        };

        // Main layout
        v_flex()
            .id("response-viewer")
            .size_full()
            .border_t_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .child(status_bar)
            .child(tab_bar)
            .child(div().flex_1().child(content_panel))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple unit tests that don't require GPUI context
    #[test]
    fn test_sub_tab_display_names() {
        assert_eq!(ResponseSubTab::Body.display_name(), "Body");
        assert_eq!(ResponseSubTab::Headers.display_name(), "Headers");
    }

    #[test]
    fn test_sub_tab_indices() {
        assert_eq!(ResponseSubTab::Body.index(), 0);
        assert_eq!(ResponseSubTab::Headers.index(), 1);
    }

    // Helper struct for testing formatting methods
    struct TestViewer;

    impl TestViewer {
        fn format_duration(&self, duration: &Duration) -> String {
            if duration.as_millis() < 1 {
                format!("{}μs", duration.as_micros())
            } else if duration.as_secs() < 1 {
                format!("{}ms", duration.as_millis())
            } else {
                format!("{:.1}s", duration.as_secs_f64())
            }
        }

        fn format_size(&self, size: usize) -> String {
            const KB: usize = 1024;
            const MB: usize = KB * 1024;

            if size < KB {
                format!("{} B", size)
            } else if size < MB {
                format!("{:.1} KB", size as f64 / KB as f64)
            } else {
                format!("{:.1} MB", size as f64 / MB as f64)
            }
        }
    }

    #[test]
    fn test_format_duration() {
        let viewer = TestViewer;

        // Microseconds
        assert_eq!(viewer.format_duration(&Duration::from_micros(500)), "500μs");

        // Milliseconds
        assert_eq!(viewer.format_duration(&Duration::from_millis(500)), "500ms");

        // Seconds
        assert_eq!(viewer.format_duration(&Duration::from_secs(2)), "2.0s");
        assert_eq!(viewer.format_duration(&Duration::from_millis(2500)), "2.5s");
    }

    #[test]
    fn test_format_size() {
        let viewer = TestViewer;

        // Bytes
        assert_eq!(viewer.format_size(500), "500 B");

        // Kilobytes
        assert_eq!(viewer.format_size(2048), "2.0 KB");
        assert_eq!(viewer.format_size(1536), "1.5 KB");

        // Megabytes
        assert_eq!(viewer.format_size(2 * 1024 * 1024), "2.0 MB");
        assert_eq!(viewer.format_size(3 * 1024 * 1024), "3.0 MB");
    }
}
