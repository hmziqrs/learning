# ReqForge — A Postman-like HTTP Client in Rust + GPUI

## Architecture Overview

```
┌─────────────────────────────────────────────────────┐
│                   GPUI Frontend                      │
│  (Views, Panels, Input Bindings, Theming)            │
│                                                      │
│  ┌───────────┐ ┌───────────┐ ┌────────────────────┐ │
│  │ Sidebar   │ │ Request   │ │ Response Viewer    │ │
│  │ (Tree)    │ │ Editor    │ │ (Body/Headers/Meta)│ │
│  └───────────┘ └───────────┘ └────────────────────┘ │
├─────────────────────────────────────────────────────┤
│                   Bridge Layer                       │
│  (Adapters: converts core types → GPUI view models)  │
├─────────────────────────────────────────────────────┤
│                 reqforge-core (lib)                   │
│  ┌────────┐ ┌──────────┐ ┌──────┐ ┌──────────────┐ │
│  │ HTTP   │ │ Environ- │ │Store │ │ Collections  │ │
│  │ Engine │ │ ments    │ │(JSON)│ │ & Folders    │ │
│  └────────┘ └──────────┘ └──────┘ └──────────────┘ │
└─────────────────────────────────────────────────────┘
```

The project is split into **two crates** inside a Cargo workspace:

| Crate | Purpose |
|---|---|
| `reqforge-core` | Headless library. All domain logic, HTTP execution, persistence, environment interpolation. Zero UI dependencies. Fully testable in isolation. |
| `reqforge-app` | GPUI binary. Thin UI shell that imports `reqforge-core` and renders everything with gpui. |

---

## 1. Workspace & Crate Layout

```
reqforge/
├── Cargo.toml                  # [workspace]
├── crates/
│   ├── reqforge-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models/
│   │       │   ├── mod.rs
│   │       │   ├── request.rs
│   │       │   ├── response.rs
│   │       │   ├── environment.rs
│   │       │   ├── collection.rs
│   │       │   └── folder.rs
│   │       ├── http/
│   │       │   ├── mod.rs
│   │       │   └── client.rs
│   │       ├── env/
│   │       │   ├── mod.rs
│   │       │   └── interpolator.rs
│   │       └── store/
│   │           ├── mod.rs
│   │           └── json_store.rs
│   └── reqforge-app/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── app_state.rs
│           ├── ui/
│           │   ├── mod.rs
│           │   ├── sidebar.rs
│           │   ├── request_editor.rs
│           │   ├── response_viewer.rs
│           │   ├── env_selector.rs
│           │   ├── tab_bar.rs
│           │   └── key_value_editor.rs
│           └── bridge/
│               ├── mod.rs
│               └── view_models.rs
├── data/                       # default workspace data (gitignored at runtime)
└── README.md
```

---

## 2. `reqforge-core` — Domain Models

### 2.1 `models/request.rs`

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
    pub enabled: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum BodyType {
    #[default]
    None,
    Raw { content: String, content_type: RawContentType },
    FormUrlEncoded(Vec<KeyValuePair>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RawContentType {
    #[default]
    Json,
    Xml,
    Text,
    Html,
}

/// The core, persistable request definition.
/// All string fields may contain `{{variable}}` placeholders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestDefinition {
    pub id: Uuid,
    pub name: String,
    pub method: HttpMethod,
    pub url: String,                         // e.g. "{{base_url}}/api/users"
    pub headers: Vec<KeyValuePair>,
    pub query_params: Vec<KeyValuePair>,
    pub body: BodyType,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl RequestDefinition {
    pub fn new(name: impl Into<String>, method: HttpMethod, url: impl Into<String>) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            method,
            url: url.into(),
            headers: Vec::new(),
            query_params: Vec::new(),
            body: BodyType::None,
            created_at: now,
            updated_at: now,
        }
    }
}
```

### 2.2 `models/response.rs`

```rust
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
```

### 2.3 `models/environment.rs`

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub key: String,
    pub value: String,
    pub secret: bool,           // mask in UI
    pub enabled: bool,
}

/// A named set of variables (e.g. "Development", "Staging", "Production").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub id: Uuid,
    pub name: String,
    pub variables: Vec<Variable>,
}

impl Environment {
    pub fn new(name: impl Into<String>) -> Self {
        Self { id: Uuid::new_v4(), name: name.into(), variables: Vec::new() }
    }

    pub fn to_map(&self) -> HashMap<String, String> {
        self.variables
            .iter()
            .filter(|v| v.enabled)
            .map(|v| (v.key.clone(), v.value.clone()))
            .collect()
    }
}
```

