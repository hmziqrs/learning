use iced::widget::{self, button};
use iced::Element;

use crate::app::Message;
use crate::ui::styles;

/// Small icon-style button — transparent with hover highlight.
pub fn icon_button<'a>(icon: impl Into<Element<'a, Message>>) -> widget::Button<'a, Message> {
    button(icon)
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

/// Secondary action button (Cancel, +) — secondary themed.
pub fn secondary_button(label: &str) -> widget::Button<'_, Message> {
    button(label)
        .style(|theme, status| styles::secondary_button(theme, status))
}
