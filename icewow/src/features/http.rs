use iced::Task;

use crate::app::Message;
use crate::model::{AppState, BodyType, HttpMethod, ResponseData};

#[derive(Debug, Clone)]
pub enum HttpMsg {
    SendRequest,
    RequestFinished(crate::model::TabId, Result<ResponseData, String>),
}

pub fn update(state: &mut AppState, msg: HttpMsg) -> Task<Message> {
    match msg {
        HttpMsg::SendRequest => {
            let tab = match state.tabs.active() {
                Some(tab) => tab,
                None => return Task::none(),
            };

            if tab.url_input.is_empty() {
                return Task::none();
            }

            let url = tab.url_input.clone();
            let method = tab.method;
            let headers = tab.headers.clone();
            let body_type = tab.body_type;
            let body_text = tab.body_text.clone();
            let form_pairs = tab.form_pairs.clone();

            let tab_id = match state.tabs.active_id() {
                Some(id) => id,
                None => return Task::none(),
            };

            if let Some(tab) = state.tabs.active_mut() {
                tab.loading = true;
                tab.response = None;
            }

            return Task::perform(
                send_engine_request(url, method, headers, body_type, body_text, form_pairs),
                move |result| Message::Http(HttpMsg::RequestFinished(tab_id, result)),
            );
        }
        HttpMsg::RequestFinished(tab_id, result) => {
            if let Some(tab) = state.tabs.get_mut(tab_id) {
                tab.loading = false;
                match result {
                    Ok(response) => tab.response = Some(response),
                    Err(e) => tab.response = Some(ResponseData {
                        status_code: 0,
                        body: format!("Error: {e}"),
                        elapsed_ms: 0,
                        headers: vec![],
                    }),
                }
            }
        }
    }
    Task::none()
}

async fn send_engine_request(
    url: String,
    method: HttpMethod,
    headers: Vec<(String, String)>,
    body_type: BodyType,
    body_text: String,
    form_pairs: Vec<(String, String)>,
) -> Result<ResponseData, String> {
    let client = icewow_engine::Client::new();

    let mut request = icewow_engine::Request::new(url, method);
    for (key, value) in headers {
        request = request.header(key, value);
    }

    match body_type {
        BodyType::None => {}
        BodyType::Raw => {
            request = request.raw_body(body_text);
        }
        BodyType::Json => {
            request = request.header(
                "Content-Type".to_string(),
                "application/json".to_string(),
            );
            request = request.raw_body(body_text);
        }
        BodyType::Form => {
            request = request.form(form_pairs);
        }
    }

    client.execute(request).await.map_err(|e| e.to_string())
}