### 2.4 `models/collection.rs` + `models/folder.rs`

```rust
// folder.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: Uuid,
    pub name: String,
    pub children: Vec<CollectionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectionItem {
    Request(Uuid),              // references RequestDefinition.id
    Folder(Folder),
}

// collection.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::folder::CollectionItem;
use super::request::RequestDefinition;
use std::collections::HashMap;

/// A Collection owns an ordered tree of folders/requests
/// and a lookup table for the actual RequestDefinition objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub tree: Vec<CollectionItem>,
    pub requests: HashMap<Uuid, RequestDefinition>,
}

impl Collection {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            tree: Vec::new(),
            requests: HashMap::new(),
        }
    }

    pub fn add_request(&mut self, req: RequestDefinition, parent_folder: Option<Uuid>) {
        let id = req.id;
        self.requests.insert(id, req);
        let item = CollectionItem::Request(id);
        match parent_folder {
            Some(folder_id) => {
                Self::insert_into_folder(&mut self.tree, folder_id, item);
            }
            None => self.tree.push(item),
        }
    }

    fn insert_into_folder(items: &mut Vec<CollectionItem>, folder_id: Uuid, new_item: CollectionItem) -> bool {
        for item in items.iter_mut() {
            if let CollectionItem::Folder(folder) = item {
                if folder.id == folder_id {
                    folder.children.push(new_item);
                    return true;
                }
                if Self::insert_into_folder(&mut folder.children, folder_id, new_item.clone()) {
                    return true;
                }
            }
        }
        false
    }
}
```

---

## 3. `reqforge-core` — HTTP Engine

### 3.1 `http/client.rs`

The HTTP engine is fully async, headless, and testable. It takes a **resolved** request (all variables already interpolated).

```rust
use reqwest::Client;
use crate::models::request::{RequestDefinition, BodyType, RawContentType, KeyValuePair};
use crate::models::response::HttpResponse;
use std::time::Instant;

pub struct HttpEngine {
    client: Client,
}

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("URL parse error: {0}")]
    UrlParse(String),
}

impl HttpEngine {
    pub fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(false)   // make configurable later
            .build()
            .expect("failed to build HTTP client");
        Self { client }
    }

    /// Execute a fully-resolved RequestDefinition.
    /// Variables must already be interpolated before calling this.
    pub async fn execute(&self, req: &RequestDefinition) -> Result<HttpResponse, HttpError> {
        let method: reqwest::Method = req.method.to_string().parse().unwrap();

        let mut builder = self.client.request(method, &req.url);

        // Query params
        let enabled_params: Vec<(&str, &str)> = req.query_params
            .iter()
            .filter(|p| p.enabled)
            .map(|p| (p.key.as_str(), p.value.as_str()))
            .collect();
        builder = builder.query(&enabled_params);

        // Headers
        for h in req.headers.iter().filter(|h| h.enabled) {
            builder = builder.header(&h.key, &h.value);
        }

        // Body
        builder = match &req.body {
            BodyType::None => builder,
            BodyType::Raw { content, content_type } => {
                let mime = match content_type {
                    RawContentType::Json => "application/json",
                    RawContentType::Xml  => "application/xml",
                    RawContentType::Text => "text/plain",
                    RawContentType::Html => "text/html",
                };
                builder.header("Content-Type", mime).body(content.clone())
            }
            BodyType::FormUrlEncoded(pairs) => {
                let form: Vec<(&str, &str)> = pairs.iter()
                    .filter(|p| p.enabled)
                    .map(|p| (p.key.as_str(), p.value.as_str()))
                    .collect();
                builder.form(&form)
            }
        };

        let start = Instant::now();
        let response = builder.send().await?;
        let elapsed = start.elapsed();

        let status = response.status().as_u16();
        let status_text = response.status().canonical_reason().unwrap_or("").to_string();
        let headers = response.headers().iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let bytes = response.bytes().await?.to_vec();
        let size = bytes.len();
        let body_text = String::from_utf8(bytes.clone()).ok();

        Ok(HttpResponse {
            status,
            status_text,
            headers,
            body: bytes,
            body_text,
            size_bytes: size,
            elapsed,
        })
    }
}
```

