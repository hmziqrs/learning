use iced::widget::{button, column, container, mouse_area, row, scrollable, stack, text, Space};
use iced::{mouse, Element, Length};

use crate::app::{sidebar_scroll_id, Message, PostmanUiApp};
use crate::model::{
    ClickAction, ContextMenuTarget, DragKind, DragState, FolderId, SidebarDropTarget, TreeNode,
};

pub fn view_sidebar(app: &PostmanUiApp) -> Element<'_, Message> {
    let mut entries: Vec<Element<'_, Message>> = vec![project_row(app)];

    render_nodes(app, None, &app.state.tree_root, 1, &mut entries);

    let content = column(entries).spacing(4).padding(8);

    container(scrollable(content).id(sidebar_scroll_id()))
        .width(Length::Fixed(320.0))
        .height(Length::Fill)
        .style(|theme| crate::ui::styles::sidebar_panel(theme))
        .into()
}

pub fn view_context_menu_overlay(app: &PostmanUiApp) -> Option<Element<'_, Message>> {
    let target = app.state.open_context_menu?;

    let pos = app
        .state
        .context_menu_position
        .unwrap_or(app.state.pointer_position);

    let max_x = (app.state.window_size.width - 220.0).max(8.0);
    let max_y = (app.state.window_size.height - 220.0).max(8.0);

    let x = pos.x.clamp(8.0, max_x);
    let y = pos.y.clamp(8.0, max_y);

    let menu_items = menu_items(target);

    let menu = container(column(menu_items).spacing(4))
        .padding(6)
        .width(Length::Fixed(210.0))
        .style(|theme| crate::ui::styles::context_menu(theme));

    let dismiss_layer: Element<'_, Message> =
        mouse_area(container(text("")).width(Length::Fill).height(Length::Fill))
            .on_press(Message::CloseContextMenu)
            .into();

    let position_layer: Element<'_, Message> = container(
        column![
            Space::new().height(Length::Fixed(y)),
            row![Space::new().width(Length::Fixed(x)), menu].align_y(iced::Alignment::Start),
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into();

    Some(stack([dismiss_layer, position_layer]).into())
}

fn menu_items(target: ContextMenuTarget) -> Vec<Element<'static, Message>> {
    match target {
        ContextMenuTarget::ProjectRoot => vec![
            button("New Folder")
                .on_press(Message::CreateFolder { parent: None })
                .style(|theme, status| crate::ui::styles::menu_button(theme, status))
                .into(),
            button("New Request")
                .on_press(Message::CreateRequest { parent: None })
                .style(|theme, status| crate::ui::styles::menu_button(theme, status))
                .into(),
        ],
        ContextMenuTarget::Folder(folder_id) => vec![
            button("New Folder")
                .on_press(Message::CreateFolder {
                    parent: Some(folder_id),
                })
                .style(|theme, status| crate::ui::styles::menu_button(theme, status))
                .into(),
            button("New Request")
                .on_press(Message::CreateRequest {
                    parent: Some(folder_id),
                })
                .style(|theme, status| crate::ui::styles::menu_button(theme, status))
                .into(),
            button("Delete Folder")
                .on_press(Message::AskDeleteFolder(folder_id))
                .style(|theme, status| crate::ui::styles::danger_button(theme, status))
                .into(),
        ],
        ContextMenuTarget::Request(request_id) => vec![
            button("Open Request")
                .on_press(Message::SelectRequest(request_id))
                .style(|theme, status| crate::ui::styles::menu_button(theme, status))
                .into(),
            button("Delete Request")
                .on_press(Message::AskDeleteRequest(request_id))
                .style(|theme, status| crate::ui::styles::danger_button(theme, status))
                .into(),
        ],
    }
}

fn project_row(app: &PostmanUiApp) -> Element<'_, Message> {
    let row = row![
        container(text("📦").size(14)).width(Length::Fixed(20.0)),
        container(text(app.state.project_name.clone()).size(15)).width(Length::Fill),
        button("⋯")
            .padding([2, 6])
            .on_press(Message::ToggleContextMenu(ContextMenuTarget::ProjectRoot))
            .style(|theme, status| crate::ui::styles::handle_button(theme, status)),
    ]
    .spacing(4)
    .align_y(iced::Alignment::Center);

    container(row)
        .padding([6, 6])
        .width(Length::Fill)
        .style(|theme| crate::ui::styles::tree_row(theme, true, false))
        .into()
}

