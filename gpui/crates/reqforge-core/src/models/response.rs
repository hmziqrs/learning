use std::collections::HashMap;
use std::time::Duration;
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Bytes,                         // refcounted, cheap clone
    pub size_bytes: usize,
    pub elapsed: Duration,
}

impl HttpResponse {
    /// Get response body as UTF-8 text, if valid.
    /// Borrows from the internal Bytes — zero allocation.
    pub fn body_text(&self) -> Option<&str> {
        std::str::from_utf8(&self.body).ok()
    }

    /// Pretty-print JSON body if applicable
    pub fn pretty_body(&self) -> Option<String> {
        let text = self.body_text()?;
        let val: serde_json::Value = serde_json::from_str(text).ok()?;
        serde_json::to_string_pretty(&val).ok()
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }
}
