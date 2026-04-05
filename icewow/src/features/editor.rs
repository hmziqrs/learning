use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Task};

use crate::app::Message;
use crate::features::ResponseMsg;
use crate::model::{AppState, BodyType, HttpMethod, RequestTab, ResponseTab, Tab};
use crate::ui::{components, scale::UiScale, styles};

#[derive(Debug, Clone)]
pub enum EditorMsg {
    UrlChanged(String),
    MethodChanged(HttpMethod),
    SetBodyType(BodyType),
    UpdateBodyText(String),
    AddFormPair,
    UpdateFormKey(usize, String),
    UpdateFormValue(usize, String),
    RemoveFormPair(usize),
    AddHeader,
    UpdateHeaderKey(usize, String),
    UpdateHeaderValue(usize, String),
    RemoveHeader(usize),
    AddQueryParam,
    UpdateQueryParamKey(usize, String),
    UpdateQueryParamValue(usize, String),
    RemoveQueryParam(usize),
    SetRequestTab(RequestTab),
    SaveRequest,
    RequestNameChanged(String),
}

// ── Update handler ─────────────────────────────────────────────

pub fn update(state: &mut AppState, msg: EditorMsg) -> Task<Message> {
    match msg {
        EditorMsg::UrlChanged(value) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.url_input = value;
                tab.dirty = true;
            }
        }
        EditorMsg::MethodChanged(method) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.method = method;
                tab.dirty = true;
            }
        }
        EditorMsg::SetBodyType(body_type) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.body_type = body_type;
                tab.dirty = true;
            }
        }
        EditorMsg::UpdateBodyText(text) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.body_text = text;
                tab.dirty = true;
            }
        }
        EditorMsg::AddFormPair => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.form_pairs.push((String::new(), String::new()));
                tab.dirty = true;
            }
        }
        EditorMsg::UpdateFormKey(index, key) => {
            if let Some(tab) = state.tabs.active_mut() {
                if let Some(pair) = tab.form_pairs.get_mut(index) {
                    pair.0 = key;
                    tab.dirty = true;
                }
            }
        }
        EditorMsg::UpdateFormValue(index, value) => {
            if let Some(tab) = state.tabs.active_mut() {
                if let Some(pair) = tab.form_pairs.get_mut(index) {
                    pair.1 = value;
                    tab.dirty = true;
                }
            }
        }
        EditorMsg::RemoveFormPair(index) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.form_pairs.remove(index);
                tab.dirty = true;
            }
        }
        EditorMsg::AddHeader => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.headers.push((String::new(), String::new()));
                tab.dirty = true;
            }
        }
        EditorMsg::UpdateHeaderKey(index, key) => {
            if let Some(tab) = state.tabs.active_mut() {
                if let Some(pair) = tab.headers.get_mut(index) {
                    pair.0 = key;
                    tab.dirty = true;
                }
            }
        }
        EditorMsg::UpdateHeaderValue(index, value) => {
            if let Some(tab) = state.tabs.active_mut() {
                if let Some(pair) = tab.headers.get_mut(index) {
                    pair.1 = value;
                    tab.dirty = true;
                }
            }
        }
        EditorMsg::RemoveHeader(index) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.headers.remove(index);
                tab.dirty = true;
            }
        }
        EditorMsg::AddQueryParam => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.query_params.push((String::new(), String::new()));
                tab.dirty = true;
            }
        }
        EditorMsg::UpdateQueryParamKey(index, key) => {
            if let Some(tab) = state.tabs.active_mut() {
                if let Some(pair) = tab.query_params.get_mut(index) {
                    pair.0 = key;
                    tab.dirty = true;
                }
            }
        }
        EditorMsg::UpdateQueryParamValue(index, value) => {
            if let Some(tab) = state.tabs.active_mut() {
                if let Some(pair) = tab.query_params.get_mut(index) {
                    pair.1 = value;
                    tab.dirty = true;
                }
            }
        }
        EditorMsg::RemoveQueryParam(index) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.query_params.remove(index);
                tab.dirty = true;
            }
        }
        EditorMsg::SetRequestTab(request_tab) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.active_request_tab = request_tab;
            }
        }
        EditorMsg::SaveRequest => {
            let tab = match state.tabs.active() {
                Some(tab) => tab,
                None => return Task::none(),
            };

            if let Some(request_id) = tab.request_id {
                let name = tab.title.clone();
                let url = tab.url_input.clone();
                let method = tab.method;

                state.tree.update_request_from_draft(
                    request_id,
                    &name,
                    &url,
                    method,
                );

                if let Some(tab) = state.tabs.active_mut() {
                    tab.dirty = false;
                }
            }
        }
        EditorMsg::RequestNameChanged(name) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.title = name;
                tab.dirty = true;
            }
        }
    }
    Task::none()
}

