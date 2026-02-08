use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub body_text: Option<String>,           // attempt UTF-8 decode
    pub size_bytes: usize,
    pub elapsed: Duration,
}

impl HttpResponse {
    /// Pretty-print JSON body if applicable
    pub fn pretty_body(&self) -> Option<String> {
        let text = self.body_text.as_ref()?;
        let val: serde_json::Value = serde_json::from_str(text).ok()?;
        serde_json::to_string_pretty(&val).ok()
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }
}
