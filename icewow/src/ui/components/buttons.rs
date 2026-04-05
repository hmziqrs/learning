use iced::widget::{self, button};
use iced::Element;

use crate::ui::scale::UiScale;
use crate::ui::styles;

/// Small icon-style button — transparent with hover highlight.
pub fn icon_button<'a, M: 'a>(
    icon: impl Into<Element<'a, M>>,
    scale: &UiScale,
) -> widget::Button<'a, M> {
    button(icon)
        .padding(scale.pad_icon())
        .style(|theme, status| styles::handle_button(theme, status))
}

/// Context menu item button — bordered with hover highlight.
pub fn menu_button<M>(label: &str) -> widget::Button<'_, M> {
    button(label)
        .style(|theme, status| styles::menu_button(theme, status))
}

/// Destructive action button (Delete) — red themed.
pub fn danger_button<M>(label: &str) -> widget::Button<'_, M> {
    button(label)
        .style(|theme, status| styles::danger_button(theme, status))
}

/// Secondary action button (Cancel, +) — secondary themed.
pub fn secondary_button<M>(label: &str) -> widget::Button<'_, M> {
    button(label)
        .style(|theme, status| styles::secondary_button(theme, status))
}
