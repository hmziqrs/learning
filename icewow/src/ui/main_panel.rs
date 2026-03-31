use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Element, Length};

use crate::app::{Message, PostmanUiApp};
use crate::model::BodyType;
use crate::ui::components;
use crate::ui::styles;

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
            components::method_badge(method),
            text(title).size(24),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(12);

    if let Some(tab) = active_tab {
        content = content.push(view_headers_editor(tab));
        content = content.push(view_body_editor(tab));
    }

    if app.state.loading {
        content = content.push(text("Sending request...").size(14));
    } else if let Some(response) = &app.state.response {
        let response_header = row![
            components::status_badge(response.status_code),
            text(format!("{}ms", response.elapsed_ms)).size(12),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        let mut response_details = column![response_header].spacing(8);

        if !response.headers.is_empty() {
            let mut header_rows = column![text("Headers").size(12)].spacing(4);
            for (key, value) in &response.headers {
                header_rows = header_rows.push(
                    row![
                        text(format!("{key}:")).size(11),
                        text(value.clone()).size(11),
                    ]
                    .spacing(4),
                );
            }
            response_details = response_details.push(header_rows);
        }

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

        content = content.push(response_details).push(body_section);
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

fn view_headers_editor(tab: &crate::model::Tab) -> Element<'_, Message> {
    let mut rows = column![text("Headers").size(12)].spacing(4);

    for (i, (key, value)) in tab.headers.iter().enumerate() {
        let key_input = text_input("Header", key)
            .on_input(move |v| Message::UpdateHeaderKey(i, v))
            .width(Length::Fill)
            .size(13);

        let value_input = text_input("Value", value)
            .on_input(move |v| Message::UpdateHeaderValue(i, v))
            .width(Length::Fill)
            .size(13);

        let remove = components::icon_button("×")
            .on_press(Message::RemoveHeader(i));

        rows = rows.push(
            row![key_input, value_input, remove]
                .spacing(4)
                .align_y(iced::Alignment::Center),
        );
    }

    rows = rows.push(
        components::secondary_button("+ Header")
            .on_press(Message::AddHeader)
            .padding([4, 8]),
    );

    container(rows)
        .padding(8)
        .width(Length::Fill)
        .style(|theme| crate::ui::styles::panel(theme))
        .into()
}

fn view_body_editor(tab: &crate::model::Tab) -> Element<'_, Message> {
    let types = [
        (BodyType::None, "None"),
        (BodyType::Raw, "Raw"),
        (BodyType::Json, "JSON"),
        (BodyType::Form, "Form"),
    ];

    let mut selector = row![].spacing(4);
    for (bt, label) in types {
        let active = tab.body_type == bt;
        selector = selector.push(
            button(label)
                .padding([4, 10])
                .on_press(Message::SetBodyType(bt))
                .style(move |theme, status| styles::body_type_button(theme, status, active)),
        );
    }

    let mut editor = column![row![text("Body").size(12), selector].spacing(8)].spacing(4);

    match tab.body_type {
        BodyType::None => {}
        BodyType::Raw | BodyType::Json => {
            editor = editor.push(
                text_input("Request body...", &tab.body_text)
                    .on_input(Message::UpdateBodyText)
                    .size(13),
            );
        }
        BodyType::Form => {
            for (i, (key, value)) in tab.form_pairs.iter().enumerate() {
                let key_input = text_input("Key", key)
                    .on_input(move |v| Message::UpdateFormKey(i, v))
                    .width(Length::Fill)
                    .size(13);

                let value_input = text_input("Value", value)
                    .on_input(move |v| Message::UpdateFormValue(i, v))
                    .width(Length::Fill)
                    .size(13);

                let remove = components::icon_button("×")
                    .on_press(Message::RemoveFormPair(i));

                editor = editor.push(
                    row![key_input, value_input, remove]
                        .spacing(4)
                        .align_y(iced::Alignment::Center),
                );
            }
            editor = editor.push(
                components::secondary_button("+ Pair")
                    .on_press(Message::AddFormPair)
                    .padding([4, 8]),
            );
        }
    }

    container(editor)
        .padding(8)
        .width(Length::Fill)
        .style(|theme| crate::ui::styles::panel(theme))
        .into()
}
