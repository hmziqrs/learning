use iced::widget::{column, container, row, text};
use iced::{Element, Length};

use crate::app::{Message, PostmanUiApp};

pub fn view_main_panel(app: &PostmanUiApp) -> Element<'_, Message> {
    let active_tab = app.state.active_tab_ref();

    let title = active_tab
        .map(|tab| tab.title.clone())
        .unwrap_or_else(|| "No tab selected".to_string());

    let request_info = active_tab
        .and_then(|tab| tab.request_id)
        .and_then(|id| app.state.find_request(id))
        .map(|request| format!("Request: {}", request.name))
        .unwrap_or_else(|| "Request: (blank tab)".to_string());

    let content = column![
        row![
            container(text("GET").size(13))
                .padding([6, 10])
                .style(|theme| crate::ui::styles::method_badge(theme)),
            text(title).size(24),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        text(request_info).size(14),
        text("UI-only prototype: request execution is intentionally disabled.").size(14),
        text(format!("URL: {}", app.state.url_input)).size(14),
    ]
    .spacing(12);

    container(content)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme| crate::ui::styles::panel(theme))
        .into()
}
