use iced::widget::{column, container, mouse_area, row, scrollable, stack, text, Space};
use iced::{mouse, Background, Element, Length, Task};

use crate::app::{sidebar_scroll_id, Message};
use crate::model::{
    AppState, ClickAction, ContextMenuTarget, DragKind, DragState, HttpMethod,
    PendingLongPress, PressKind, SidebarDropTarget,
};
use crate::state::tree::{NodeData, NodeId};
use crate::state::TabStore;
use crate::state::TreeArena;
use crate::ui::{components, icons, scale::UiScale, theme};

#[derive(Debug, Clone)]
pub enum SidebarMsg {
    ToggleContextMenu(ContextMenuTarget),
    CloseContextMenu,
    CreateFolder {
        parent: Option<NodeId>,
    },
    CreateRequest {
        parent: Option<NodeId>,
    },
    AskDeleteFolder(NodeId),
    AskDeleteRequest(NodeId),
    SelectRequest(NodeId),
    ToggleFolder(NodeId),
    BeginLongPress {
        kind: DragKind,
        source_parent: Option<NodeId>,
        source_index: usize,
        click_action: Option<ClickAction>,
    },
    HoverTarget(SidebarDropTarget),
    ClearHover,
}

// ── Update handler ─────────────────────────────────────────────

pub fn update(state: &mut AppState, msg: SidebarMsg) -> Task<Message> {
    match msg {
        SidebarMsg::ToggleContextMenu(target) => {
            if state.open_context_menu == Some(target) {
                state.open_context_menu = None;
                state.context_menu_position = None;
            } else {
                state.open_context_menu = Some(target);
                state.context_menu_position = Some(state.pointer_position);
            }
        }
        SidebarMsg::CloseContextMenu => {
            state.open_context_menu = None;
            state.context_menu_position = None;
        }
        SidebarMsg::CreateFolder { parent } => {
            let id = state.tree.alloc_folder_id();
            let name = format!("New Folder {id}");
            let index = state.tree.children_len(parent);
            state.tree.insert(
                parent,
                index,
                id,
                NodeData::Folder { name, expanded: true },
            );
            state.open_context_menu = None;
            state.context_menu_position = None;
        }
        SidebarMsg::CreateRequest { parent } => {
            let id = state.tree.alloc_request_id();
            let name = format!("New Request {id}");
            let index = state.tree.children_len(parent);
            state.tree.insert(
                parent,
                index,
                id,
                NodeData::Request {
                    name,
                    url: "https://api.example.com/new".to_string(),
                    method: HttpMethod::Get,
                },
            );
            state.open_context_menu = None;
            state.context_menu_position = None;
        }
        SidebarMsg::AskDeleteFolder(folder_id) => {
            state.delete_dialog = Some(crate::model::DeleteDialog::Folder(folder_id));
            state.open_context_menu = None;
            state.context_menu_position = None;
        }
        SidebarMsg::AskDeleteRequest(request_id) => {
            state.delete_dialog = Some(crate::model::DeleteDialog::Request(request_id));
            state.open_context_menu = None;
            state.context_menu_position = None;
        }
        SidebarMsg::SelectRequest(request_id) => {
            state.open_context_menu = None;
            state.context_menu_position = None;
            return Task::done(Message::Tabs(crate::features::TabsMsg::OpenForRequest(
                request_id,
            )));
        }
        SidebarMsg::ToggleFolder(folder_id) => {
            if let Some(current) = state.tree.is_expanded(folder_id) {
                state.tree.set_expanded(folder_id, !current);
            }
        }
        SidebarMsg::BeginLongPress {
            kind,
            source_parent,
            source_index,
            click_action,
        } => {
            let token = state.alloc_press_token();
            state.pending_long_press = Some(PendingLongPress {
                token,
                kind: PressKind::Sidebar {
                    kind,
                    source_parent,
                    source_index,
                },
                click_action,
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
        SidebarMsg::HoverTarget(target) => {
            if let Some(DragState::Sidebar { hover, .. }) = &mut state.drag_state {
                *hover = Some(target);
            }
        }
        SidebarMsg::ClearHover => {
            if let Some(DragState::Sidebar { hover, .. }) = &mut state.drag_state {
                *hover = None;
            }
        }
    }
    Task::none()
}

// ── View functions ──────────────────────────────────────────────

pub fn view_sidebar(state: &AppState) -> Element<'_, Message> {
    let scale = &state.ui_scale;
    let mut entries: Vec<Element<'_, Message>> = vec![project_row(
        &state.project_name,
        scale,
    )];

    render_nodes(
        &state.tree,
        &state.tabs,
        &state.drag_state,
        &state.selected_folder,
        None,
        state.tree.root_children(),
        &[],
        scale,
        &mut entries,
    );

    let content = column(entries).spacing(0).padding([scale.space_sm(), 0.0]);

    container(scrollable(content).id(sidebar_scroll_id()))
        .width(Length::Fixed(UiScale::SIDEBAR_WIDTH))
        .height(Length::Fill)
        .style(|theme| crate::ui::styles::sidebar_panel(theme))
        .into()
}

