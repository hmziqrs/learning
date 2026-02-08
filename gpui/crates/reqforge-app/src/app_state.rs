//! Shared application state for ReqForge GPUI application.
//!
//! This module defines the main application state structure using GPUI's Entity system.

use reqforge_core::{
    ReqForgeCore, models::request::{RequestDefinition, KeyValuePair},
    models::response::HttpResponse,
};
use uuid::Uuid;
use std::sync::Arc;

/// Main application state using GPUI's Entity system.
///
/// This replaces the Arc<RwLock<AppState>> pattern with GPUI's built-in
/// entity management for reactive UI updates.
pub struct AppState {
    /// The headless core logic engine
    pub core: Arc<ReqForgeCore>,
    /// All currently open tabs in the UI
    pub tabs: Vec<TabState>,
    /// Index of the currently active tab (if any tabs are open)
    pub active_tab: Option<usize>,
    /// Currently active environment ID
    pub active_env_id: Option<Uuid>,
}

impl AppState {
    /// Create a new AppState with the given core engine.
    pub fn new(core: ReqForgeCore) -> Self {
        Self {
            core: Arc::new(core),
            tabs: Vec::new(),
            active_tab: None,
            active_env_id: None,
        }
    }

    /// Get a reference to the active tab, if any.
    pub fn active_tab(&self) -> Option<&TabState> {
        self.active_tab.and_then(|index| self.tabs.get(index))
    }

    /// Get a mutable reference to the active tab, if any.
    pub fn active_tab_mut(&mut self) -> Option<&mut TabState> {
        self.active_tab.and_then(move |index| self.tabs.get_mut(index))
    }

    /// Open a new tab with the given request definition.
    pub fn open_tab(&mut self, request_id: Uuid, collection_id: Uuid, draft: RequestDefinition) {
        // Note: This will be updated in Phase 3 to create Entity<InputState> for each field
        // For now, we use a simplified structure that will be fleshed out when GPUI is integrated
        let tab = TabState {
            request_id,
            collection_id,
            draft,
            last_response: None,
            is_loading: false,
            is_dirty: false,
        };
        self.tabs.push(tab);
        self.active_tab = Some(self.tabs.len() - 1);
    }

    /// Close the currently active tab.
    pub fn close_active_tab(&mut self) {
        if let Some(index) = self.active_tab {
            self.tabs.remove(index);
            // Update active tab index
            if self.tabs.is_empty() {
                self.active_tab = None;
            } else if index >= self.tabs.len() {
                self.active_tab = Some(self.tabs.len() - 1);
            } else {
                self.active_tab = Some(index);
            }
        }
    }

    /// Execute the active tab's request and update with the response.
    pub async fn execute_active_tab_request(&self) -> Result<HttpResponse, String> {
        let request = self
            .active_tab()
            .map(|tab| tab.draft.clone())
            .ok_or_else(|| "No active tab".to_string())?;

        self.core.execute_request(&request)
            .await
            .map_err(|e| format!("Request failed: {}", e))
    }
}

/// Represents a single open tab in the application.
///
/// In the full GPUI implementation, this will contain Entity<InputState>
/// fields for URL, headers, params, and body inputs.
pub struct TabState {
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

/// Key-value row for headers and params editors.
///
/// In the full GPUI implementation, this will use Entity<InputState>
/// for the key and value inputs.
pub struct KeyValueRow {
    /// Unique identifier for this row
    pub id: Uuid,
    /// Whether this row is enabled
    pub enabled: bool,
    /// Key string (will be Entity<InputState> in Phase 3)
    pub key: String,
    /// Value string (will be Entity<InputState> in Phase 3)
    pub value: String,
}

impl KeyValueRow {
    /// Create a new key-value row.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            enabled: true,
            key: String::new(),
            value: String::new(),
        }
    }

    /// Create a key-value row from a KeyValuePair.
    pub fn from_kv_pair(pair: &KeyValuePair) -> Self {
        Self {
            id: Uuid::new_v4(),
            enabled: pair.enabled,
            key: pair.key.clone(),
            value: pair.value.clone(),
        }
    }

    /// Convert to a KeyValuePair.
    pub fn to_kv_pair(&self) -> KeyValuePair {
        KeyValuePair {
            key: self.key.clone(),
            value: self.value.clone(),
            enabled: self.enabled,
            description: None,
        }
    }
}
