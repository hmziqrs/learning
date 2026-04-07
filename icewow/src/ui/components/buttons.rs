use iced::widget::{self, button};
use iced::Element;

use crate::ui::scale::UiScale;
use crate::ui::styles;

/// Small icon-style button — transparent with hover highlight.
pub fn icon_button<'a, M: 'a>(
    icon: impl Into<Element<'a, M>>,
    scale: &UiScale,
    hover_t: f32,
) -> widget::Button<'a, M> {
    button(icon)
        .padding(scale.pad_icon())
        .style(move |theme, status| styles::handle_button(theme, status, hover_t))
}

/// Context menu item button — bordered with hover highlight.
pub fn menu_button<'a, M: 'a>(label: &'a str, scale: &UiScale, hover_t: f32) -> widget::Button<'a, M> {
    button(label)
        .padding(scale.pad_button())
        .style(move |theme, status| styles::menu_button(theme, status, hover_t))
}

/// Destructive action button (Delete) — red themed.
pub fn danger_button<'a, M: 'a>(label: &'a str, scale: &UiScale, hover_t: f32) -> widget::Button<'a, M> {
    button(label)
        .padding(scale.pad_button())
        .style(move |theme, status| styles::danger_button(theme, status, hover_t))
}

/// Secondary action button (Cancel, +) — secondary themed.
pub fn secondary_button<'a, M: 'a>(label: &'a str, scale: &UiScale, hover_t: f32) -> widget::Button<'a, M> {
    button(label)
        .padding(scale.pad_button())
        .style(move |theme, status| styles::secondary_button(theme, status, hover_t))
}
