pub mod models;
pub mod http;
pub mod env;
pub mod store;
pub mod history;
pub mod validation;
pub mod templates;
pub mod import_export;

#[cfg(test)]
mod integration_tests;

// Re-export commonly used types for external consumers
pub use models::{collection::Collection, environment::Environment, request::RequestDefinition, response::HttpResponse};
pub use models::history::{RequestHistoryEntry, ResponseSnapshot};
pub use models::template::{RequestTemplate, TemplateCategory, TemplateVariable, BodyTemplateType};
pub use http::client::{HttpEngine, HttpError};
pub use env::interpolator::Interpolator;
pub use store::json_store::{JsonStore, StoreError};
pub use history::{RequestHistory, ReplayError};
pub use validation::{ValidationError, ValidationResult, validate_url, validate_headers, validate_body, validate_request};
pub use templates::{TemplateManager, TemplateError};
pub use import_export::{export_collection, import_collection, export_environment, import_environment, export_all, import_all, import_collection_from_postman, import_collection_from_openapi};
pub use import_export::WorkspaceImport;
use std::collections::HashMap;

/// The top-level headless API surface.
/// The UI crate only talks to this.
pub struct ReqForgeCore {
    pub engine: HttpEngine,
    pub store: JsonStore,
    pub history: RequestHistory,
    pub environments: Vec<Environment>,
    pub collections: Vec<Collection>,
    pub active_environment_id: Option<uuid::Uuid>,
}

impl ReqForgeCore {
    pub fn open(workspace_dir: impl Into<std::path::PathBuf>) -> Result<Self, StoreError> {
        let workspace_dir = workspace_dir.into();
        let store = JsonStore::open(&workspace_dir)?;
        let environments = store.load_environments()?;
        let collections = store.list_collections()?;

        let mut history = RequestHistory::new(workspace_dir.clone());
        history.load()?;

        Ok(Self {
            engine: HttpEngine::new(),
            store,
            history,
            environments,
            collections,
            active_environment_id: None,
        })
    }

    /// Get the merged variable map for the active environment.
    pub fn active_vars(&self) -> HashMap<String, String> {
        self.active_environment_id
            .and_then(|id| self.environments.iter().find(|e| e.id == id))
            .map(|e| e.to_map())
            .unwrap_or_default()
    }

    /// Get the active environment name if set
    pub fn active_environment_name(&self) -> Option<String> {
        self.active_environment_id
            .and_then(|id| self.environments.iter().find(|e| e.id == id))
            .map(|e| e.name.clone())
    }

    /// Execute a request with environment interpolation.
    pub async fn execute_request(&mut self, req: &RequestDefinition) -> Result<HttpResponse, HttpError> {
        let vars = self.active_vars();
        let resolved = Interpolator::resolve(req, &vars);
        let response = self.engine.execute(&resolved).await;

        // Add to history regardless of success/failure
        let snapshot = match &response {
            Ok(resp) => ResponseSnapshot::from(resp),
            Err(_) => ResponseSnapshot {
                status: 0,
                status_text: "Error".to_string(),
                size_bytes: 0,
                elapsed_millis: 0,
                success: false,
            },
        };

        let entry = RequestHistoryEntry::new(
            req.clone(),
            snapshot,
            self.active_environment_id,
            self.active_environment_name(),
        );
        self.history.add_entry(entry);

        response
    }

    /// Persist all state to disk.
    pub fn save_all(&self) -> Result<(), StoreError> {
        self.store.save_environments(&self.environments)?;
        for col in &self.collections {
            self.store.save_collection(col)?;
        }
        self.history.save()?;
        Ok(())
    }

    /// Get recent history entries
    pub fn get_recent_history(&self, count: usize) -> Vec<RequestHistoryEntry> {
        self.history.get_recent(count)
    }

    /// Get all history entries
    pub fn get_all_history(&self) -> Vec<RequestHistoryEntry> {
        self.history.get_all()
    }

    /// Clear all history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Replay a history entry
    pub async fn replay_history(&self, entry_id: uuid::Uuid) -> Result<HttpResponse, ReplayError> {
        self.history.replay(entry_id, &self.engine, &self.active_vars()).await
    }

    /// Get history entry count
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Check if history is empty
    pub fn history_is_empty(&self) -> bool {
        self.history.is_empty()
    }
}
