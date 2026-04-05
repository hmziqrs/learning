use iced::Task;

use crate::app::Message;
use crate::model::{AppState, BodyType, HttpMethod, RequestTab};

#[derive(Debug, Clone)]
pub enum EditorMsg {
    UrlChanged(String),
    MethodChanged(HttpMethod),
    SetBodyType(BodyType),
    UpdateBodyText(String),
    AddFormPair,
    UpdateFormKey(usize, String),
    UpdateFormValue(usize, String),
    RemoveFormPair(usize),
    AddHeader,
    UpdateHeaderKey(usize, String),
    UpdateHeaderValue(usize, String),
    RemoveHeader(usize),
    AddQueryParam,
    UpdateQueryParamKey(usize, String),
    UpdateQueryParamValue(usize, String),
    RemoveQueryParam(usize),
    SetRequestTab(RequestTab),
    SaveRequest,
    RequestNameChanged(String),
}

pub fn update(state: &mut AppState, msg: EditorMsg) -> Task<Message> {
    match msg {
        EditorMsg::UrlChanged(value) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.url_input = value;
                tab.dirty = true;
            }
        }
        EditorMsg::MethodChanged(method) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.method = method;
                tab.dirty = true;
            }
        }
        EditorMsg::SetBodyType(body_type) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.body_type = body_type;
                tab.dirty = true;
            }
        }
        EditorMsg::UpdateBodyText(text) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.body_text = text;
                tab.dirty = true;
            }
        }
        EditorMsg::AddFormPair => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.form_pairs.push((String::new(), String::new()));
                tab.dirty = true;
            }
        }
        EditorMsg::UpdateFormKey(index, key) => {
            if let Some(tab) = state.tabs.active_mut() {
                if let Some(pair) = tab.form_pairs.get_mut(index) {
                    pair.0 = key;
                    tab.dirty = true;
                }
            }
        }
        EditorMsg::UpdateFormValue(index, value) => {
            if let Some(tab) = state.tabs.active_mut() {
                if let Some(pair) = tab.form_pairs.get_mut(index) {
                    pair.1 = value;
                    tab.dirty = true;
                }
            }
        }
        EditorMsg::RemoveFormPair(index) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.form_pairs.remove(index);
                tab.dirty = true;
            }
        }
        EditorMsg::AddHeader => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.headers.push((String::new(), String::new()));
                tab.dirty = true;
            }
        }
        EditorMsg::UpdateHeaderKey(index, key) => {
            if let Some(tab) = state.tabs.active_mut() {
                if let Some(pair) = tab.headers.get_mut(index) {
                    pair.0 = key;
                    tab.dirty = true;
                }
            }
        }
        EditorMsg::UpdateHeaderValue(index, value) => {
            if let Some(tab) = state.tabs.active_mut() {
                if let Some(pair) = tab.headers.get_mut(index) {
                    pair.1 = value;
                    tab.dirty = true;
                }
            }
        }
        EditorMsg::RemoveHeader(index) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.headers.remove(index);
                tab.dirty = true;
            }
        }
        EditorMsg::AddQueryParam => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.query_params.push((String::new(), String::new()));
                tab.dirty = true;
            }
        }
        EditorMsg::UpdateQueryParamKey(index, key) => {
            if let Some(tab) = state.tabs.active_mut() {
                if let Some(pair) = tab.query_params.get_mut(index) {
                    pair.0 = key;
                    tab.dirty = true;
                }
            }
        }
        EditorMsg::UpdateQueryParamValue(index, value) => {
            if let Some(tab) = state.tabs.active_mut() {
                if let Some(pair) = tab.query_params.get_mut(index) {
                    pair.1 = value;
                    tab.dirty = true;
                }
            }
        }
        EditorMsg::RemoveQueryParam(index) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.query_params.remove(index);
                tab.dirty = true;
            }
        }
        EditorMsg::SetRequestTab(request_tab) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.active_request_tab = request_tab;
            }
        }
        EditorMsg::SaveRequest => {
            let tab = match state.tabs.active() {
                Some(tab) => tab,
                None => return Task::none(),
            };

            if let Some(request_id) = tab.request_id {
                let name = tab.title.clone();
                let url = tab.url_input.clone();
                let method = tab.method;

                state.tree.update_request_from_draft(
                    request_id,
                    &name,
                    &url,
                    method,
                );

                if let Some(tab) = state.tabs.active_mut() {
                    tab.dirty = false;
                }
            }
        }
        EditorMsg::RequestNameChanged(name) => {
            if let Some(tab) = state.tabs.active_mut() {
                tab.title = name;
                tab.dirty = true;
            }
        }
    }
    Task::none()
}
