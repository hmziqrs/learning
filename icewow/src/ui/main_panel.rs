use iced::widget::{column, container, row, scrollable, text};
use iced::{Element, Length};

use crate::app::{Message, PostmanUiApp};

pub fn view_main_panel(app: &PostmanUiApp) -> Element<'_, Message> {
    let active_tab = app.state.active_tab_ref();

    let method = active_tab
        .map(|tab| tab.method)
        .unwrap_or(crate::model::HttpMethod::Get);

    let title = active_tab
        .map(|tab| tab.title.clone())
        .unwrap_or_else(|| "No tab selected".to_string());

    let mut content = column![
        row![
            container(text(method.as_str()).size(13))
                .padding([6, 10])
                .style(move |theme| crate::ui::styles::method_badge(theme, method)),
            text(title).size(24),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(12);

    if app.state.loading {
        content = content.push(text("Sending request...").size(14));
    } else if let Some(response) = &app.state.response {
        let status_text = if response.status_code == 0 {
            "Error".to_string()
        } else {
            format!("{}", response.status_code)
        };

        let response_header = row![
            container(text(status_text).size(13))
                .padding([4, 8])
                .style(move |theme| {
                    crate::ui::styles::status_badge(theme, response.status_code)
                }),
            text(format!("{}ms", response.elapsed_ms)).size(12),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        let body_display = if response.body.is_empty() {
            text("(empty response)").size(13)
        } else {
            text(response.body.clone()).size(13)
        };

        let body_section = container(scrollable(body_display))
            .padding(12)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme| crate::ui::styles::response_panel(theme));

        content = content.push(response_header).push(body_section);
    } else {
        content = content.push(
            text("Enter a URL and press Send to make a request.").size(14),
        );
    }

    container(content)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme| crate::ui::styles::panel(theme))
        .into()
}
