use crate::http::method::HttpMethod;

pub struct Request {
    pub url: String,
    pub method: HttpMethod,
    pub headers: Vec<(String, String)>,
    pub body: Option<RequestBody>,
}

#[derive(Debug, Clone)]
pub enum RequestBody {
    Raw(String),
    Json(serde_json::Value),
    Form(Vec<(String, String)>),
}

impl Request {
    pub fn new(url: String, method: HttpMethod) -> Self {
        Self {
            url,
            method,
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.push((key, value));
        self
    }

    pub fn body(mut self, body: RequestBody) -> Self {
        self.body = Some(body);
        self
    }

    pub fn json(mut self, value: serde_json::Value) -> Self {
        self.body = Some(RequestBody::Json(value));
        self
    }

    pub fn raw_body(mut self, text: String) -> Self {
        self.body = Some(RequestBody::Raw(text));
        self
    }

    pub fn form(mut self, pairs: Vec<(String, String)>) -> Self {
        self.body = Some(RequestBody::Form(pairs));
        self
    }
}
