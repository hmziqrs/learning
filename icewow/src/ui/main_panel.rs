use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length};

use crate::app::{Message, PostmanUiApp};
use crate::model::{BodyType, RequestTab, ResponseTab};
use crate::ui::{components, icons, styles};

/// Request name row: [method badge] Request Name                    [Save]
pub fn view_request_name_row(app: &PostmanUiApp) -> Element<'_, Message> {
    let active_tab = app.state.active_tab_ref();

    let method = active_tab
        .map(|tab| tab.method)
        .unwrap_or(crate::model::HttpMethod::Get);

    let title = active_tab
        .map(|tab| tab.title.clone())
        .unwrap_or_else(|| "Untitled Request".to_string());

    let name_input = text_input("Request name", &title)
        .on_input(Message::RequestNameChanged)
        .size(14)
        .width(Length::Fill);

    let content = row![
        components::method_badge(method),
        name_input,
        button(text("Save").size(13))
            .on_press(Message::SaveRequest)
            .padding([4, 12])
            .style(|theme, status| styles::save_button(theme, status)),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    container(content)
        .padding([6, 12])
        .width(Length::Fill)
        .style(|theme| styles::panel(theme))
        .into()
}

/// Request section tabs: [Params] [Headers (N)] [Body]
pub fn view_request_section_tabs(app: &PostmanUiApp) -> Element<'_, Message> {
    let active_tab = match app.state.active_tab_ref() {
        Some(tab) => tab,
        None => return container(Space::new()).into(),
    };

    let active = active_tab.active_request_tab;
    let header_count = active_tab.headers.len();
    let param_count = active_tab.query_params.len();

    let param_label = if param_count > 0 {
        format!("Params ({param_count})")
    } else {
        "Params".to_string()
    };
    let header_label = if header_count > 0 {
        format!("Headers ({header_count})")
    } else {
        "Headers".to_string()
    };

    let tabs = row![
        section_tab_button(param_label, RequestTab::Params, active),
        section_tab_button(header_label, RequestTab::Headers, active),
        section_tab_button("Body".to_string(), RequestTab::Body, active),
    ]
    .spacing(0);

    container(tabs)
        .padding([0, 12])
        .width(Length::Fill)
        .style(|theme| styles::panel(theme))
        .into()
}

fn section_tab_button(label: String, tab: RequestTab, active: RequestTab) -> Element<'static, Message> {
    let is_active = tab == active;
    button(text(label).size(13))
        .on_press(Message::SetRequestTab(tab))
        .padding([8, 12])
        .style(move |theme, _status| styles::section_tab(theme, is_active))
        .into()
}

/// Request tab content — renders content for the active request tab
pub fn view_request_content(app: &PostmanUiApp) -> Element<'_, Message> {
    let active_tab = match app.state.active_tab_ref() {
        Some(tab) => tab,
        None => return container(Space::new()).into(),
    };

    let content: Element<'_, Message> = match active_tab.active_request_tab {
        RequestTab::Params => view_params_editor(active_tab),
        RequestTab::Headers => view_headers_editor(active_tab),
        RequestTab::Body => view_body_editor(active_tab),
    };

    container(content)
        .padding(12)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme| styles::content_area(theme))
        .into()
}

/// Response section at the bottom
pub fn view_response_section(app: &PostmanUiApp) -> Element<'_, Message> {
    let divider = container(Space::new().height(1))
        .width(Length::Fill)
        .style(|theme| styles::section_divider(theme));

    if app.state.loading {
        let content = column![divider, text("Sending request...").size(14)]
            .spacing(8);
        return container(content)
            .padding(12)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into();
    }

    let Some(response) = &app.state.response else {
        let content = column![
            divider,
            text("Enter a URL and press Send to get a response").size(13),
        ]
        .spacing(8);
        return container(content)
            .padding(12)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into();
    };

    let response_header = row![
        components::status_badge(response.status_code),
        text(format!("{}ms", response.elapsed_ms)).size(12),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    let active = app.state.active_response_tab;
    let response_tabs = row![
        response_tab_button("Body".to_string(), ResponseTab::Body, active),
        response_tab_button("Cookies".to_string(), ResponseTab::Cookies, active),
        response_tab_button("Headers".to_string(), ResponseTab::Headers, active),
    ]
    .spacing(0);

    let response_tab_content: Element<'_, Message> = match active {
        ResponseTab::Body => {
            let body_display = if response.body.is_empty() {
                text("(empty response)").size(13)
            } else {
                text(response.body.clone()).size(13)
            };
            scrollable(body_display).into()
        }
        ResponseTab::Cookies => {
            text("No cookies").size(13).into()
        }
        ResponseTab::Headers => {
            let mut header_rows = column![].spacing(4);
            for (key, value) in &response.headers {
                header_rows = header_rows.push(
                    row![
                        text(format!("{key}:")).size(12),
                        text(value.clone()).size(12),
                    ]
                    .spacing(4),
                );
            }
            if response.headers.is_empty() {
                header_rows = header_rows.push(text("No headers").size(13));
            }
            scrollable(header_rows).into()
        }
    };

    let content = column![
        divider,
        response_header,
        response_tabs,
        container(response_tab_content)
            .height(Length::Fixed(200.0))
            .width(Length::Fill),
    ]
    .spacing(6);

    container(content)
        .padding([8, 12])
        .width(Length::Fill)
        .into()
}

fn response_tab_button(label: String, tab: ResponseTab, active: ResponseTab) -> Element<'static, Message> {
    let is_active = tab == active;
    button(text(label).size(12))
        .on_press(Message::SetResponseTab(tab))
        .padding([4, 10])
        .style(move |theme, _status| styles::section_tab(theme, is_active))
        .into()
}

fn view_params_editor(tab: &crate::model::Tab) -> Element<'_, Message> {
    let mut rows = column![].spacing(4);

    for (i, (key, value)) in tab.query_params.iter().enumerate() {
        let key_input = text_input("Key", key)
            .on_input(move |v| Message::UpdateQueryParamKey(i, v))
            .width(Length::Fill)
            .size(13);

        let value_input = text_input("Value", value)
            .on_input(move |v| Message::UpdateQueryParamValue(i, v))
            .width(Length::Fill)
            .size(13);

        let remove = components::icon_button(icons::lucide_icon("x", 14.0))
            .on_press(Message::RemoveQueryParam(i));

        rows = rows.push(
            row![key_input, value_input, remove]
                .spacing(4)
                .align_y(iced::Alignment::Center),
        );
    }

    rows = rows.push(
        components::secondary_button("+ Param")
            .on_press(Message::AddQueryParam)
            .padding([4, 8]),
    );

    column![rows].into()
}

fn view_headers_editor(tab: &crate::model::Tab) -> Element<'_, Message> {
    let mut rows = column![].spacing(4);

    for (i, (key, value)) in tab.headers.iter().enumerate() {
        let key_input = text_input("Header", key)
            .on_input(move |v| Message::UpdateHeaderKey(i, v))
            .width(Length::Fill)
            .size(13);

        let value_input = text_input("Value", value)
            .on_input(move |v| Message::UpdateHeaderValue(i, v))
            .width(Length::Fill)
            .size(13);

        let remove = components::icon_button(icons::lucide_icon("x", 14.0))
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

    column![rows].into()
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

    let mut editor = column![selector].spacing(4);

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

                let remove = components::icon_button(icons::lucide_icon("x", 14.0))
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

    column![editor].into()
}
