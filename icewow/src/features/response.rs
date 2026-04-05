use iced::Task;

use crate::app::Message;
use crate::model::{AppState, ResponseTab};

#[derive(Debug, Clone)]
pub enum ResponseMsg {
    SetResponseTab(ResponseTab),
}

pub fn update(state: &mut AppState, msg: ResponseMsg) -> Task<Message> {
    match msg {
        ResponseMsg::SetResponseTab(response_tab) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.active_response_tab = response_tab;
            }
        }
    }
    Task::none()
}