pub fn view_context_menu_overlay(
    target: &ContextMenuTarget,
    context_menu_position: &Option<iced::Point>,
    pointer_position: iced::Point,
    window_size: iced::Size,
    scale: &UiScale,
) -> Option<Element<'static, Message>> {
    let pos = context_menu_position.unwrap_or(pointer_position);

    let max_x = (window_size.width - 220.0).max(8.0);
    let max_y = (window_size.height - 220.0).max(8.0);

    let x = pos.x.clamp(8.0, max_x);
    let y = pos.y.clamp(8.0, max_y);

    let menu_items = menu_items(target);

    let menu = container(column(menu_items).spacing(scale.space_sm()))
        .padding(scale.space_sm())
        .width(Length::Fixed(UiScale::CONTEXT_MENU_WIDTH))
        .style(|theme| crate::ui::styles::context_menu(theme));

    let dismiss_layer: Element<'static, Message> =
        mouse_area(container(text("")).width(Length::Fill).height(Length::Fill))
            .on_press(Message::Sidebar(SidebarMsg::CloseContextMenu))
            .into();

    let position_layer: Element<'static, Message> = container(
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

fn menu_items(target: &ContextMenuTarget) -> Vec<Element<'static, Message>> {
    match target {
        ContextMenuTarget::ProjectRoot => vec![
            components::menu_button("New Folder")
                .on_press(Message::Sidebar(SidebarMsg::CreateFolder { parent: None }))
                .into(),
            components::menu_button("New Request")
                .on_press(Message::Sidebar(SidebarMsg::CreateRequest { parent: None }))
                .into(),
        ],
        ContextMenuTarget::Folder(folder_id) => vec![
            components::menu_button("New Folder")
                .on_press(Message::Sidebar(SidebarMsg::CreateFolder {
                    parent: Some(*folder_id),
                }))
                .into(),
            components::menu_button("New Request")
                .on_press(Message::Sidebar(SidebarMsg::CreateRequest {
                    parent: Some(*folder_id),
                }))
                .into(),
            components::danger_button("Delete Folder")
                .on_press(Message::Sidebar(SidebarMsg::AskDeleteFolder(*folder_id)))
                .into(),
        ],
        ContextMenuTarget::Request(request_id) => vec![
            components::menu_button("Open Request")
                .on_press(Message::Sidebar(SidebarMsg::SelectRequest(*request_id)))
                .into(),
            components::danger_button("Delete Request")
                .on_press(Message::Sidebar(SidebarMsg::AskDeleteRequest(*request_id)))
                .into(),
        ],
    }
}

fn project_row<'a>(project_name: &'a str, scale: &UiScale) -> Element<'a, Message> {
    let row = row![
        container(icons::lucide_icon("package", scale.icon_sm())).width(Length::Fixed(20.0)),
        container(text(project_name).size(scale.text_label())).width(Length::Fill),
        components::icon_button(icons::lucide_icon("ellipsis", scale.icon_md()), scale)
            .on_press(Message::Sidebar(SidebarMsg::ToggleContextMenu(
                ContextMenuTarget::ProjectRoot,
            ))),
    ]
    .spacing(scale.space_sm())
    .align_y(iced::Alignment::Center);

    container(row)
        .padding([scale.space_sm(), scale.space_sm()])
        .width(Length::Fill)
        .style(|theme| crate::ui::styles::tree_row(theme, true, false))
        .into()
}