fn render_nodes<'a>(
    app: &'a PostmanUiApp,
    parent: Option<FolderId>,
    nodes: &'a [TreeNode],
    depth: u16,
    out: &mut Vec<Element<'a, Message>>,
) {
    if nodes.is_empty() {
        out.push(drop_line(
            app,
            SidebarDropTarget::Before { parent, index: 0 },
            depth,
        ));
        return;
    }

    for (index, node) in nodes.iter().enumerate() {
        out.push(drop_line(
            app,
            SidebarDropTarget::Before { parent, index },
            depth,
        ));

        match node {
            TreeNode::Folder(folder) => {
                out.push(folder_row(app, parent, index, depth, folder));

                if folder.expanded {
                    render_nodes(app, Some(folder.id), &folder.children, depth + 1, out);
                }
            }
            TreeNode::Request(request) => {
                out.push(request_row(app, parent, index, depth, request));
            }
        }

        out.push(drop_line(
            app,
            SidebarDropTarget::After { parent, index },
            depth,
        ));
    }
}

fn folder_row<'a>(
    app: &'a PostmanUiApp,
    parent: Option<FolderId>,
    index: usize,
    depth: u16,
    folder: &'a crate::model::FolderNode,
) -> Element<'a, Message> {
    let inside_target = SidebarDropTarget::InsideFolder {
        folder_id: folder.id,
        index: folder.children.len(),
    };

    let inside_active = is_sidebar_hover(app, inside_target);

    let row = row![
        button(if folder.expanded { "▾" } else { "▸" })
            .padding([2, 6])
            .on_press(Message::ToggleFolder(folder.id))
            .style(|theme, status| crate::ui::styles::handle_button(theme, status)),
        container(text(folder.name.clone()).size(14)).width(Length::Fill),
        button("⋯")
            .padding([2, 6])
            .on_press(Message::ToggleContextMenu(ContextMenuTarget::Folder(
                folder.id
            )))
            .style(|theme, status| crate::ui::styles::handle_button(theme, status)),
    ]
    .spacing(4)
    .align_y(iced::Alignment::Center);

    let inner: Element<'a, Message> = mouse_area(
        container(row)
            .padding([4, 6])
            .width(Length::Fill)
            .style(move |theme| crate::ui::styles::tree_row(theme, false, inside_active)),
    )
    .on_press(Message::BeginLongPressSidebar {
        kind: DragKind::Folder(folder.id),
        source_parent: parent,
        source_index: index,
        click_action: None,
    })
    .on_enter(Message::HoverSidebarTarget(inside_target))
    .on_exit(Message::ClearSidebarHover)
    .interaction(mouse::Interaction::Grab)
    .into();

    indent(depth, inner)
}

fn request_row<'a>(
    app: &'a PostmanUiApp,
    parent: Option<FolderId>,
    index: usize,
    depth: u16,
    request: &'a crate::model::RequestNode,
) -> Element<'a, Message> {
    let selected = app
        .state
        .active_tab_ref()
        .is_some_and(|tab| tab.request_id == Some(request.id));

    let row = row![
        container(text("•").size(14)).width(Length::Fixed(18.0)),
        container(text(request.name.clone()).size(14)).width(Length::Fill),
        button("⋯")
            .padding([2, 6])
            .on_press(Message::ToggleContextMenu(ContextMenuTarget::Request(
                request.id,
            )))
            .style(|theme, status| crate::ui::styles::handle_button(theme, status)),
    ]
    .spacing(4)
    .align_y(iced::Alignment::Center);

    let inner: Element<'a, Message> = mouse_area(
        container(row)
            .padding([4, 6])
            .width(Length::Fill)
            .style(move |theme| crate::ui::styles::tree_row(theme, selected, false)),
    )
    .on_press(Message::BeginLongPressSidebar {
        kind: DragKind::Request(request.id),
        source_parent: parent,
        source_index: index,
        click_action: Some(ClickAction::SelectRequest(request.id)),
    })
    .interaction(mouse::Interaction::Grab)
    .into();

    indent(depth, inner)
}

fn indent<'a>(depth: u16, inner: Element<'a, Message>) -> Element<'a, Message> {
    row![
        Space::new().width(Length::Fixed((depth as f32) * 16.0)),
        inner
    ]
    .align_y(iced::Alignment::Center)
    .into()
}

fn drop_line(app: &PostmanUiApp, target: SidebarDropTarget, depth: u16) -> Element<'_, Message> {
    let active = is_sidebar_hover(app, target);

    let line: Element<'_, Message> = mouse_area(
        container(text(""))
            .height(Length::Fixed(4.0))
            .width(Length::Fill)
            .style(move |theme| crate::ui::styles::drop_line(theme, active)),
    )
    .on_enter(Message::HoverSidebarTarget(target))
    .on_exit(Message::ClearSidebarHover)
    .into();

    indent(depth, line)
}

fn is_sidebar_hover(app: &PostmanUiApp, target: SidebarDropTarget) -> bool {
    matches!(
        app.state.drag_state,
        Some(DragState::Sidebar {
            hover: Some(current),
            ..
        }) if current == target
    )
}