---

## 4. `reqforge-core` — Environment Interpolation

### 4.1 `env/interpolator.rs`

This replaces `{{variable_name}}` placeholders in any string field of a `RequestDefinition`.

```rust
use regex::Regex;
use std::collections::HashMap;
use crate::models::request::{RequestDefinition, BodyType, KeyValuePair};

pub struct Interpolator;

impl Interpolator {
    /// Resolve all `{{var}}` placeholders in a request definition,
    /// returning a new owned copy with concrete values.
    pub fn resolve(
        req: &RequestDefinition,
        vars: &HashMap<String, String>,
    ) -> RequestDefinition {
        let mut resolved = req.clone();
        resolved.url = Self::replace(&resolved.url, vars);
        Self::resolve_pairs(&mut resolved.headers, vars);
        Self::resolve_pairs(&mut resolved.query_params, vars);
        resolved.body = match &resolved.body {
            BodyType::None => BodyType::None,
            BodyType::Raw { content, content_type } => BodyType::Raw {
                content: Self::replace(content, vars),
                content_type: content_type.clone(),
            },
            BodyType::FormUrlEncoded(pairs) => {
                let mut p = pairs.clone();
                Self::resolve_pairs(&mut p, vars);
                BodyType::FormUrlEncoded(p)
            }
        };
        resolved
    }

    fn replace(input: &str, vars: &HashMap<String, String>) -> String {
        let re = Regex::new(r"\{\{(\w+)\}\}").unwrap(); // compile once in real code
        re.replace_all(input, |caps: &regex::Captures| {
            let key = &caps[1];
            vars.get(key).cloned().unwrap_or_else(|| format!("{{{{{}}}}}", key))
        }).to_string()
    }

    fn resolve_pairs(pairs: &mut Vec<KeyValuePair>, vars: &HashMap<String, String>) {
        for pair in pairs.iter_mut() {
            pair.key = Self::replace(&pair.key, vars);
            pair.value = Self::replace(&pair.value, vars);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_interpolation() {
        let mut vars = HashMap::new();
        vars.insert("base_url".into(), "https://api.example.com".into());
        vars.insert("token".into(), "abc123".into());

        let result = Interpolator::replace("{{base_url}}/users?t={{token}}", &vars);
        assert_eq!(result, "https://api.example.com/users?t=abc123");
    }

    #[test]
    fn test_missing_var_preserved() {
        let vars = HashMap::new();
        let result = Interpolator::replace("{{missing}}", &vars);
        assert_eq!(result, "{{missing}}");
    }
}
```

---

## 5. `reqforge-core` — Persistence (JSON Store)

### 5.1 `store/json_store.rs`

Simple file-based persistence. Each collection is a JSON file. Environments are stored in a separate file.