// ── View functions ──────────────────────────────────────────────

/// Request name row: [method badge] Request Name                    [Save]
pub fn view_request_name_row<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, Message> {
    let method = tab.method;
    let title = tab.title.clone();
    let dirty = tab.dirty;

    let name_input = text_input("Request name", &title)
        .on_input(|v| Message::Editor(EditorMsg::RequestNameChanged(v)))
        .size(scale.text_label())
        .width(Length::Fill);

    let mut save_btn = button(text("Save").size(scale.text_body()))
        .padding([scale.space_sm(), scale.space_lg()])
        .style(|theme, status| styles::save_button(theme, status));

    if dirty {
        save_btn = save_btn.on_press(Message::Editor(EditorMsg::SaveRequest));
    }

    let content = row![
        components::method_badge(method),
        name_input,
        save_btn,
    ]
    .spacing(scale.space_md())
    .align_y(iced::Alignment::Center);

    container(content)
        .padding(scale.pad_button())
        .width(Length::Fill)
        .style(|theme| styles::panel(theme))
        .into()
}

/// Request section tabs: [Params] [Headers (N)] [Body]
pub fn view_request_section_tabs<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, Message> {
    let active = tab.active_request_tab;
    let header_count = tab.headers.len();
    let param_count = tab.query_params.len();

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
        section_tab_button(param_label, RequestTab::Params, active, scale),
        section_tab_button(header_label, RequestTab::Headers, active, scale),
        section_tab_button("Body".to_string(), RequestTab::Body, active, scale),
    ]
    .spacing(0);

    container(tabs)
        .padding([0.0, scale.space_lg()])
        .width(Length::Fill)
        .style(|theme| styles::panel(theme))
        .into()
}

fn section_tab_button(
    label: String,
    tab: RequestTab,
    active: RequestTab,
    scale: &UiScale,
) -> Element<'static, Message> {
    let is_active = tab == active;
    button(text(label).size(scale.text_body()))
        .on_press(Message::Editor(EditorMsg::SetRequestTab(tab)))
        .padding([scale.space_md(), scale.space_lg()])
        .style(move |theme, _status| styles::section_tab(theme, is_active))
        .into()
}

/// Request tab content — renders content for the active request tab
pub fn view_request_content<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, Message> {
    let content: Element<'_, Message> = match tab.active_request_tab {
        RequestTab::Params => view_params_editor(tab, scale),
        RequestTab::Headers => view_headers_editor(tab, scale),
        RequestTab::Body => view_body_editor(tab, scale),
    };

    container(content)
        .padding(scale.space_lg())
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme| styles::content_area(theme))
        .into()
}

fn view_params_editor<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, Message> {
    components::kv_editor(
        &tab.query_params,
        "Key",
        "Value",
        |i, v| Message::Editor(EditorMsg::UpdateQueryParamKey(i, v)),
        |i, v| Message::Editor(EditorMsg::UpdateQueryParamValue(i, v)),
        |i| Message::Editor(EditorMsg::RemoveQueryParam(i)),
        "+ Param",
        Message::Editor(EditorMsg::AddQueryParam),
        scale,
    )
}

fn view_headers_editor<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, Message> {
    components::kv_editor(
        &tab.headers,
        "Header",
        "Value",
        |i, v| Message::Editor(EditorMsg::UpdateHeaderKey(i, v)),
        |i, v| Message::Editor(EditorMsg::UpdateHeaderValue(i, v)),
        |i| Message::Editor(EditorMsg::RemoveHeader(i)),
        "+ Header",
        Message::Editor(EditorMsg::AddHeader),
        scale,
    )
}

