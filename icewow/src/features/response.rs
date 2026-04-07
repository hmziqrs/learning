use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Element, Length, Task};

use crate::app::Message;
use crate::model::{AppState, ResponseTab, Tab};
use crate::ui::anim::ButtonId;
use crate::ui::{components, scale::UiScale, styles};

#[derive(Debug, Clone)]
pub enum ResponseMsg {
    SetResponseTab(ResponseTab),
    ButtonHover(ButtonId, bool),
}

pub fn update(state: &mut AppState, msg: ResponseMsg) -> Task<Message> {
    match msg {
        ResponseMsg::SetResponseTab(response_tab) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.active_response_tab = response_tab;
            }
        }
        ResponseMsg::ButtonHover(id, hovered) => {
            state
                .button_anims
                .set_hover(id, hovered, iced::time::Instant::now());
        }
    }
    Task::none()
}

// ── View functions ──────────────────────────────────────────────

/// Response section at the bottom
pub fn view_response_section<'a>(tab: &'a Tab, scale: &'a UiScale) -> Element<'a, ResponseMsg> {
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
        components::status_badge(response.status_code, scale),
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

    let response_tab_content: Element<'_, ResponseMsg> = match active {
        ResponseTab::Body => {
            let body_display = if response.body.is_empty() {
                text("(empty response)").size(scale.text_body())
            } else {
                text(response.body.clone()).size(scale.text_body())
            };
            scrollable(body_display).into()
        }
        ResponseTab::Cookies => text("No cookies").size(scale.text_body()).into(),
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
) -> Element<'static, ResponseMsg> {
    let is_active = tab == active;
    button(text(label).size(scale.text_small()))
        .on_press(ResponseMsg::SetResponseTab(tab))
        .padding(scale.pad_chip())
        .style(move |theme, _status| styles::section_tab(theme, is_active))
        .into()
}
