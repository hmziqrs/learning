pub mod components;
pub mod icons;
pub mod main_panel;
pub mod scale;
pub mod sidebar;
pub mod styles;
pub mod tabs;
pub mod theme;

use iced::widget::{column, container, opaque, row, text, Space};
use iced::{Element, Length};

use crate::app::{Message, PostmanUiApp};
use crate::model::DeleteDialog;
use crate::ui::scale::UiScale;

pub fn delete_modal(app: &PostmanUiApp) -> Element<'_, Message> {
    let scale = &app.state.ui_scale;
    let description = match app.state.delete_dialog {
        Some(DeleteDialog::Folder(_)) => "Delete this folder and all nested requests?",
        Some(DeleteDialog::Request(_)) => "Delete this request?",
        Some(DeleteDialog::Tab(_)) => "Close this tab?",
        None => "",
    };

    let card = container(
        column![
            text("Confirm Delete").size(scale.text_heading()),
            text(description).size(scale.text_label()),
            row![
                components::secondary_button("Cancel")
                    .on_press(Message::CancelDelete),
                components::danger_button("Delete")
                    .on_press(Message::ConfirmDelete),
            ]
            .spacing(scale.space_md()),
        ]
        .spacing(14),
    )
    .padding(18)
    .width(Length::Fixed(UiScale::MODAL_WIDTH))
    .style(|theme| styles::modal_card(theme));

    opaque(
        container(card)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme| styles::modal_backdrop(theme)),
    )
}

pub fn drag_preview_overlay(app: &PostmanUiApp) -> Option<Element<'_, Message>> {
    let scale = &app.state.ui_scale;
    let label = app.drag_preview_text()?;
    let pos = app.state.pointer_position;

    let x = (pos.x + scale.icon_sm()).clamp(8.0, (app.state.window_size.width - 260.0).max(8.0));
    let y = (pos.y + scale.icon_sm()).clamp(8.0, (app.state.window_size.height - 80.0).max(8.0));

    let card = container(text(label).size(scale.text_label()))
        .padding([scale.space_md(), scale.space_lg()])
        .width(Length::Fixed(240.0))
        .style(|theme| styles::drag_preview(theme));

    Some(
        container(
            column![
                Space::new().height(Length::Fixed(y)),
                row![Space::new().width(Length::Fixed(x)), card].align_y(iced::Alignment::Start),
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),
    )
}
