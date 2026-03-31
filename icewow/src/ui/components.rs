use iced::widget::{self, button, container, text};

use crate::app::Message;
use crate::model::HttpMethod;
use crate::ui::styles;

/// Small icon-style button (⋯, ×, ▾, ▸) — transparent with hover highlight.
pub fn icon_button(label: &str) -> widget::Button<'_, Message> {
    button(label)
        .padding([2, 6])
        .style(|theme, status| styles::handle_button(theme, status))
}

/// Context menu item button — bordered with hover highlight.
pub fn menu_button(label: &str) -> widget::Button<'_, Message> {
    button(label)
        .style(|theme, status| styles::menu_button(theme, status))
}

/// Destructive action button (Delete) — red themed.
pub fn danger_button(label: &str) -> widget::Button<'_, Message> {
    button(label)
        .style(|theme, status| styles::danger_button(theme, status))
}

/// Secondary action button (Cancel, +) — purple themed.
pub fn secondary_button(label: &str) -> widget::Button<'_, Message> {
    button(label)
        .style(|theme, status| styles::secondary_button(theme, status))
}

/// HTTP method badge (GET, POST, etc.) — colored pill.
pub fn method_badge(method: HttpMethod) -> widget::Container<'static, Message> {
    container(text(method.as_str()).size(13))
        .padding([6, 10])
        .style(move |theme| styles::method_badge(theme, method))
}

/// Response status badge — colored by status range (2xx green, 4xx orange, 5xx red).
pub fn status_badge(status_code: u16) -> widget::Container<'static, Message> {
    let label = if status_code == 0 {
        "Error".to_string()
    } else {
        format!("{}", status_code)
    };
    container(text(label).size(13))
        .padding([4, 8])
        .style(move |theme| styles::status_badge(theme, status_code))
}
