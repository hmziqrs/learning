//! Shared application state for ReqForge GPUI application.
//!
//! This module defines the main application state structure using GPUI's Entity system.
//!
//! Zero-copy architecture:
//! - TabState uses Entity<InputState> for URL, body, headers, and params
//! - Text is managed internally by gpui-component (Rope-backed)
//! - String allocation only occurs at ownership boundaries (save/send)

use gpui::{AppContext, Context, Entity, Window};
use gpui_component::input::InputState;
use reqforge_core::{
    ReqForgeCore,
    models::request::{HttpMethod, KeyValuePair, RequestDefinition},
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

    /// Open a new tab with the given pre-constructed TabState.
    ///
    /// The TabState must have all entities (url_input, body_input, headers, params)
    /// already created by the UI layer before calling this method.
    /// This follows the ownership boundary pattern where entity creation happens
    /// in the UI layer with proper App context, not in AppState.
    pub fn open_tab(&mut self, tab: TabState) {
        self.tabs.push(tab);
        self.active_tab = Some(self.tabs.len() - 1);
    }

    /// Create a new tab from a RequestDefinition.
    ///
    /// This is a convenience method that creates all necessary entities
    /// and adds the tab to the application state. Entity creation happens
    /// within the Context<AppState> which is available in update closures.
    pub fn create_tab_from_request(
        &mut self,
        req: &RequestDefinition,
        collection_id: Uuid,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Create URL input state
        let url_input = cx.new(|cx| {
            InputState::new(window, cx).default_value(req.url.clone())
        });

        // Create body input state based on BodyType
        let body_content = match &req.body {
            reqforge_core::models::request::BodyType::Raw { content, .. } => content.clone(),
            reqforge_core::models::request::BodyType::FormUrlEncoded(_) => {
                // Convert form data to URL-encoded string
                crate::bridge::body_type_to_string(&req.body).unwrap_or_default()
            }
            reqforge_core::models::request::BodyType::None => String::new(),
        };
        let body_input = cx.new(|cx| {
            InputState::new(window, cx).default_value(body_content)
        });

        // Create header rows from KeyValuePair vector
        let headers: Vec<KeyValueRow> = req
            .headers
            .iter()
            .map(|pair| {
                let key_input = cx.new(|cx| {
                    InputState::new(window, cx).default_value(pair.key.clone())
                });
                let value_input = cx.new(|cx| {
                    InputState::new(window, cx).default_value(pair.value.clone())
                });
                KeyValueRow::with_enabled(key_input, value_input, pair.enabled)
            })
            .collect();

        // Create param rows from KeyValuePair vector
        let params: Vec<KeyValueRow> = req
            .query_params
            .iter()
            .map(|pair| {
                let key_input = cx.new(|cx| {
                    InputState::new(window, cx).default_value(pair.key.clone())
                });
                let value_input = cx.new(|cx| {
                    InputState::new(window, cx).default_value(pair.value.clone())
                });
                KeyValueRow::with_enabled(key_input, value_input, pair.enabled)
            })
            .collect();

        let tab = TabState::new(
            req.id,
            collection_id,
            req.name.clone(),
            req.method.clone(),
            url_input,
            body_input,
            headers,
            params,
        );

        self.open_tab(tab);
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
    ///
    /// This method builds a RequestDefinition from the Entity<InputState> fields
    /// and passes it to the core for execution.
    pub async fn execute_active_tab_request(&mut self, cx: &mut Context<'_, Self>) -> Result<HttpResponse, String> {
        let (request_id, _collection_id, method, url, headers, query_params, body) = self
            .active_tab()
            .map(|tab| {
                let url = tab.url_input.read(cx).text().to_string();
                let body_content = tab.body_input.read(cx).text().to_string();
                let headers: Vec<KeyValuePair> = tab.headers.iter().map(|row| row.to_kv_pair(cx)).collect();
                let query_params: Vec<KeyValuePair> = tab.params.iter().map(|row| row.to_kv_pair(cx)).collect();

                let body = if body_content.is_empty() {
                    reqforge_core::models::request::BodyType::None
                } else {
                    reqforge_core::models::request::BodyType::Raw {
                        content: body_content,
                        content_type: reqforge_core::models::request::RawContentType::Json,
                    }
                };

                (tab.request_id, tab.collection_id, tab.method.clone(), url, headers, query_params, body)
            })
            .ok_or_else(|| "No active tab".to_string())?;

        // Build a temporary RequestDefinition for execution
        let now = chrono::Utc::now();
        let request = RequestDefinition {
            id: request_id,
            name: format!("Request {}", request_id),
            method,
            url,
            headers,
            query_params,
            body,
            created_at: now,
            updated_at: now,
        };

        self.core.execute_request(&request)
            .await
            .map_err(|e| format!("Request failed: {}", e))
    }
}

/// Represents a single open tab in the application.
///
/// Zero-copy architecture: Uses Entity<InputState> for all text inputs.
/// Text is managed internally by gpui-component and only converted to
/// String at ownership boundaries (save/send).
pub struct TabState {
    /// ID of the request being edited in this tab
    pub request_id: Uuid,
    /// ID of the collection this request belongs to
    pub collection_id: Uuid,
    /// Request name for display in the tab bar
    pub name: String,
    /// HTTP method for this request
    pub method: HttpMethod,
    /// URL input state (zero-copy: managed by gpui)
    pub url_input: Entity<InputState>,
    /// Body input state (zero-copy: managed by gpui)
    pub body_input: Entity<InputState>,
    /// Header rows with Entity<InputState> for key and value
    pub headers: Vec<KeyValueRow>,
    /// Query parameter rows with Entity<InputState> for key and value
    pub params: Vec<KeyValueRow>,
    /// Last response received from executing this request
    pub last_response: Option<HttpResponse>,
    /// Whether a request is currently in-flight
    pub is_loading: bool,
    /// Whether there are unsaved changes
    pub is_dirty: bool,
}

impl TabState {
    /// Create a new TabState with all required entities.
    ///
    /// All entities must be pre-created by the UI layer before calling this constructor.
    /// This follows the ownership boundary pattern where entity creation happens
    /// in the UI layer with proper App context.
    pub fn new(
        request_id: Uuid,
        collection_id: Uuid,
        name: String,
        method: HttpMethod,
        url_input: Entity<InputState>,
        body_input: Entity<InputState>,
        headers: Vec<KeyValueRow>,
        params: Vec<KeyValueRow>,
    ) -> Self {
        Self {
            request_id,
            collection_id,
            name,
            method,
            url_input,
            body_input,
            headers,
            params,
            last_response: None,
            is_loading: false,
            is_dirty: false,
        }
    }

    /// Build a RequestDefinition from this tab's state.
    ///
    /// This is the ownership boundary where we convert from Entity<InputState>
    /// to String for creating the RequestDefinition that will be passed to the core.
    pub fn to_request_definition(&self, cx: &mut Context<AppState>) -> RequestDefinition {
        let url = self.url_input.read(cx).text().to_string();
        let body_content = self.body_input.read(cx).text().to_string();

        let headers: Vec<KeyValuePair> = self
            .headers
            .iter()
            .map(|row| row.to_kv_pair(cx))
            .collect();

        let query_params: Vec<KeyValuePair> = self
            .params
            .iter()
            .map(|row| row.to_kv_pair(cx))
            .collect();

        // For now, we'll use a simple Raw body type if there's content
        let body = if body_content.is_empty() {
            reqforge_core::models::request::BodyType::None
        } else {
            reqforge_core::models::request::BodyType::Raw {
                content: body_content,
                content_type: reqforge_core::models::request::RawContentType::Json,
            }
        };

        let now = chrono::Utc::now();
        RequestDefinition {
            id: self.request_id,
            name: self.name.clone(),
            method: self.method.clone(),
            url,
            headers,
            query_params,
            body,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Key-value row for headers and params editors.
///
/// Uses Entity<InputState> for zero-copy text management.
/// The text is stored internally by gpui-component and only
/// converted to String when needed at ownership boundaries.
#[derive(Clone)]
pub struct KeyValueRow {
    /// Unique identifier for this row
    pub id: Uuid,
    /// Whether this row is enabled
    pub enabled: bool,
    /// Key input state (zero-copy: managed by gpui)
    pub key_input: Entity<InputState>,
    /// Value input state (zero-copy: managed by gpui)
    pub value_input: Entity<InputState>,
}

impl KeyValueRow {
    /// Create a new key-value row with pre-constructed InputState entities.
    ///
    /// The entities must be created by the UI layer before calling this constructor.
    /// This follows the ownership boundary pattern where entity creation happens
    /// in the UI layer with proper App context.
    pub fn new(key_input: Entity<InputState>, value_input: Entity<InputState>) -> Self {
        Self {
            id: Uuid::new_v4(),
            enabled: true,
            key_input,
            value_input,
        }
    }

    /// Create a new key-value row with enabled flag.
    pub fn with_enabled(key_input: Entity<InputState>, value_input: Entity<InputState>, enabled: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            enabled,
            key_input,
            value_input,
        }
    }

    /// Get the current key text.
    ///
    /// This performs a single allocation to convert from the internal
    /// representation to String. This is the ownership boundary.
    pub fn key_text(&self, cx: &mut Context<AppState>) -> String {
        self.key_input.read(cx).text().to_string()
    }

    /// Get the current value text.
    ///
    /// This performs a single allocation to convert from the internal
    /// representation to String. This is the ownership boundary.
    pub fn value_text(&self, cx: &mut Context<AppState>) -> String {
        self.value_input.read(cx).text().to_string()
    }

    /// Convert to a KeyValuePair.
    ///
    /// This is the ownership boundary where we convert from Entity<InputState> to KeyValuePair.
    pub fn to_kv_pair(&self, cx: &mut Context<AppState>) -> KeyValuePair {
        KeyValuePair {
            key: self.key_text(cx),
            value: self.value_text(cx),
            enabled: self.enabled,
            description: None,
        }
    }

    /// Check if this row has valid content.
    pub fn is_valid(&self, cx: &mut Context<AppState>) -> bool {
        !self.key_text(cx).is_empty() || !self.value_text(cx).is_empty()
    }
}
