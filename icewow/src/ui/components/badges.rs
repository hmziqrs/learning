use iced::widget::{container, text};
use iced::Element;

use crate::model::HttpMethod;
use crate::ui::scale::UiScale;
use crate::ui::styles;

/// HTTP method badge (GET, POST, etc.) — colored pill.
pub fn method_badge<'a, M: 'static>(method: HttpMethod, scale: &UiScale) -> Element<'a, M> {
    container(text(method.as_str()).size(scale.text_body()))
        .padding(scale.pad_badge_method())
        .style(move |theme| styles::method_badge(theme, method))
        .into()
}

/// Response status badge — colored by status range (2xx green, 4xx orange, 5xx red).
pub fn status_badge<'a, M: 'static>(status_code: u16, scale: &UiScale) -> Element<'a, M> {
    let label = if status_code == 0 {
        "Error".to_string()
    } else {
        format!("{}", status_code)
    };
    container(text(label).size(scale.text_body()))
        .padding(scale.pad_badge_status())
        .style(move |theme| styles::status_badge(theme, status_code))
        .into()
}
