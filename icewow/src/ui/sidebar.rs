use iced::widget::{column, container, mouse_area, row, scrollable, stack, text, Space};
use iced::{mouse, Background, Element, Length};

use crate::app::{sidebar_scroll_id, Message, PostmanUiApp};
use crate::model::{
    ClickAction, ContextMenuTarget, DragKind, DragState, FolderId, SidebarDropTarget, TreeNode,
};
use crate::ui::{components, icons, theme};

pub fn view_sidebar(app: &PostmanUiApp) -> Element<'_, Message> {
    let mut entries: Vec<Element<'_, Message>> = vec![project_row(app)];

    render_nodes(app, None, &app.state.tree_root, &[], &mut entries);

    let content = column(entries).spacing(0).padding([4, 0]);

    container(scrollable(content).id(sidebar_scroll_id()))
        .width(Length::Fixed(280.0))
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
            components::menu_button("New Folder")
                .on_press(Message::CreateFolder { parent: None })
                .into(),
            components::menu_button("New Request")
                .on_press(Message::CreateRequest { parent: None })
                .into(),
        ],
        ContextMenuTarget::Folder(folder_id) => vec![
            components::menu_button("New Folder")
                .on_press(Message::CreateFolder {
                    parent: Some(folder_id),
                })
                .into(),
            components::menu_button("New Request")
                .on_press(Message::CreateRequest {
                    parent: Some(folder_id),
                })
                .into(),
            components::danger_button("Delete Folder")
                .on_press(Message::AskDeleteFolder(folder_id))
                .into(),
        ],
        ContextMenuTarget::Request(request_id) => vec![
            components::menu_button("Open Request")
                .on_press(Message::SelectRequest(request_id))
                .into(),
            components::danger_button("Delete Request")
                .on_press(Message::AskDeleteRequest(request_id))
                .into(),
        ],
    }
}

fn project_row(app: &PostmanUiApp) -> Element<'_, Message> {
    let row = row![
        container(icons::lucide_icon("package", 14.0)).width(Length::Fixed(20.0)),
        container(text(app.state.project_name.clone()).size(15)).width(Length::Fill),
        components::icon_button(icons::lucide_icon("ellipsis", 16.0))
            .on_press(Message::ToggleContextMenu(ContextMenuTarget::ProjectRoot)),
    ]
    .spacing(4)
    .align_y(iced::Alignment::Center);

    container(row)
        .padding([4, 6])
        .width(Length::Fill)
        .style(|theme| crate::ui::styles::tree_row(theme, true, false))
        .into()
}

fn render_nodes<'a>(
    app: &'a PostmanUiApp,
    parent: Option<FolderId>,
    nodes: &'a [TreeNode],
    ancestors: &[bool],
    out: &mut Vec<Element<'a, Message>>,
) {
    let len = nodes.len();

    if nodes.is_empty() {
        out.push(drop_line(
            app,
            SidebarDropTarget::Before { parent, index: 0 },
            ancestors,
            false,
        ));
        if let Some(folder_id) = parent {
            out.push(empty_folder_state(ancestors, folder_id));
        }
        return;
    }

    for (index, node) in nodes.iter().enumerate() {
        let is_last = index == len - 1;

        out.push(drop_line(
            app,
            SidebarDropTarget::Before { parent, index },
            ancestors,
            index > 0, // pipe between items, not before first
        ));

        match node {
            TreeNode::Folder(folder) => {
                out.push(folder_row(app, parent, index, ancestors, is_last, folder));

                if folder.expanded {
                    let mut child_ancestors = ancestors.to_vec();
                    child_ancestors.push(!is_last);
                    render_nodes(
                        app,
                        Some(folder.id),
                        &folder.children,
                        &child_ancestors,
                        out,
                    );
                }
            }
            TreeNode::Request(request) => {
                out.push(request_row(app, parent, index, ancestors, is_last, request));
            }
        }

        out.push(drop_line(
            app,
            SidebarDropTarget::After { parent, index },
            ancestors,
            !is_last, // pipe between items, not after last
        ));
    }
}

