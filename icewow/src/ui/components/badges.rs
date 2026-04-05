use iced::widget::{container, text};
use iced::Element;

use crate::model::HttpMethod;
use crate::ui::styles;

/// HTTP method badge (GET, POST, etc.) — colored pill.
pub fn method_badge<M: 'static>(method: HttpMethod) -> Element<'static, M> {
    container(text(method.as_str()).size(13))
        .padding([6, 10])
        .style(move |theme| styles::method_badge(theme, method))
        .into()
}

/// Response status badge — colored by status range (2xx green, 4xx orange, 5xx red).
pub fn status_badge<M: 'static>(status_code: u16) -> Element<'static, M> {
    let label = if status_code == 0 {
        "Error".to_string()
    } else {
        format!("{}", status_code)
    };
    container(text(label).size(13))
        .padding([4, 8])
        .style(move |theme| styles::status_badge(theme, status_code))
        .into()
}
