use iced::Task;

use crate::app::Message;
use crate::model::{
    AppState, ClickAction, ContextMenuTarget, DragKind, DragState, HttpMethod,
    PendingLongPress, PressKind, SidebarDropTarget,
};
use crate::state::tree::{NodeData, NodeId};

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
            return Task::done(Message::Tabs(crate::features::TabsMsg::OpenForRequest(request_id)));
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