fn folder_row<'a>(
    app: &'a PostmanUiApp,
    parent: Option<FolderId>,
    index: usize,
    ancestors: &[bool],
    is_last: bool,
    folder: &'a crate::model::FolderNode,
) -> Element<'a, Message> {
    let inside_target = SidebarDropTarget::InsideFolder {
        folder_id: folder.id,
        index: folder.children.len(),
    };

    let inside_active = is_sidebar_hover(app, inside_target);
    let selected = app.state.selected_folder == Some(folder.id);

    let chevron = if folder.expanded {
        icons::lucide_icon("chevron-down", 16.0)
    } else {
        icons::lucide_icon("chevron-right", 16.0)
    };

    let content = container(
        row![
            container(icons::lucide_icon("folder", 14.0).color(theme::MUTED_FOREGROUND))
                .width(Length::Fixed(18.0)),
            components::icon_button(chevron)
                .on_press(Message::ToggleFolder(folder.id)),
            container(text(folder.name.clone()).size(14)).width(Length::Fill),
            components::icon_button(icons::lucide_icon("ellipsis", 16.0))
                .on_press(Message::ToggleContextMenu(ContextMenuTarget::Folder(
                    folder.id
                ))),
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center),
    )
    .padding([3, 0])
    .width(Length::Fill);

    let mut items = item_guides(ancestors, is_last);
    items.push(content.into());

    let full_row = row(items);

    mouse_area(
        container(full_row)
            .padding([0, 6])
            .width(Length::Fill)
            .style(move |theme| crate::ui::styles::tree_row(theme, selected, inside_active)),
    )
    .on_press(Message::BeginLongPressSidebar {
        kind: DragKind::Folder(folder.id),
        source_parent: parent,
        source_index: index,
        click_action: Some(ClickAction::SelectFolder(folder.id)),
    })
    .on_enter(Message::HoverSidebarTarget(inside_target))
    .on_exit(Message::ClearSidebarHover)
    .interaction(mouse::Interaction::Grab)
    .into()
}

fn request_row<'a>(
    app: &'a PostmanUiApp,
    parent: Option<FolderId>,
    index: usize,
    ancestors: &[bool],
    is_last: bool,
    request: &'a crate::model::RequestNode,
) -> Element<'a, Message> {
    let selected = app.state.selected_folder.is_none()
        && app
            .state
            .active_tab_ref()
            .is_some_and(|tab| tab.request_id == Some(request.id));

    let method_color = theme::method_text_color(request.method);
    let method_label = text(request.method.as_str())
        .size(10)
        .color(method_color)
        .font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..iced::Font::default()
        });

    let content = container(
        row![
            container(method_label)
                .width(Length::Fixed(36.0))
                .align_x(iced::Alignment::End),
            container(text(request.name.clone()).size(14)).width(Length::Fill),
            components::icon_button(icons::lucide_icon("ellipsis", 16.0))
                .on_press(Message::ToggleContextMenu(ContextMenuTarget::Request(
                    request.id,
                ))),
        ]
        .spacing(6)
        .align_y(iced::Alignment::Center),
    )
    .padding([3, 0])
    .width(Length::Fill);

    let mut items = item_guides(ancestors, is_last);
    items.push(content.into());

    let full_row = row(items);

    mouse_area(
        container(full_row)
            .padding([0, 6])
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
    .into()
}

fn empty_folder_state(ancestors: &[bool], folder_id: FolderId) -> Element<'static, Message> {
    let hint = container(
        column![
            text("This folder is empty.")
                .size(12)
                .color(theme::MUTED_FOREGROUND),
            mouse_area(
                text("Add a request")
                    .size(12)
                    .color(theme::PRIMARY),
            )
            .on_press(Message::CreateRequest {
                parent: Some(folder_id),
            })
            .interaction(mouse::Interaction::Pointer),
        ]
        .spacing(2),
    )
    .padding([3, 0])
    .width(Length::Fill);

    let mut items = continuation_guides(ancestors);
    items.push(hint.into());

    container(row(items))
        .padding([0, 6])
        .width(Length::Fill)
        .into()
}

// ── Tree guide rendering ────────────────────────────────────

/// Guides for item rows: pass-through pipes at ancestor levels + ├─ or └─ connector.
fn item_guides<'a>(ancestors: &[bool], is_last: bool) -> Vec<Element<'a, Message>> {
    let depth = ancestors.len() + 1;
    let mut items = Vec::new();

    // Column 0: root padding (no guide at root level)
    items.push(Space::new().width(Length::Fixed(16.0)).into());

    if depth <= 1 {
        return items; // root-level items have no connectors
    }

    // Pass-through columns: 1..depth-2
    // Column c uses ancestors[c] — whether the parent at that depth has more siblings.
    for c in 1..depth.saturating_sub(1) {
        if c < ancestors.len() && ancestors[c] {
            items.push(pipe_guide());
        } else {
            items.push(Space::new().width(Length::Fixed(16.0)).into());
        }
    }

    // Connector column
    if is_last {
        items.push(corner_guide());
    } else {
        items.push(tee_guide());
    }

    items
}