```rust
use std::path::{Path, PathBuf};
use crate::models::collection::Collection;
use crate::models::environment::Environment;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

/// Layout on disk:
/// workspace_dir/
///   environments.json      → Vec<Environment>
///   collections/
///     {collection_id}.json → Collection
pub struct JsonStore {
    root: PathBuf,
}

impl JsonStore {
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, StoreError> {
        let root = root.into();
        std::fs::create_dir_all(root.join("collections"))?;
        Ok(Self { root })
    }

    // --- Environments ---

    pub fn load_environments(&self) -> Result<Vec<Environment>, StoreError> {
        let path = self.root.join("environments.json");
        if !path.exists() { return Ok(Vec::new()); }
        let data = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save_environments(&self, envs: &[Environment]) -> Result<(), StoreError> {
        let json = serde_json::to_string_pretty(envs)?;
        std::fs::write(self.root.join("environments.json"), json)?;
        Ok(())
    }

    // --- Collections ---

    pub fn list_collections(&self) -> Result<Vec<Collection>, StoreError> {
        let dir = self.root.join("collections");
        let mut collections = Vec::new();
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            if entry.path().extension().map(|e| e == "json").unwrap_or(false) {
                let data = std::fs::read_to_string(entry.path())?;
                collections.push(serde_json::from_str(&data)?);
            }
        }
        Ok(collections)
    }

    pub fn save_collection(&self, col: &Collection) -> Result<(), StoreError> {
        let path = self.root.join("collections").join(format!("{}.json", col.id));
        let json = serde_json::to_string_pretty(col)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn delete_collection(&self, col: &Collection) -> Result<(), StoreError> {
        let path = self.root.join("collections").join(format!("{}.json", col.id));
        if path.exists() { std::fs::remove_file(path)?; }
        Ok(())
    }
}
```

---

## 6. `reqforge-core` — Public Facade

### 6.1 `lib.rs`

```rust
pub mod models;
pub mod http;
pub mod env;
pub mod store;

use models::{collection::Collection, environment::Environment, request::RequestDefinition, response::HttpResponse};
use http::client::{HttpEngine, HttpError};
use env::interpolator::Interpolator;
use store::json_store::{JsonStore, StoreError};
use std::collections::HashMap;

/// The top-level headless API surface.
/// The UI crate only talks to this.
pub struct ReqForgeCore {
    pub engine: HttpEngine,
    pub store: JsonStore,
    pub environments: Vec<Environment>,
    pub collections: Vec<Collection>,
    pub active_environment_id: Option<uuid::Uuid>,
}

impl ReqForgeCore {
    pub fn open(workspace_dir: impl Into<std::path::PathBuf>) -> Result<Self, StoreError> {
        let store = JsonStore::open(workspace_dir)?;
        let environments = store.load_environments()?;
        let collections = store.list_collections()?;
        Ok(Self {
            engine: HttpEngine::new(),
            store,
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

    /// Execute a request with environment interpolation.
    pub async fn execute_request(&self, req: &RequestDefinition) -> Result<HttpResponse, HttpError> {
        let vars = self.active_vars();
        let resolved = Interpolator::resolve(req, &vars);
        self.engine.execute(&resolved).await
    }

    /// Persist all state to disk.
    pub fn save_all(&self) -> Result<(), StoreError> {
        self.store.save_environments(&self.environments)?;
        for col in &self.collections {
            self.store.save_collection(col)?;
        }
        Ok(())
    }
}
```

---

## 7. `reqforge-app` — GPUI Application

### 7.1 Dependencies (`reqforge-app/Cargo.toml`)

```toml
[package]
name = "reqforge-app"
version = "0.1.0"
edition = "2021"

[dependencies]
reqforge-core = { path = "../reqforge-core" }
gpui = { git = "https://github.com/zed-industries/zed", package = "gpui" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1.18.1", features = ["v4"] }
parking_lot = "0.12.5"
dirs = "6"
```

