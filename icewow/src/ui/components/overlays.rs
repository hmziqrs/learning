use iced::widget::{column, container, opaque, row, text, Space};
use iced::{Element, Length};

use crate::app::Message;
use crate::model::{DeleteDialog, DragKind, DragState};
use crate::ui::scale::UiScale;
use crate::ui::{components, styles};

/// Delete confirmation modal overlay.
pub fn delete_modal<'a>(dialog: &'a DeleteDialog, scale: &'a UiScale) -> Element<'a, Message> {
    let description = match dialog {
        DeleteDialog::Folder(_) => "Delete this folder and all nested requests?",
        DeleteDialog::Request(_) => "Delete this request?",
        DeleteDialog::Tab(_) => "Close this tab?",
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
    .style(|theme| styles::modal_card(theme, scale));

    opaque(
        container(card)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme| styles::modal_backdrop(theme)),
    )
}

/// Drag preview overlay — shows a floating label near the cursor during drag.
pub fn drag_preview_overlay(
    drag_state: &Option<DragState>,
    tree: &crate::state::TreeArena,
    tabs: &crate::state::TabStore,
    pointer: iced::Point,
    window_size: iced::Size,
    scale: &UiScale,
) -> Option<Element<'static, Message>> {
    let label = drag_preview_text(drag_state, tree, tabs)?;
    let x = (pointer.x + scale.icon_sm()).clamp(8.0, (window_size.width - 260.0).max(8.0));
    let y = (pointer.y + scale.icon_sm()).clamp(8.0, (window_size.height - 80.0).max(8.0));

    let drag_blur = scale.space_xl();
    let card = container(text(label).size(scale.text_label()))
        .padding([scale.space_md(), scale.space_lg()])
        .width(Length::Fixed(240.0))
        .style(move |theme| styles::drag_preview(theme, drag_blur));

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

fn drag_preview_text(
    drag_state: &Option<DragState>,
    tree: &crate::state::TreeArena,
    tabs: &crate::state::TabStore,
) -> Option<String> {
    match drag_state {
        Some(DragState::Sidebar {
            kind: DragKind::Folder(folder_id),
            ..
        }) => tree
            .folder_name(*folder_id)
            .map(|name| format!("Folder: {name}")),
        Some(DragState::Sidebar {
            kind: DragKind::Request(request_id),
            ..
        }) => tree.get(*request_id).and_then(|e| match &e.data {
            crate::state::tree::NodeData::Request { name, .. } => {
                Some(format!("Request: {name}"))
            }
            _ => None,
        }),
        Some(DragState::Tabs { tab_id, .. }) => tabs
            .get(*tab_id)
            .map(|tab| format!("Tab: {}", tab.title)),
        None => None,
    }
}