#[allow(clippy::too_many_arguments)]
fn render_nodes<'a>(
    tree: &'a TreeArena,
    tabs: &'a TabStore,
    drag_state: &'a Option<DragState>,
    selected_folder: &'a Option<NodeId>,
    parent: Option<NodeId>,
    node_ids: &'a [NodeId],
    ancestors: &[bool],
    scale: &UiScale,
    out: &mut Vec<Element<'a, Message>>,
) {
    let len = node_ids.len();

    if node_ids.is_empty() {
        out.push(drop_line(
            drag_state,
            SidebarDropTarget::Before { parent, index: 0 },
            ancestors,
            false,
        ));
        if let Some(folder_id) = parent {
            out.push(empty_folder_state(ancestors, folder_id));
        }
        return;
    }

    for (index, &node_id) in node_ids.iter().enumerate() {
        let is_last = index == len - 1;
        let entry = match tree.get(node_id) {
            Some(e) => e,
            None => continue,
        };

        out.push(drop_line(
            drag_state,
            SidebarDropTarget::Before { parent, index },
            ancestors,
            index > 0,
        ));

        match &entry.data {
            NodeData::Folder {
                name,
                expanded,
            } => {
                out.push(folder_row(
                    drag_state,
                    selected_folder,
                    scale,
                    parent,
                    index,
                    ancestors,
                    is_last,
                    node_id,
                    name,
                    *expanded,
                    &entry.children,
                ));
                if *expanded {
                    let mut child_ancestors = ancestors.to_vec();
                    child_ancestors.push(!is_last);
                    render_nodes(
                        tree,
                        tabs,
                        drag_state,
                        selected_folder,
                        Some(node_id),
                        &entry.children,
                        &child_ancestors,
                        scale,
                        out,
                    );
                }
            }
            NodeData::Request { name, method, .. } => {
                out.push(request_row(
                    tabs,
                    drag_state,
                    selected_folder,
                    scale,
                    parent,
                    index,
                    ancestors,
                    is_last,
                    node_id,
                    name,
                    *method,
                ));
            }
        }

        out.push(drop_line(
            drag_state,
            SidebarDropTarget::After { parent, index },
            ancestors,
            !is_last,
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn folder_row<'a>(
    drag_state: &'a Option<DragState>,
    selected_folder: &'a Option<NodeId>,
    scale: &UiScale,
    parent: Option<NodeId>,
    index: usize,
    ancestors: &[bool],
    is_last: bool,
    folder_id: NodeId,
    folder_name: &'a str,
    expanded: bool,
    children: &[NodeId],
) -> Element<'a, Message> {
    let inside_target = SidebarDropTarget::InsideFolder {
        folder_id,
        index: children.len(),
    };

    let inside_active = is_sidebar_hover(drag_state, inside_target);
    let selected = *selected_folder == Some(folder_id);

    let chevron = if expanded {
        icons::lucide_icon("chevron-down", scale.icon_md())
    } else {
        icons::lucide_icon("chevron-right", scale.icon_md())
    };

    let content = container(
        row![
            container(icons::lucide_icon("folder", scale.icon_sm()).color(theme::MUTED_FOREGROUND))
                .width(Length::Fixed(18.0)),
            components::icon_button(chevron, scale)
                .on_press(Message::Sidebar(SidebarMsg::ToggleFolder(folder_id))),
            container(text(folder_name).size(scale.text_label())).width(Length::Fill),
            components::icon_button(icons::lucide_icon("ellipsis", scale.icon_md()), scale)
                .on_press(Message::Sidebar(SidebarMsg::ToggleContextMenu(
                    ContextMenuTarget::Folder(folder_id)
                ))),
        ]
        .spacing(scale.space_sm())
        .align_y(iced::Alignment::Center),
    )
    .padding([3.0, 0.0])
    .width(Length::Fill);

    let mut items = item_guides(ancestors, is_last);
    items.push(content.into());

    let full_row = row(items);

    mouse_area(
        container(full_row)
            .padding([0.0, scale.space_sm()])
            .width(Length::Fill)
            .style(move |theme| crate::ui::styles::tree_row(theme, selected, inside_active)),
    )
    .on_press(Message::Sidebar(SidebarMsg::BeginLongPress {
        kind: DragKind::Folder(folder_id),
        source_parent: parent,
        source_index: index,
        click_action: Some(ClickAction::SelectFolder(folder_id)),
    }))
    .on_enter(Message::Sidebar(SidebarMsg::HoverTarget(inside_target)))
    .on_exit(Message::Sidebar(SidebarMsg::ClearHover))
    .interaction(mouse::Interaction::Grab)
    .into()
}

#[allow(clippy::too_many_arguments)]
fn request_row<'a>(
    tabs: &'a TabStore,
    _drag_state: &'a Option<DragState>,
    selected_folder: &'a Option<NodeId>,
    scale: &UiScale,
    parent: Option<NodeId>,
    index: usize,
    ancestors: &[bool],
    is_last: bool,
    request_id: NodeId,
    request_name: &'a str,
    method: HttpMethod,
) -> Element<'a, Message> {
    let selected = selected_folder.is_none()
        && tabs
            .active()
            .is_some_and(|tab| tab.request_id == Some(request_id));

    let method_color = theme::method_text_color(method);
    let method_label = text(method.as_str())
        .size(scale.text_caption())
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
            container(text(request_name).size(scale.text_label())).width(Length::Fill),
            components::icon_button(icons::lucide_icon("ellipsis", scale.icon_md()), scale)
                .on_press(Message::Sidebar(SidebarMsg::ToggleContextMenu(
                    ContextMenuTarget::Request(request_id),
                ))),
        ]
        .spacing(scale.space_sm())
        .align_y(iced::Alignment::Center),
    )
    .padding([3.0, 0.0])
    .width(Length::Fill);

    let mut items = item_guides(ancestors, is_last);
    items.push(content.into());

    let full_row = row(items);

    mouse_area(
        container(full_row)
            .padding([0.0, scale.space_sm()])
            .width(Length::Fill)
            .style(move |theme| crate::ui::styles::tree_row(theme, selected, false)),
    )
    .on_press(Message::Sidebar(SidebarMsg::BeginLongPress {
        kind: DragKind::Request(request_id),
        source_parent: parent,
        source_index: index,
        click_action: Some(ClickAction::SelectRequest(request_id)),
    }))
    .interaction(mouse::Interaction::Grab)
    .into()
}