> **Note:** GPUI is not yet published as a standalone crate. You will either:
> (a) Use a git dependency pointing at Zed's repo and selecting the `gpui` package, or
> (b) Vendor the `gpui` crate locally. Track [github.com/zed-industries/zed](https://github.com/zed-industries/zed) for updates.

### 7.2 `app_state.rs` — Shared State

```rust
use reqforge_core::ReqForgeCore;
use reqforge_core::models::request::RequestDefinition;
use reqforge_core::models::response::HttpResponse;
use gpui::*;
use std::sync::Arc;
use parking_lot::RwLock;

/// Wraps the headless core and adds UI-specific state.
pub struct AppState {
    pub core: Arc<RwLock<ReqForgeCore>>,
    pub open_tabs: Vec<OpenTab>,
    pub active_tab_index: Option<usize>,
}

pub struct OpenTab {
    pub request_id: uuid::Uuid,
    pub collection_id: uuid::Uuid,
    pub draft: RequestDefinition,       // in-progress edits (unsaved)
    pub last_response: Option<HttpResponse>,
    pub is_loading: bool,
    pub is_dirty: bool,
}
```

### 7.3 Main Window Layout

The app uses a classic three-panel layout:

```
┌────────────────────────────────────────────────────────────┐
│ Toolbar  [Env Selector ▾]                       [Save All]│
├──────────┬─────────────────────────────────────────────────┤
│          │  Tab Bar  [ GET /users ][ POST /login ]         │
│ Sidebar  │─────────────────────────────────────────────────│
│          │  Request Editor                                 │
│ ▸ My API │  ┌─────────────────────────────────────────┐    │
│   GET /u │  │ [GET ▾] [{{base_url}}/api/users    ] [▶]│    │
│   POST / │  ├─────────────────────────────────────────┤    │
│          │  │ Params │ Headers │ Body │                │    │
│ ▸ Auth   │  │ ┌─────┬───────┬───┐                     │    │
│   ...    │  │ │ Key │ Value │ ✓ │                     │    │
│          │  │ └─────┴───────┴───┘                     │    │
│          │  ├─────────────────────────────────────────┤    │
│          │  │ Response                                │    │
│          │  │ 200 OK  ·  142ms  ·  3.2 KB             │    │
│          │  │ Body │ Headers │                         │    │
│          │  │ {                                        │    │
│          │  │   "users": [...]                        │    │
│          │  │ }                                        │    │
│          │  └─────────────────────────────────────────┘    │
└──────────┴─────────────────────────────────────────────────┘
```

### 7.4 Key GPUI Components

Each UI region is a `gpui::View<T>` with its own `Render` impl. Here's a mapping:

| Component | GPUI Entity | Responsibilities |
|---|---|---|
| `Sidebar` | `View<SidebarPanel>` | Renders collection tree, drag-drop reorder, context menus (rename, delete, new folder) |
| `TabBar` | `View<TabBar>` | Horizontal tabs for open requests, close/dirty indicators |
| `RequestEditor` | `View<RequestEditor>` | Method dropdown, URL input, sub-tabs (Params, Headers, Body), Send button |
| `KeyValueEditor` | `View<KeyValueEditor>` | Reusable table for headers, params, form-data. Add/remove/toggle rows. |
| `BodyEditor` | `View<BodyEditor>` | Text area for raw body with content-type selector |
| `ResponseViewer` | `View<ResponseViewer>` | Status badge, timing, size, sub-tabs (Body, Headers). Syntax highlighted JSON. |
| `EnvSelector` | `View<EnvSelector>` | Dropdown in toolbar listing environments, "Manage Environments" action |
| `EnvEditorModal` | `View<EnvEditorModal>` | Modal dialog for CRUD on environments and their variables |

### 7.5 GPUI Rendering Sketch — `RequestEditor`

```rust
use gpui::*;

pub struct RequestEditor {
    request: reqforge_core::models::request::RequestDefinition,
    active_sub_tab: SubTab,
    url_input: View<TextInput>,
    // ...
}

#[derive(PartialEq)]
enum SubTab { Params, Headers, Body }

impl Render for RequestEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(
                // URL bar row
                div()
                    .flex()
                    .gap_2()
                    .p_2()
                    .child(self.render_method_dropdown(cx))
                    .child(self.url_input.clone())
                    .child(
                        div()
                            .px_4()
                            .py_1()
                            .bg(rgb(0x3B82F6))
                            .text_color(white())
                            .rounded_md()
                            .cursor_pointer()
                            .child("Send")
                            .on_click(cx.listener(|this, _, cx| {
                                this.send_request(cx);
                            }))
                    )
            )
            .child(
                // Sub-tab bar
                div().flex().gap_1().p_1()
                    .child(self.render_sub_tab("Params", SubTab::Params, cx))
                    .child(self.render_sub_tab("Headers", SubTab::Headers, cx))
                    .child(self.render_sub_tab("Body", SubTab::Body, cx))
            )
            .child(
                // Sub-tab content
                match self.active_sub_tab {
                    SubTab::Params  => self.render_params_editor(cx).into_any_element(),
                    SubTab::Headers => self.render_headers_editor(cx).into_any_element(),
                    SubTab::Body    => self.render_body_editor(cx).into_any_element(),
                }
            )
    }
}
```

### 7.6 `main.rs` — Bootstrap

```rust
use gpui::*;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        // Load headless core
        let workspace_dir = dirs::data_local_dir()
            .unwrap()
            .join("reqforge");
        let core = reqforge_core::ReqForgeCore::open(&workspace_dir)
            .expect("Failed to open workspace");

        let app_state = std::sync::Arc::new(parking_lot::RwLock::new(
            crate::app_state::AppState {
                core: std::sync::Arc::new(parking_lot::RwLock::new(core)),
                open_tabs: Vec::new(),
                active_tab_index: None,
            }
        ));

        cx.open_window(
            WindowOptions {
                bounds: Some(Bounds {
                    origin: point(px(100.0), px(100.0)),
                    size: size(px(1400.0), px(900.0)),
                }),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|cx| RootView::new(app_state.clone(), cx))
            }
        );
    });
}
```

---

## 8. Async Execution Flow (Send Button)

```
 User clicks "Send"
       │
       ▼
 RequestEditor::send_request(cx)
       │
       ├─ 1. Clone draft RequestDefinition
       ├─ 2. Set tab.is_loading = true, cx.notify()
       │
       ▼
 cx.spawn(async move {                          ← GPUI async task
       │
       ├─ 3. let core = app_state.core.read();
       ├─ 4. let response = core.execute_request(&draft).await;
       │
       └─ move result back to main thread
 }).detach();
       │
       ▼
 cx.update_model(|tab| {
       tab.last_response = Some(response);
       tab.is_loading = false;
       cx.notify();                              ← triggers re-render
 })
```

---

## 9. Implementation Phases

### Phase 1: Core Library

- [ ] Set up Cargo workspace with `reqforge-core` and `reqforge-app` crates
- [ ] Define `HttpMethod`, `KeyValuePair`, `BodyType`, `RawContentType` enums/structs
- [ ] Define `RequestDefinition` model with constructor
- [ ] Define `HttpResponse` model with `pretty_body()` and `is_success()`
- [ ] Define `Variable` and `Environment` models with `to_map()`
- [ ] Define `Folder`, `CollectionItem`, and `Collection` models
- [ ] Implement `Collection::add_request()` with folder insertion logic
- [ ] Implement `HttpEngine` using `reqwest` — method, URL, headers, query params, body
- [ ] Implement `Interpolator::resolve()` for `{{variable}}` replacement in all request fields
- [ ] Write unit tests for `Interpolator` (basic vars, missing vars, nested, edge cases)
- [ ] Implement `JsonStore::open()` with directory creation
- [ ] Implement `JsonStore::load_environments()` / `save_environments()`
- [ ] Implement `JsonStore::list_collections()` / `save_collection()` / `delete_collection()`
- [ ] Write `ReqForgeCore` facade: `open()`, `active_vars()`, `execute_request()`, `save_all()`
- [ ] Write integration tests for `HttpEngine` using `wiremock` mock server
- [ ] Write persistence round-trip tests using `tempfile`
- [ ] (Optional) Write a CLI smoke-test binary that sends a request from a JSON file

### Phase 2: GPUI Shell

- [ ] Bootstrap GPUI `App::new().run()` with an empty window
- [ ] Create `AppState` struct wrapping `ReqForgeCore` in `Arc<RwLock<>>`
- [ ] Create `RootView` with three-panel layout (sidebar + main area)
- [ ] Build `KeyValueEditor` reusable component (add/remove/toggle rows)
- [ ] Build `RequestEditor` — method dropdown, URL text input, sub-tabs (Params, Headers, Body)
- [ ] Build `BodyEditor` — text area with content-type selector (JSON, XML, Text, HTML)
- [ ] Build `ResponseViewer` — status badge, timing, size, body/headers sub-tabs
- [ ] Wire up Send button: spawn async task → call `core.execute_request()` → update `last_response`
- [ ] Verify end-to-end: type URL → hit Send → see response rendered

### Phase 3: Collections & Sidebar

- [ ] Build `SidebarPanel` tree view rendering `CollectionItem` recursively
- [ ] Implement collapse/expand for folders in the tree
- [ ] Implement "New Request" action (context menu or button)
- [ ] Implement "New Folder" action
- [ ] Implement "Rename" action (inline editing)
- [ ] Implement "Delete" action with confirmation
- [ ] Build `TabBar` — horizontal tabs for open requests
- [ ] Implement close tab / dirty indicator on tabs
- [ ] Clicking a request in sidebar opens it in a new tab (or focuses existing tab)
- [ ] Auto-save request to `JsonStore` on send
- [ ] Manual save via Ctrl+S keybinding
- [ ] Implement drag-and-drop reordering in sidebar

### Phase 4: Environments

- [ ] Build `EnvSelector` dropdown in the toolbar listing all environments
- [ ] Switching environment updates `core.active_environment_id` and re-resolves preview
- [ ] Build `EnvEditorModal` — create, rename, delete environments
- [ ] Build variable table inside `EnvEditorModal` (key/value/secret/enabled per row)
- [ ] Highlight `{{variables}}` in URL and header input fields with distinct styling
- [ ] Implement variable autocomplete in text fields (suggest from active environment)

### Phase 5: Polish

- [ ] Keyboard shortcuts: Ctrl+Enter = Send, Ctrl+S = Save, Ctrl+N = New Request
- [ ] Syntax highlighting for JSON response body
- [ ] Status code color coding (green 2xx, yellow 3xx, red 4xx/5xx)
- [ ] Loading spinner / indicator during in-flight request
- [ ] User-facing error messages for network errors, timeouts, parse failures
- [ ] Theme support (dark mode / light mode toggle or OS detection)

---

## 10. Key Dependencies

### `reqforge-core/Cargo.toml`

```toml
[package]
name = "reqforge-core"
version = "0.1.0"
edition = "2021"

[dependencies]
reqwest = { version = "0.13.1", features = ["json", "rustls-tls"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1.18.1", features = ["v4", "serde"] }
chrono = { version = "0.4.42", features = ["serde"] }
regex = "1.12.2"
thiserror = "2.0.17"

[dev-dependencies]
wiremock = "0.6.5"
tokio-test = "0.4"
tempfile = "3"
```

### `reqforge-app/Cargo.toml`

```toml
[package]
name = "reqforge-app"
version = "0.1.0"
edition = "2021"

[dependencies]
reqforge-core = { path = "../reqforge-core" }
gpui = { git = "https://github.com/zed-industries/zed", package = "gpui" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1.18.1", features = ["v4"] }
parking_lot = "0.12.5"
dirs = "6"
```

---

## 11. Testing Strategy

| Layer | Approach |
|---|---|
| **Models** | Unit tests for serialization round-trips, collection tree manipulation |
| **Interpolator** | Unit tests: basic vars, nested, missing vars, edge cases (empty string, special chars) |
| **HttpEngine** | Integration tests using `wiremock` — spin up a local mock server, assert request/response mapping |
| **JsonStore** | Tests with `tempfile` — write, read back, verify integrity |
| **ReqForgeCore** | End-to-end: create collection → add request → set env → execute → verify response |
| **GPUI (manual)** | Manual testing initially; GPUI doesn't yet have a mature UI testing framework |
