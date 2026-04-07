use iced::widget::{button, column, container, row, text, text_input};
use iced::{Element, Length, Task};

use crate::app::Message;
use crate::model::{AppState, BodyType, HttpMethod, RequestTab, Tab};
use crate::ui::anim::ButtonId;
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
    ButtonHover(ButtonId, bool),
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
        EditorMsg::ButtonHover(id, hovered) => {
            state
                .button_anims
                .set_hover(id, hovered, iced::time::Instant::now());
        }
    }
    Task::none()
}

// ── View functions ──────────────────────────────────────────────

/// Request name row: [method badge] Request Name                    [Save]
pub fn view_request_name_row<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, EditorMsg> {
    let method = tab.method;
    let title = tab.title.clone();
    let dirty = tab.dirty;

    let name_input = text_input("Request name", &title)
        .on_input(EditorMsg::RequestNameChanged)
        .size(scale.text_label())
        .width(Length::Fill);

    let mut save_btn = button(text("Save").size(scale.text_body()))
        .padding([scale.space_sm(), scale.space_lg()])
        .style(|theme, status| styles::save_button(theme, status, 0.0));

    if dirty {
        save_btn = save_btn.on_press(EditorMsg::SaveRequest);
    }

    let content = row![
        components::method_badge(method, scale),
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
pub fn view_request_section_tabs<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, EditorMsg> {
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
) -> Element<'static, EditorMsg> {
    let is_active = tab == active;
    button(text(label).size(scale.text_body()))
        .on_press(EditorMsg::SetRequestTab(tab))
        .padding([scale.space_md(), scale.space_lg()])
        .style(move |theme, _status| styles::section_tab(theme, is_active))
        .into()
}

/// Request tab content — renders content for the active request tab
pub fn view_request_content<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, EditorMsg> {
    let content: Element<'_, EditorMsg> = match tab.active_request_tab {
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

fn view_params_editor<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, EditorMsg> {
    components::kv_editor(
        &tab.query_params,
        "Key",
        "Value",
        |i, v| EditorMsg::UpdateQueryParamKey(i, v),
        |i, v| EditorMsg::UpdateQueryParamValue(i, v),
        |i| EditorMsg::RemoveQueryParam(i),
        "+ Param",
        EditorMsg::AddQueryParam,
        scale,
    )
}

fn view_headers_editor<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, EditorMsg> {
    components::kv_editor(
        &tab.headers,
        "Header",
        "Value",
        |i, v| EditorMsg::UpdateHeaderKey(i, v),
        |i, v| EditorMsg::UpdateHeaderValue(i, v),
        |i| EditorMsg::RemoveHeader(i),
        "+ Header",
        EditorMsg::AddHeader,
        scale,
    )
}

fn view_body_editor<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, EditorMsg> {
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
                .on_press(EditorMsg::SetBodyType(bt))
                .style(move |theme, status| styles::body_type_button(theme, status, active)),
        );
    }

    let mut editor = column![selector].spacing(scale.space_sm());

    match tab.body_type {
        BodyType::None => {}
        BodyType::Raw | BodyType::Json => {
            editor = editor.push(
                text_input("Request body...", &tab.body_text)
                    .on_input(EditorMsg::UpdateBodyText)
                    .size(scale.text_body()),
            );
        }
        BodyType::Form => {
            editor = editor.push(
                components::kv_editor(
                    &tab.form_pairs,
                    "Key",
                    "Value",
                    |i, v| EditorMsg::UpdateFormKey(i, v),
                    |i, v| EditorMsg::UpdateFormValue(i, v),
                    |i| EditorMsg::RemoveFormPair(i),
                    "+ Pair",
                    EditorMsg::AddFormPair,
                    scale,
                ),
            );
        }
    }

    column![editor].into()
}