fn empty_folder_state(ancestors: &[bool], folder_id: NodeId) -> Element<'static, Message> {
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
            .on_press(Message::Sidebar(SidebarMsg::CreateRequest {
                parent: Some(folder_id),
            }))
            .interaction(mouse::Interaction::Pointer),
        ]
        .spacing(2),
    )
    .padding([3.0, 0.0])
    .width(Length::Fill);

    let mut items = continuation_guides(ancestors);
    items.push(hint.into());

    container(row(items))
        .padding([0.0, 6.0])
        .width(Length::Fill)
        .into()
}

// ── Tree guide rendering ────────────────────────────────────

fn item_guides<'a>(ancestors: &[bool], is_last: bool) -> Vec<Element<'a, Message>> {
    let depth = ancestors.len() + 1;
    let mut items = Vec::new();

    items.push(Space::new().width(Length::Fixed(UiScale::TREE_INDENT)).into());

    if depth <= 1 {
        return items;
    }

    for c in 1..depth.saturating_sub(1) {
        if c < ancestors.len() && ancestors[c] {
            items.push(pipe_guide());
        } else {
            items.push(Space::new().width(Length::Fixed(UiScale::TREE_INDENT)).into());
        }
    }

    if is_last {
        items.push(corner_guide());
    } else {
        items.push(tee_guide());
    }

    items
}

