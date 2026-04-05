use iced::Task;

use crate::app::Message;
use crate::model::{AppState, DragState, TabId};

#[derive(Debug, Clone)]
pub enum TabsMsg {
    NewTab,
    AskDeleteTab(TabId),
    SelectTab(TabId),
    BeginLongPress {
        tab_id: TabId,
        source_index: usize,
    },
    HoverIndex(usize),
    ClearHover,
}

pub fn update(state: &mut AppState, msg: TabsMsg) -> Task<Message> {
    match msg {
        TabsMsg::NewTab => {
            state.tabs.new_tab();
        }
        TabsMsg::AskDeleteTab(tab_id) => {
            state.delete_dialog = Some(crate::model::DeleteDialog::Tab(tab_id));
        }
        TabsMsg::SelectTab(tab_id) => {
            state.tabs.set_active(tab_id);
        }
        TabsMsg::BeginLongPress { tab_id, source_index } => {
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
