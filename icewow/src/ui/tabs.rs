use iced::widget::{button, container, mouse_area, row, text};
use iced::{mouse, Element, Length};

use crate::app::{Message, PostmanUiApp};
use crate::model::DragState;

pub fn view_tabs(app: &PostmanUiApp) -> Element<'_, Message> {
    let mut tabs_row = row![].spacing(4).align_y(iced::Alignment::Center);

    tabs_row = tabs_row.push(tab_drop_zone(app, 0));

    for (index, tab) in app.state.tabs.iter().enumerate() {
        let active = app.state.active_tab == Some(tab.id);

        let chip_content = container(
            row![
                container(text(tab.title.clone()).size(14)).width(Length::Fill),
                button("×")
                    .padding([4, 6])
                    .on_press(Message::AskDeleteTab(tab.id))
                    .style(|theme, status| crate::ui::styles::handle_button(theme, status)),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center),
        )
        .padding([6, 8])
        .style(move |theme| crate::ui::styles::tab_chip(theme, active));

        let chip: Element<'_, Message> = mouse_area(chip_content)
            .on_press(Message::BeginLongPressTab {
                tab_id: tab.id,
                source_index: index,
            })
            .on_enter(Message::HoverTabIndex(index + 1))
            .interaction(mouse::Interaction::Grab)
            .into();

        tabs_row = tabs_row.push(chip).push(tab_drop_zone(app, index + 1));
    }

    let container = container(
        row![
            tabs_row.width(Length::Fill),
            button("+")
                .on_press(Message::NewTab)
                .padding([6, 10])
                .style(|theme, status| crate::ui::styles::secondary_button(theme, status)),
        ]
        .spacing(6)
        .align_y(iced::Alignment::Center),
    )
    .padding(8)
    .style(|theme| crate::ui::styles::tab_strip(theme));

    container.into()
}

fn tab_drop_zone(app: &PostmanUiApp, index: usize) -> Element<'_, Message> {
    let active = matches!(
        app.state.drag_state,
        Some(DragState::Tabs {
            hover_index: Some(current),
            ..
        }) if current == index
    );

    mouse_area(
        container(text(""))
            .width(Length::Fixed(10.0))
            .height(Length::Fixed(30.0))
            .style(move |theme| crate::ui::styles::tab_insert(theme, active)),
    )
    .on_enter(Message::HoverTabIndex(index))
    .on_exit(Message::ClearTabHover)
    .into()
}
