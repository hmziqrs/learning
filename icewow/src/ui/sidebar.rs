use iced::mouse;
use iced::widget::{button, column, container, mouse_area, row, scrollable, text, Space};
use iced::{Element, Length};

use crate::app::{Message, PostmanUiApp};
use crate::model::{
    ContextMenuTarget, DragKind, DragState, FolderId, SidebarDropTarget, TreeNode,
};

pub fn view_sidebar(app: &PostmanUiApp) -> Element<'_, Message> {
    let mut entries: Vec<Element<'_, Message>> = vec![project_row(app)];

    if app.state.open_context_menu == Some(ContextMenuTarget::ProjectRoot) {
        entries.push(project_menu());
    }

    render_nodes(app, None, &app.state.tree_root, 1, &mut entries);

    let content = column(entries).spacing(4).padding(8);

    container(scrollable(content))
        .width(Length::Fixed(320.0))
        .height(Length::Fill)
        .style(|theme| crate::ui::styles::sidebar_panel(theme))
        .into()
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

fn project_menu() -> Element<'static, Message> {
    context_menu(
        vec![
            button("New Folder")
                .on_press(Message::CreateFolder { parent: None })
                .style(|theme, status| crate::ui::styles::menu_button(theme, status))
                .into(),
            button("New Request")
                .on_press(Message::CreateRequest { parent: None })
                .style(|theme, status| crate::ui::styles::menu_button(theme, status))
                .into(),
        ],
        1,
    )
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

                if app.state.open_context_menu == Some(ContextMenuTarget::Folder(folder.id)) {
                    out.push(folder_menu(folder.id, depth + 1));
                }

                if folder.expanded {
                    render_nodes(app, Some(folder.id), &folder.children, depth + 1, out);
                }
            }
            TreeNode::Request(request) => {
                out.push(request_row(app, parent, index, depth, request));

                if app.state.open_context_menu == Some(ContextMenuTarget::Request(request.id)) {
                    out.push(request_menu(request.id, depth + 1));
                }
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
        drag_handle(Message::StartDragSidebar(
            DragKind::Folder(folder.id),
            parent,
            index,
        )),
        button(if folder.expanded { "▾" } else { "▸" })
            .padding([2, 6])
            .on_press(Message::ToggleFolder(folder.id))
            .style(|theme, status| crate::ui::styles::handle_button(theme, status)),
        container(text(folder.name.clone()).size(14)).width(Length::Fill),
        button("⋯")
            .padding([2, 6])
            .on_press(Message::ToggleContextMenu(ContextMenuTarget::Folder(folder.id)))
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
    .on_enter(Message::HoverSidebarTarget(inside_target))
    .on_exit(Message::ClearSidebarHover)
    .into();

    indent(depth, inner)
}

fn folder_menu(folder_id: FolderId, depth: u16) -> Element<'static, Message> {
    context_menu(
        vec![
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
        depth,
    )
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
        drag_handle(Message::StartDragSidebar(
            DragKind::Request(request.id),
            parent,
            index,
        )),
        container(text(" ")).width(Length::Fixed(18.0)),
        button(text(request.name.clone()).size(14))
            .on_press(Message::SelectRequest(request.id))
            .padding([2, 6])
            .width(Length::Fill)
            .style(move |theme, status| {
                if selected {
                    crate::ui::styles::secondary_button(theme, status)
                } else {
                    crate::ui::styles::handle_button(theme, status)
                }
            }),
        button("⋯")
            .padding([2, 6])
            .on_press(Message::ToggleContextMenu(ContextMenuTarget::Request(request.id)))
            .style(|theme, status| crate::ui::styles::handle_button(theme, status)),
    ]
    .spacing(4)
    .align_y(iced::Alignment::Center);

    let inner: Element<'a, Message> = container(row)
        .padding([4, 6])
        .width(Length::Fill)
        .style(move |theme| crate::ui::styles::tree_row(theme, selected, false))
        .into();

    indent(depth, inner)
}

fn request_menu(request_id: u64, depth: u16) -> Element<'static, Message> {
    context_menu(
        vec![
            button("Open Request")
                .on_press(Message::SelectRequest(request_id))
                .style(|theme, status| crate::ui::styles::menu_button(theme, status))
                .into(),
            button("Delete Request")
                .on_press(Message::AskDeleteRequest(request_id))
                .style(|theme, status| crate::ui::styles::danger_button(theme, status))
                .into(),
        ],
        depth,
    )
}

fn context_menu<'a>(items: Vec<Element<'a, Message>>, depth: u16) -> Element<'a, Message> {
    let menu = container(column(items).spacing(4))
        .padding(6)
        .width(Length::Shrink)
        .style(|theme| crate::ui::styles::context_menu(theme))
        .into();

    indent(depth, menu)
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

fn drag_handle<'a>(start: Message) -> Element<'a, Message> {
    mouse_area(
        container(text("⋮⋮").size(13))
            .padding([3, 6])
            .style(|theme| crate::ui::styles::drag_handle(theme)),
    )
    .on_press(start)
    .interaction(mouse::Interaction::Grab)
    .into()
}
