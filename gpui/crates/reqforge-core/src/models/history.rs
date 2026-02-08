use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::request::RequestDefinition;

/// Snapshot of response information for history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseSnapshot {
    pub status: u16,
    pub status_text: String,
    pub size_bytes: usize,
    pub elapsed_millis: u64,
    pub success: bool,
}

/// A single entry in the request history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestHistoryEntry {
    pub id: Uuid,
    pub request: RequestDefinition,
    pub response: ResponseSnapshot,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub environment_id: Option<Uuid>,
    pub environment_name: Option<String>,
}

impl RequestHistoryEntry {
    pub fn new(
        request: RequestDefinition,
        response: ResponseSnapshot,
        environment_id: Option<Uuid>,
        environment_name: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            request,
            response,
            timestamp: chrono::Utc::now(),
            environment_id,
            environment_name,
        }
    }
}

impl From<&crate::HttpResponse> for ResponseSnapshot {
    fn from(resp: &crate::HttpResponse) -> Self {
        Self {
            status: resp.status,
            status_text: resp.status_text.clone(),
            size_bytes: resp.size_bytes,
            elapsed_millis: resp.elapsed.as_millis() as u64,
            success: resp.is_success(),
        }
    }
}
