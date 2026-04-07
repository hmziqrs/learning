use std::collections::VecDeque;
use std::path::PathBuf;

use crate::http::{HttpEngine, HttpError};
use crate::models::history::{RequestHistoryEntry, ResponseSnapshot};
use crate::models::request::RequestDefinition;
use crate::store::StoreError;

const DEFAULT_MAX_HISTORY_SIZE: usize = 100;

/// Manager for request history with persistence
#[derive(Debug)]
pub struct RequestHistory {
    entries: VecDeque<RequestHistoryEntry>,
    max_size: usize,
    history_path: PathBuf,
    is_dirty: bool,
}

impl RequestHistory {
    /// Create a new RequestHistory manager
    pub fn new(workspace_dir: PathBuf) -> Self {
        let history_path = workspace_dir.join("history.json");
        Self {
            entries: VecDeque::with_capacity(DEFAULT_MAX_HISTORY_SIZE),
            max_size: DEFAULT_MAX_HISTORY_SIZE,
            history_path,
            is_dirty: false,
        }
    }

    /// Set the maximum number of history entries to keep
    pub fn with_max_size(mut self, max_size: usize) -> Self {
        self.max_size = max_size;
        self
    }

    /// Load history from disk
    pub fn load(&mut self) -> Result<(), StoreError> {
        if !self.history_path.exists() {
            return Ok(());
        }

        let data = std::fs::read_to_string(&self.history_path)?;
        let loaded: Vec<RequestHistoryEntry> = serde_json::from_str(&data)?;

        self.entries.clear();
        for entry in loaded.into_iter().rev().take(self.max_size) {
            self.entries.push_back(entry);
        }

        self.is_dirty = false;
        Ok(())
    }

    /// Save history to disk if dirty
    pub fn save(&self) -> Result<(), StoreError> {
        if !self.is_dirty {
            return Ok(());
        }

        let entries: Vec<_> = self.entries.iter().cloned().collect();
        let json = serde_json::to_string_pretty(&entries)?;
        std::fs::write(&self.history_path, json)?;

        Ok(())
    }

    /// Add a new history entry
    pub fn add_entry(&mut self, entry: RequestHistoryEntry) {
        self.entries.push_front(entry);

        // Trim to max size
        while self.entries.len() > self.max_size {
            self.entries.pop_back();
        }

        self.is_dirty = true;
    }

    /// Get all history entries, most recent first
    pub fn get_all(&self) -> Vec<RequestHistoryEntry> {
        self.entries.iter().cloned().collect()
    }

    /// Get recent history entries
    pub fn get_recent(&self, count: usize) -> Vec<RequestHistoryEntry> {
        self.entries
            .iter()
            .take(count)
            .cloned()
            .collect()
    }

    /// Get a specific entry by ID
    pub fn get_entry(&self, id: uuid::Uuid) -> Option<&RequestHistoryEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.entries.clear();
        self.is_dirty = true;
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Replay a history entry
    pub async fn replay(
        &self,
        entry_id: uuid::Uuid,
        engine: &HttpEngine,
        vars: &std::collections::HashMap<String, String>,
    ) -> Result<crate::HttpResponse, ReplayError> {
        let entry = self
            .get_entry(entry_id)
            .ok_or(ReplayError::EntryNotFound(entry_id))?;

        // Re-resolve the request with current variables
        let resolved = crate::env::Interpolator::resolve(&entry.request, vars);
        Ok(engine.execute(&resolved).await?)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ReplayError {
    #[error("History entry not found: {0}")]
    EntryNotFound(uuid::Uuid),

    #[error("HTTP error: {0}")]
    Http(#[from] HttpError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::request::HttpMethod;

    #[test]
    fn test_add_entry() {
        let temp = tempfile::tempdir().unwrap();
        let mut history = RequestHistory::new(temp.path().to_path_buf());

        let request = RequestDefinition::new("Test", HttpMethod::GET, "https://example.com");
        let response = ResponseSnapshot {
            status: 200,
            status_text: "OK".to_string(),
            size_bytes: 100,
            elapsed_millis: 50,
            success: true,
        };

        let entry = RequestHistoryEntry::new(request, response, None, None);
        history.add_entry(entry);

        assert_eq!(history.len(), 1);
        assert!(!history.is_empty());
    }

    #[test]
    fn test_max_size() {
        let temp = tempfile::tempdir().unwrap();
        let mut history = RequestHistory::new(temp.path().to_path_buf()).with_max_size(3);

        for i in 0..5 {
            let request = RequestDefinition::new(
                format!("Test {}", i),
                HttpMethod::GET,
                "https://example.com",
            );
            let response = ResponseSnapshot {
                status: 200,
                status_text: "OK".to_string(),
                size_bytes: 100,
                elapsed_millis: 50,
                success: true,
            };

            let entry = RequestHistoryEntry::new(request, response, None, None);
            history.add_entry(entry);
        }

        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_get_recent() {
        let temp = tempfile::tempdir().unwrap();
        let mut history = RequestHistory::new(temp.path().to_path_buf());

        for i in 0..10 {
            let request = RequestDefinition::new(
                format!("Test {}", i),
                HttpMethod::GET,
                "https://example.com",
            );
            let response = ResponseSnapshot {
                status: 200,
                status_text: "OK".to_string(),
                size_bytes: 100,
                elapsed_millis: 50,
                success: true,
            };

            let entry = RequestHistoryEntry::new(request, response, None, None);
            history.add_entry(entry);
        }

        let recent = history.get_recent(5);
        assert_eq!(recent.len(), 5);
        // Most recent should be "Test 9" (added last)
        assert_eq!(recent[0].request.name, "Test 9");
    }

    #[test]
    fn test_clear() {
        let temp = tempfile::tempdir().unwrap();
        let mut history = RequestHistory::new(temp.path().to_path_buf());

        let request = RequestDefinition::new("Test", HttpMethod::GET, "https://example.com");
        let response = ResponseSnapshot {
            status: 200,
            status_text: "OK".to_string(),
            size_bytes: 100,
            elapsed_millis: 50,
            success: true,
        };

        let entry = RequestHistoryEntry::new(request, response, None, None);
        history.add_entry(entry);

        assert_eq!(history.len(), 1);

        history.clear();
        assert_eq!(history.len(), 0);
        assert!(history.is_empty());
    }

    #[test]
    fn test_persistence() {
        let temp = tempfile::tempdir().unwrap();
        let mut history = RequestHistory::new(temp.path().to_path_buf());

        let request = RequestDefinition::new("Test", HttpMethod::GET, "https://example.com");
        let response = ResponseSnapshot {
            status: 200,
            status_text: "OK".to_string(),
            size_bytes: 100,
            elapsed_millis: 50,
            success: true,
        };

        let entry = RequestHistoryEntry::new(request, response, None, None);
        history.add_entry(entry);

        // Save and reload
        history.save().unwrap();

        let mut history2 = RequestHistory::new(temp.path().to_path_buf());
        history2.load().unwrap();

        assert_eq!(history2.len(), 1);
        let loaded = history2.get_all();
        assert_eq!(loaded[0].request.name, "Test");
        assert_eq!(loaded[0].response.status, 200);
    }
}
