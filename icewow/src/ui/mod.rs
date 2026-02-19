pub mod main_panel;
pub mod sidebar;
pub mod styles;
pub mod tabs;

use iced::widget::{button, column, container, opaque, row, text};
use iced::{Element, Length};

use crate::app::{Message, PostmanUiApp};
use crate::model::DeleteDialog;

pub fn delete_modal(app: &PostmanUiApp) -> Element<'_, Message> {
    let description = match app.state.delete_dialog {
        Some(DeleteDialog::Folder(_)) => "Delete this folder and all nested requests?",
        Some(DeleteDialog::Request(_)) => "Delete this request?",
        Some(DeleteDialog::Tab(_)) => "Close this tab?",
        None => "",
    };

    let card = container(
        column![
            text("Confirm Delete").size(20),
            text(description).size(14),
            row![
                button("Cancel")
                    .on_press(Message::CancelDelete)
                    .style(|theme, status| styles::secondary_button(theme, status)),
                button("Delete")
                    .on_press(Message::ConfirmDelete)
                    .style(|theme, status| styles::danger_button(theme, status)),
            ]
            .spacing(8),
        ]
        .spacing(14),
    )
    .padding(18)
    .width(Length::Fixed(380.0))
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