fn view_body_editor<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, Message> {
    let types = [
        (BodyType::None, "None"),
        (BodyType::Raw, "Raw"),
        (BodyType::Json, "JSON"),
        (BodyType::Form, "Form"),
    ];

    let mut selector = row![].spacing(scale.space_sm());
    for (bt, label) in types {
        let active = tab.body_type == bt;
        selector = selector.push(
            button(label)
                .padding(scale.pad_chip())
                .on_press(Message::Editor(EditorMsg::SetBodyType(bt)))
                .style(move |theme, status| styles::body_type_button(theme, status, active)),
        );
    }

    let mut editor = column![selector].spacing(scale.space_sm());

    match tab.body_type {
        BodyType::None => {}
        BodyType::Raw | BodyType::Json => {
            editor = editor.push(
                text_input("Request body...", &tab.body_text)
                    .on_input(|v| Message::Editor(EditorMsg::UpdateBodyText(v)))
                    .size(scale.text_body()),
            );
        }
        BodyType::Form => {
            editor = editor.push(
                components::kv_editor(
                    &tab.form_pairs,
                    "Key",
                    "Value",
                    |i, v| Message::Editor(EditorMsg::UpdateFormKey(i, v)),
                    |i, v| Message::Editor(EditorMsg::UpdateFormValue(i, v)),
                    |i| Message::Editor(EditorMsg::RemoveFormPair(i)),
                    "+ Pair",
                    Message::Editor(EditorMsg::AddFormPair),
                    scale,
                ),
            );
        }
    }

    column![editor].into()
}

/// Response section at the bottom
pub fn view_response_section<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, Message> {
    let divider = container(Space::new().height(1))
        .width(Length::Fill)
        .style(|theme| styles::section_divider(theme));

    if tab.loading {
        let content = column![divider, text("Sending request...").size(scale.text_label())]
            .spacing(scale.space_md());
        return container(content)
            .padding(scale.space_lg())
            .width(Length::Fill)
            .height(Length::Shrink)
            .into();
    }

    let Some(response) = &tab.response else {
        let content = column![
            divider,
            text("Enter a URL and press Send to get a response").size(scale.text_body()),
        ]
        .spacing(scale.space_md());
        return container(content)
            .padding(scale.space_lg())
            .width(Length::Fill)
            .height(Length::Shrink)
            .into();
    };

    let response_header = row![
        components::status_badge(response.status_code),
        text(format!("{}ms", response.elapsed_ms)).size(scale.text_small()),
    ]
    .spacing(scale.space_md())
    .align_y(iced::Alignment::Center);

    let active = tab.active_response_tab;
    let response_tabs = row![
        response_tab_button("Body".to_string(), ResponseTab::Body, active, scale),
        response_tab_button("Cookies".to_string(), ResponseTab::Cookies, active, scale),
        response_tab_button("Headers".to_string(), ResponseTab::Headers, active, scale),
    ]
    .spacing(0);

    let response_tab_content: Element<'_, Message> = match active {
        ResponseTab::Body => {
            let body_display = if response.body.is_empty() {
                text("(empty response)").size(scale.text_body())
            } else {
                text(response.body.clone()).size(scale.text_body())
            };
            scrollable(body_display).into()
        }
        ResponseTab::Cookies => {
            text("No cookies").size(scale.text_body()).into()
        }
        ResponseTab::Headers => {
            let mut header_rows = column![].spacing(scale.space_sm());
            for (key, value) in &response.headers {
                header_rows = header_rows.push(
                    row![
                        text(format!("{key}:")).size(scale.text_small()),
                        text(value.clone()).size(scale.text_small()),
                    ]
                    .spacing(scale.space_sm()),
                );
            }
            if response.headers.is_empty() {
                header_rows = header_rows.push(text("No headers").size(scale.text_body()));
            }
            scrollable(header_rows).into()
        }
    };

    let content = column![
        divider,
        response_header,
        response_tabs,
        container(response_tab_content)
            .height(Length::Fixed(UiScale::RESPONSE_MIN_HEIGHT))
            .width(Length::Fill),
    ]
    .spacing(scale.space_sm());

    container(content)
        .padding([scale.space_md(), scale.space_lg()])
        .width(Length::Fill)
        .into()
}

fn response_tab_button(
    label: String,
    tab: ResponseTab,
    active: ResponseTab,
    scale: &UiScale,
) -> Element<'static, Message> {
    let is_active = tab == active;
    button(text(label).size(scale.text_small()))
        .on_press(Message::Response(ResponseMsg::SetResponseTab(tab)))
        .padding(scale.pad_chip())
        .style(move |theme, _status| styles::section_tab(theme, is_active))
        .into()
}
