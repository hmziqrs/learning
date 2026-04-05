use iced::widget::{container, mouse_area, row, text};
use iced::{mouse, Element, Length, Task};

use crate::app::Message;
use crate::model::{AppState, DragState, TabId};
use crate::state::TabStore;
use crate::state::tree::NodeId;
use crate::ui::{components, icons, scale::UiScale, theme};

#[derive(Debug, Clone)]
pub enum TabsMsg {
    NewTab,
    OpenForRequest(NodeId),
    AskDeleteTab(TabId),
    SelectTab(TabId),
    BeginLongPress {
        tab_id: TabId,
        source_index: usize,
    },
    HoverIndex(usize),
    ClearHover,
}

// ── Update handler ─────────────────────────────────────────────

pub fn update(state: &mut AppState, msg: TabsMsg) -> Task<Message> {
    match msg {
        TabsMsg::NewTab => {
            state.tabs.new_tab();
        }
        TabsMsg::OpenForRequest(request_id) => {
            state.open_request_tab(request_id);
        }
        TabsMsg::AskDeleteTab(tab_id) => {
            state.delete_dialog = Some(crate::model::DeleteDialog::Tab(tab_id));
        }
        TabsMsg::SelectTab(tab_id) => {
            state.tabs.set_active(tab_id);
        }
        TabsMsg::BeginLongPress {
            tab_id,
            source_index,
        } => {
            let token = state.alloc_press_token();
            state.pending_long_press = Some(crate::model::PendingLongPress {
                token,
                kind: crate::model::PressKind::Tab {
                    tab_id,
                    source_index,
                },
                click_action: Some(crate::model::ClickAction::SelectTab(tab_id)),
            });
            state.open_context_menu = None;
            state.context_menu_position = None;

            return Task::perform(
                async move {
                    tokio::time::sleep(std::time::Duration::from_millis(220)).await;
                    token
                },
                Message::LongPressElapsed,
            );
        }
        TabsMsg::HoverIndex(index) => {
            if let Some(DragState::Tabs { hover_index, .. }) = &mut state.drag_state {
                *hover_index = Some(index);
            }
        }
        TabsMsg::ClearHover => {
            if let Some(DragState::Tabs { hover_index, .. }) = &mut state.drag_state {
                *hover_index = None;
            }
        }
    }
    Task::none()
}

// ── View functions ──────────────────────────────────────────────

pub fn view_tabs<'a>(tabs: &'a TabStore, drag_state: &'a Option<DragState>, scale: &'a UiScale) -> Element<'a, TabsMsg> {
    let mut tabs_row = row![].spacing(0).align_y(iced::Alignment::Center);

    tabs_row = tabs_row.push(tab_drop_zone(drag_state, 0));

    for (index, tab_id, tab) in tabs.ordered_enumerate() {
        let active = tabs.active_id() == Some(tab_id);

        let method_text_color = theme::method_text_color(tab.method);
        let method_label = text(tab.method.as_str()).size(11).color(method_text_color);
        let title_label = text(tab.title.clone()).size(scale.text_body());

        // Dirty indicator dot
        let dirty_dot = if tab.dirty {
            text("\u{2022}").size(8).color(theme::PRIMARY)
        } else {
            text("").size(8)
        };

        let chip_content = container(
            row![
                method_label,
                title_label,
                dirty_dot,
                components::icon_button(icons::lucide_icon("x", scale.icon_sm()), scale)
                    .padding([scale.space_xs(), scale.space_sm()])
                    .on_press(TabsMsg::AskDeleteTab(tab_id)),
            ]
            .spacing(scale.space_sm())
            .align_y(iced::Alignment::Center),
        )
        .padding(scale.pad_chip())
        .style(move |theme| crate::ui::styles::tab_chip(theme, active));

        let chip: Element<'_, TabsMsg> = mouse_area(chip_content)
            .on_press(TabsMsg::BeginLongPress {
                tab_id,
                source_index: index,
            })
            .on_enter(TabsMsg::HoverIndex(index + 1))
            .interaction(mouse::Interaction::Grab)
            .into();

        tabs_row = tabs_row.push(chip).push(tab_drop_zone(drag_state, index + 1));
    }

    let container = container(
        row![
            tabs_row.width(Length::Fill),
            components::secondary_button("+")
                .on_press(TabsMsg::NewTab)
                .padding(scale.pad_chip()),
        ]
        .spacing(scale.space_sm())
        .align_y(iced::Alignment::Center),
    )
    .padding([scale.space_sm(), scale.space_md()])
    .style(|theme| crate::ui::styles::tab_strip(theme));

    container.into()
}

fn tab_drop_zone(drag_state: &Option<DragState>, index: usize) -> Element<'static, TabsMsg> {
    let active = matches!(
        drag_state,
        Some(DragState::Tabs {
            hover_index: Some(current),
            ..
        }) if *current == index
    );

    mouse_area(
        container(text(""))
            .width(Length::Fixed(if active { 16.0 } else { 2.0 }))
            .height(Length::Fixed(28.0))
            .style(move |theme| crate::ui::styles::tab_insert(theme, active)),
    )
    .on_enter(TabsMsg::HoverIndex(index))
    .on_exit(TabsMsg::ClearHover)
    .into()
}