fn continuation_guides<'a>(ancestors: &[bool]) -> Vec<Element<'a, Message>> {
    let depth = ancestors.len() + 1;
    let mut items = Vec::new();

    items.push(Space::new().width(Length::Fixed(UiScale::TREE_INDENT)).into());

    for c in 1..depth {
        if c < ancestors.len() && ancestors[c] {
            items.push(pipe_guide());
        } else {
            items.push(Space::new().width(Length::Fixed(UiScale::TREE_INDENT)).into());
        }
    }

    items
}

fn drop_line_guides<'a>(ancestors: &[bool], pipe_at_connector: bool) -> Vec<Element<'a, Message>> {
    let depth = ancestors.len() + 1;
    let mut items = Vec::new();

    items.push(Space::new().width(Length::Fixed(UiScale::TREE_INDENT)).into());

    if depth <= 1 {
        return items;
    }

    for c in 1..depth.saturating_sub(1) {
        if c < ancestors.len() && ancestors[c] {
            items.push(pipe_guide());
        } else {
            items.push(Space::new().width(Length::Fixed(UiScale::TREE_INDENT)).into());
        }
    }

    if pipe_at_connector {
        items.push(pipe_guide());
    } else {
        items.push(Space::new().width(Length::Fixed(UiScale::TREE_INDENT)).into());
    }

    items
}

fn pipe_guide<'a>() -> Element<'a, Message> {
    let indent = UiScale::TREE_INDENT;
    row![
        container(Space::new())
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(guide_style),
        Space::new().width(Length::Fixed(indent - 1.0)),
    ]
    .width(Length::Fixed(indent))
    .height(Length::Fill)
    .into()
}

fn tee_guide<'a>() -> Element<'a, Message> {
    let indent = UiScale::TREE_INDENT;
    let top = row![
        container(Space::new())
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(guide_style),
        Space::new().width(Length::Fixed(indent - 1.0)),
    ]
    .height(Length::Fill);

    let mid = container(Space::new())
        .width(Length::Fixed(indent))
        .height(Length::Fixed(1.0))
        .style(guide_style);

    let bot = row![
        container(Space::new())
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(guide_style),
        Space::new().width(Length::Fixed(indent - 1.0)),
    ]
    .height(Length::Fill);

    column![top, mid, bot]
        .width(Length::Fixed(indent))
        .height(Length::Fill)
        .into()
}

fn corner_guide<'a>() -> Element<'a, Message> {
    let indent = UiScale::TREE_INDENT;
    let top = row![
        container(Space::new())
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(guide_style),
        Space::new().width(Length::Fixed(indent - 1.0)),
    ]
    .height(Length::Fill);

    let mid = container(Space::new())
        .width(Length::Fixed(indent))
        .height(Length::Fixed(1.0))
        .style(guide_style);

    let bot = Space::new()
        .width(Length::Fixed(indent))
        .height(Length::Fill);

    column![top, mid, bot]
        .width(Length::Fixed(indent))
        .height(Length::Fill)
        .into()
}

fn guide_style(_: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(Background::Color(theme::WHITE_10)),
        ..iced::widget::container::Style::default()
    }
}

// ── Drop line ───────────────────────────────────────────────

fn drop_line<'a>(
    drag_state: &'a Option<DragState>,
    target: SidebarDropTarget,
    ancestors: &[bool],
    pipe_at_connector: bool,
) -> Element<'a, Message> {
    let active = is_sidebar_hover(drag_state, target);

    let line_bar = container(text(""))
        .height(Length::Fixed(if active { 22.0 } else { 2.0 }))
        .width(Length::Fill)
        .style(move |theme| crate::ui::styles::drop_line(theme, active));

    let mut items = drop_line_guides(ancestors, pipe_at_connector);
    items.push(line_bar.into());

    mouse_area(
        container(row(items))
            .padding([0.0, 6.0])
            .width(Length::Fill),
    )
    .on_enter(Message::Sidebar(SidebarMsg::HoverTarget(target)))
    .on_exit(Message::Sidebar(SidebarMsg::ClearHover))
    .into()
}

fn is_sidebar_hover(drag_state: &Option<DragState>, target: SidebarDropTarget) -> bool {
    matches!(
        drag_state,
        Some(DragState::Sidebar {
            hover: Some(current),
            ..
        }) if *current == target
    )
}