/// Guides for drop lines and empty-folder hints: pipes only, no connector.
fn continuation_guides<'a>(ancestors: &[bool]) -> Vec<Element<'a, Message>> {
    let depth = ancestors.len() + 1;
    let mut items = Vec::new();

    items.push(Space::new().width(Length::Fixed(16.0)).into());

    for c in 1..depth {
        if c < ancestors.len() && ancestors[c] {
            items.push(pipe_guide());
        } else {
            items.push(Space::new().width(Length::Fixed(16.0)).into());
        }
    }

    items
}

/// Guides for drop lines: continuation pipes + optional pipe at the connector column.
fn drop_line_guides<'a>(ancestors: &[bool], pipe_at_connector: bool) -> Vec<Element<'a, Message>> {
    let depth = ancestors.len() + 1;
    let mut items = Vec::new();

    items.push(Space::new().width(Length::Fixed(16.0)).into());

    if depth <= 1 {
        return items;
    }

    // Pass-through columns
    for c in 1..depth.saturating_sub(1) {
        if c < ancestors.len() && ancestors[c] {
            items.push(pipe_guide());
        } else {
            items.push(Space::new().width(Length::Fixed(16.0)).into());
        }
    }

    // Connector column: pipe if between siblings, else empty
    if pipe_at_connector {
        items.push(pipe_guide());
    } else {
        items.push(Space::new().width(Length::Fixed(16.0)).into());
    }

    items
}

/// │ — vertical line running full height of the row.
fn pipe_guide<'a>() -> Element<'a, Message> {
    row![
        container(Space::new())
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(guide_style),
        Space::new().width(Length::Fixed(15.0)),
    ]
    .width(Length::Fixed(16.0))
    .height(Length::Fill)
    .into()
}

/// ├─ — vertical line full height + horizontal branch at center.
fn tee_guide<'a>() -> Element<'a, Message> {
    let top = row![
        container(Space::new())
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(guide_style),
        Space::new().width(Length::Fixed(15.0)),
    ]
    .height(Length::Fill);

    let mid = container(Space::new())
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(1.0))
        .style(guide_style);

    let bot = row![
        container(Space::new())
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(guide_style),
        Space::new().width(Length::Fixed(15.0)),
    ]
    .height(Length::Fill);

    column![top, mid, bot]
        .width(Length::Fixed(16.0))
        .height(Length::Fill)
        .into()
}

/// └─ — vertical line top half + horizontal branch at center.
fn corner_guide<'a>() -> Element<'a, Message> {
    let top = row![
        container(Space::new())
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(guide_style),
        Space::new().width(Length::Fixed(15.0)),
    ]
    .height(Length::Fill);

    let mid = container(Space::new())
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(1.0))
        .style(guide_style);

    let bot = Space::new()
        .width(Length::Fixed(16.0))
        .height(Length::Fill);

    column![top, mid, bot]
        .width(Length::Fixed(16.0))
        .height(Length::Fill)
        .into()
}

fn guide_style(_: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme::WHITE_10)),
        ..container::Style::default()
    }
}

// ── Drop line ───────────────────────────────────────────────

fn drop_line<'a>(
    app: &'a PostmanUiApp,
    target: SidebarDropTarget,
    ancestors: &[bool],
    pipe_at_connector: bool,
) -> Element<'a, Message> {
    let active = is_sidebar_hover(app, target);

    let line_bar = container(text(""))
        .height(Length::Fixed(if active { 22.0 } else { 2.0 }))
        .width(Length::Fill)
        .style(move |theme| crate::ui::styles::drop_line(theme, active));

    let mut items = drop_line_guides(ancestors, pipe_at_connector);
    items.push(line_bar.into());

    mouse_area(
        container(row(items))
            .padding([0, 6])
            .width(Length::Fill),
    )
    .on_enter(Message::HoverSidebarTarget(target))
    .on_exit(Message::ClearSidebarHover)
    .into()
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
