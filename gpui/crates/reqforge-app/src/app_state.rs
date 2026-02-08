//! Shared application state for ReqForge GPUI application.
//!
//! This module defines the main application state structure that wraps
//! the headless core and adds UI-specific state like open tabs.

use reqforge_core::{ReqForgeCore, models::request::RequestDefinition, models::response::HttpResponse};
use std::sync::Arc;
use parking_lot::RwLock;
use uuid::Uuid;

/// Wraps the headless core and adds UI-specific state.
pub struct AppState {
    /// The headless core logic engine wrapped in a thread-safe lock
    pub core: Arc<RwLock<ReqForgeCore>>,
    /// All currently open tabs in the UI
    pub open_tabs: Vec<OpenTab>,
    /// Index of the currently active tab (if any tabs are open)
    pub active_tab_index: Option<usize>,
}

impl AppState {
    /// Create a new AppState with the given core engine.
    pub fn new(core: ReqForgeCore) -> Self {
        Self {
            core: Arc::new(RwLock::new(core)),
            open_tabs: Vec::new(),
            active_tab_index: None,
        }
    }

    /// Open a new tab with the given request.
    pub fn open_tab(&mut self, request_id: Uuid, collection_id: Uuid, draft: RequestDefinition) {
        let tab = OpenTab {
            request_id,
            collection_id,
            draft,
            last_response: None,
            is_loading: false,
            is_dirty: false,
        };
        self.open_tabs.push(tab);
        self.active_tab_index = Some(self.open_tabs.len() - 1);
    }

    /// Close the currently active tab.
    pub fn close_active_tab(&mut self) {
        if let Some(index) = self.active_tab_index {
            self.open_tabs.remove(index);
            // Update active tab index
            if self.open_tabs.is_empty() {
                self.active_tab_index = None;
            } else if index >= self.open_tabs.len() {
                self.active_tab_index = Some(self.open_tabs.len() - 1);
            } else {
                self.active_tab_index = Some(index);
            }
        }
    }

    /// Get a reference to the active tab, if any.
    pub fn active_tab(&self) -> Option<&OpenTab> {
        self.active_tab_index.and_then(|index| self.open_tabs.get(index))
    }

    /// Get a mutable reference to the active tab, if any.
    #[allow(dead_code)]
    pub fn active_tab_mut(&mut self) -> Option<&mut OpenTab> {
        self.active_tab_index.and_then(move |index| self.open_tabs.get_mut(index))
    }

    /// Update the request draft in the active tab.
    pub fn update_active_tab_draft(&mut self, draft: RequestDefinition) {
        if let Some(tab) = self.active_tab_mut() {
            tab.draft = draft;
            tab.is_dirty = true;
        }
    }

    /// Set the loading state for the active tab.
    pub fn set_active_tab_loading(&mut self, loading: bool) {
        if let Some(tab) = self.active_tab_mut() {
            tab.is_loading = loading;
        }
    }

    /// Update the response for the active tab.
    pub fn update_active_tab_response(&mut self, response: HttpResponse) {
        if let Some(tab) = self.active_tab_mut() {
            tab.last_response = Some(response);
            tab.is_loading = false;
        }
    }

    /// Execute a request and update the active tab with the response.
    pub async fn execute_active_tab_request(&self) -> Result<HttpResponse, String> {
        // Get the active tab's request
        let request = self.active_tab()
            .map(|tab| tab.draft.clone())
            .ok_or_else(|| "No active tab".to_string())?;

        // Execute the request using the core
        let core = self.core.read();
        core.execute_request(&request)
            .await
            .map_err(|e| format!("Request failed: {}", e))
    }
}

/// Represents a single open tab in the application.
#[allow(dead_code)]
pub struct OpenTab {
    /// ID of the request being edited in this tab
    pub request_id: Uuid,
    /// ID of the collection this request belongs to
    pub collection_id: Uuid,
    /// In-progress edits (unsaved changes)
    pub draft: RequestDefinition,
    /// Last response received from executing this request
    pub last_response: Option<HttpResponse>,
    /// Whether a request is currently in-flight
    pub is_loading: bool,
    /// Whether there are unsaved changes
    pub is_dirty: bool,
}
