# Plan: ReqForge UI with gpui-component + Zero-Copy Architecture

## Context

ReqForge has a solid headless core (`reqforge-core`, 120+ tests) but the GPUI UI is blocked because raw GPUI from Zed's monorepo has unresolvable dependency conflicts. The `gpui-component` crate (v0.5.1 on crates.io) bundles GPUI and provides 60+ production components (Input, Select, Tab, Table, Tree, Button, Dialog, Sidebar, etc.). This plan replaces the stub UI with a real gpui-component UI and refactors the core+app boundary to be zero-copy from the start.

---

## Phase 0: Zero-Copy Core Refactor

Refactor `reqforge-core` models and HTTP client to eliminate unnecessary allocations at the core-to-UI boundary. This happens **before** UI work so all downstream code benefits.

### 0.1 Add `bytes` crate to workspace dependencies

**File:** `Cargo.toml` (workspace root)
- Add `bytes = "1"` to `[workspace.dependencies]`
- Add `bytes.workspace = true` to `reqforge-core/Cargo.toml`

### 0.2 Refactor `HttpResponse` to use `bytes::Bytes`

**File:** `crates/reqforge-core/src/models/response.rs`

```rust
use bytes::Bytes;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Bytes,              // was Vec<u8> — now refcounted, cheap clone
    pub size_bytes: usize,
    pub elapsed: Duration,
}

impl HttpResponse {
    /// Borrow body as UTF-8 str without allocating. Returns None if not valid UTF-8.
    pub fn body_text(&self) -> Option<&str> {
        std::str::from_utf8(&self.body).ok()
    }

    /// Pretty-print JSON body if applicable (allocates only when called)
    pub fn pretty_body(&self) -> Option<String> {
        let text = self.body_text()?;
        let val: serde_json::Value = serde_json::from_str(text).ok()?;
        serde_json::to_string_pretty(&val).ok()
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }
}
```

**Key change:** Remove `body_text: Option<String>` field — replace with `body_text(&self) -> Option<&str>` method that borrows from `Bytes`. No allocation on the hot path.

### 0.3 Update `HttpEngine::execute()` to use `Bytes` directly

**File:** `crates/reqforge-core/src/http/client.rs`

Replace:
```rust
let body = response.bytes().await?;
let size = body.len();
let body_text = std::str::from_utf8(body.as_ref()).ok().map(ToOwned::to_owned);
let bytes = body.to_vec();
```

With:
```rust
let body: Bytes = response.bytes().await?;
let size = body.len();
// No body_text allocation — callers use body_text() method which borrows
```

### 0.4 Refactor `Interpolator::resolve()` to use `Cow<str>`

**File:** `crates/reqforge-core/src/env/interpolator.rs`

Change `replace()` to return `Cow<str>`:
```rust
fn replace<'a>(input: &'a str, vars: &HashMap<String, String>) -> Cow<'a, str> {
    if !input.contains("{{") {
        return Cow::Borrowed(input);  // fast path: no variables, zero alloc
    }
    // slow path: regex replace (allocates only when needed)
    let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
    Cow::Owned(re.replace_all(input, |caps: &regex::Captures| {
        // ...
    }).to_string())
}
```

This keeps the existing `resolve()` public API (returns owned `RequestDefinition`) but avoids internal string allocations when there are no `{{variables}}` in a field.

### 0.5 Update all callers of `HttpResponse`

Grep for `.body_text` field access across the codebase (history snapshots, response viewer, tests) and update to use `.body_text()` method. Update any `.body.to_vec()` / `.body.clone()` patterns to use `Bytes::clone()` (which is just a refcount bump).

### 0.6 Run all 120+ tests — ensure nothing breaks

---

## Phase 1: gpui-component Dependency Setup

### 1.1 Update `reqforge-app/Cargo.toml`

**File:** `crates/reqforge-app/Cargo.toml`

Replace the commented-out GPUI dependency with gpui-component:

```toml
[package]
name = "reqforge-app"
version.workspace = true
edition.workspace = true

[dependencies]
reqforge-core = { path = "../reqforge-core" }
gpui-component = "0.5"
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
bytes = "1"
```

Remove `parking_lot`, `dirs`, `tokio` direct deps — gpui-component bundles GPUI which has its own async runtime. We'll use GPUI's `cx.spawn()` / `cx.background_spawn()` instead of tokio.

### 1.2 Verify `cargo check -p reqforge-app` compiles

Resolve any dependency conflicts. The key advantage of `gpui-component` over raw GPUI is that it's published to crates.io with resolved deps.

### 1.3 Update workspace root `Cargo.toml` if needed

If gpui-component re-exports `gpui`, add it to workspace deps for shared use.

---

## Phase 2: App State — Entity-Based Architecture

Replace the current `Arc<RwLock<AppState>>` pattern with GPUI's `Entity<T>` system for reactive, zero-copy state management.

### 2.1 Rewrite `app_state.rs` as GPUI Entity

**File:** `crates/reqforge-app/src/app_state.rs`

```rust
use gpui::*;
use reqforge_core::ReqForgeCore;
use uuid::Uuid;

pub struct AppState {
    pub core: ReqForgeCore,
    pub tabs: Vec<TabState>,
    pub active_tab: Option<usize>,
    pub active_env_id: Option<Uuid>,
}

pub struct TabState {
    pub request_id: Uuid,
    pub collection_id: Uuid,
    pub url_input: Entity<InputState>,      // gpui-component input state
    pub method: HttpMethod,
    pub headers: Vec<KeyValueRow>,
    pub params: Vec<KeyValueRow>,
    pub body_input: Entity<InputState>,
    pub last_response: Option<reqforge_core::HttpResponse>,
    pub is_loading: bool,
    pub is_dirty: bool,
}
```

**Key design:** `Entity<AppState>` is the single source of truth. All UI components receive `&Entity<AppState>` and read/update through it. No `Arc<RwLock>` — GPUI entities provide safe concurrent access with automatic re-render notifications.

### 2.2 Create `KeyValueRow` as zero-copy view model

```rust
pub struct KeyValueRow {
    pub key_input: Entity<InputState>,
    pub value_input: Entity<InputState>,
    pub enabled: bool,
}
```

Input state lives as GPUI entities — text is managed by gpui-component's `InputState` (backed by a Rope data structure), not duplicated as `String` clones.

---

## Phase 3: Main Window Bootstrap

### 3.1 Rewrite `main.rs`

**File:** `crates/reqforge-app/src/main.rs`

```rust
use gpui::*;
use gpui_component::init as init_gpui_component;

fn main() {
    App::new().run(|cx: &mut App| {
        init_gpui_component(cx);  // register gpui-component actions/themes

        // Load core
        let workspace_dir = dirs::data_local_dir()
            .unwrap_or_else(|| ".".into())
            .join("reqforge");
        let core = ReqForgeCore::open(&workspace_dir)
            .expect("Failed to open workspace");

        let app_state = cx.new(|_| AppState {
            core,
            tabs: Vec::new(),
            active_tab: None,
            active_env_id: None,
        });

        cx.open_window(
            WindowOptions { /* size, title */ ..Default::default() },
            |window, cx| cx.new(|cx| RootView::new(app_state, window, cx)),
        ).unwrap();
    });
}
```

### 3.2 Create `RootView` with three-panel layout

**File:** `crates/reqforge-app/src/ui/root.rs`

```rust
pub struct RootView {
    app_state: Entity<AppState>,
    sidebar: Entity<SidebarPanel>,
    // main content area renders based on active tab
}

impl Render for RootView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .child(self.render_sidebar(window, cx))     // left panel
            .child(self.render_main_area(window, cx))    // right panel
    }
}
```

---

## Phase 4: UI Components (using gpui-component)

### 4.1 Sidebar — Collection Tree

**File:** `crates/reqforge-app/src/ui/sidebar.rs`

Uses gpui-component's `Tree` component to render the collection/folder/request hierarchy.

- Each `Collection` maps to a root tree node
- Each `Folder` maps to a collapsible tree node
- Each `RequestDefinition` maps to a leaf node with method badge + name
- Click opens request in a tab (or focuses existing tab)
- Context menu: New Request, New Folder, Rename, Delete

### 4.2 Tab Bar

**File:** `crates/reqforge-app/src/ui/tab_bar.rs`

Uses gpui-component's `Tab` / `TabList` components.

- One tab per open request
- Shows method badge + request name
- Dirty indicator (dot) when unsaved
- Close button per tab
- Click switches active tab

### 4.3 Request Editor

**File:** `crates/reqforge-app/src/ui/request_editor.rs`

The main editing area. Composed of:

- **URL bar row:** `Select` for HTTP method + `Input` for URL + `Button` "Send"
- **Sub-tab bar:** `Tab` components for Params / Headers / Body
- **Key-value table:** Custom component using `Input` pairs in a vertical list
- **Body editor:** `Input` (multiline) with content-type `Select`

The `Send` button handler:
```rust
fn on_send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
    let weak = cx.entity().downgrade();
    let app_state = self.app_state.clone();

    // Build RequestDefinition from current input states (reads from Entity<InputState>)
    let req = self.build_request(cx);

    // Set loading
    app_state.update(cx, |state, cx| {
        if let Some(tab) = state.active_tab_mut() {
            tab.is_loading = true;
        }
        cx.notify();
    });

    // Spawn async — uses GPUI's integrated async, not tokio
    cx.spawn(async move |cx| {
        let response = {
            let core = app_state.read(cx)?.core;
            // background_spawn for the actual HTTP call
            core.execute_request(&req).await
        };

        app_state.update(cx, |state, cx| {
            if let Some(tab) = state.active_tab_mut() {
                tab.last_response = response.ok();
                tab.is_loading = false;
            }
            cx.notify();
        }).ok();
    }).detach();
}
```

### 4.4 Response Viewer

**File:** `crates/reqforge-app/src/ui/response_viewer.rs`

Displays the response after a request completes:

- **Status bar:** Status code (color-coded badge), elapsed time, response size
- **Sub-tabs:** Body / Headers
- **Body tab:** Pretty-printed JSON (using `response.pretty_body()`), or raw text
  - Uses gpui-component `Input` (multiline, readonly) or a `div` with pre-formatted text
- **Headers tab:** Table of response headers

**Zero-copy:** Response body is `Bytes` (refcounted). The viewer borrows `&str` via `body_text()` — no allocation to display text.

### 4.5 Environment Selector

**File:** `crates/reqforge-app/src/ui/env_selector.rs`

Uses gpui-component's `Select` dropdown in the toolbar:

- Lists all environments from `core.environments`
- "No Environment" option
- Selecting updates `app_state.active_env_id`
- "Manage Environments" option opens modal

### 4.6 Environment Editor Modal

**File:** `crates/reqforge-app/src/ui/env_editor_modal.rs`

Uses gpui-component's `Dialog`:

- List of environments (left)
- Variable table (right): key/value `Input` pairs with enabled `Checkbox` and secret `Switch`
- Add/Remove/Rename buttons
- Save/Cancel footer

### 4.7 Key-Value Editor (Reusable)

**File:** `crates/reqforge-app/src/ui/key_value_editor.rs`

Reusable component for headers, query params, form data:

- Each row: `Input` (key) + `Input` (value) + `Checkbox` (enabled) + delete `Button`
- "Add Row" button at bottom
- Zero-copy: Input states are `Entity<InputState>`, text lives in gpui-component's Rope, never cloned until save/send

---

## Phase 5: Bridge Layer — Core <-> UI Conversion

### 5.1 `build_request()` — Read Entity<InputState> -> RequestDefinition

**File:** `crates/reqforge-app/src/bridge.rs`

Only allocates `String`s at the **ownership boundary** (when building a `RequestDefinition` to send/persist):

```rust
pub fn build_request_from_tab(tab: &TabState, cx: &App) -> RequestDefinition {
    RequestDefinition {
        id: tab.request_id,
        name: tab.url_input.read(cx).text().to_string(), // alloc here: ownership boundary
        method: tab.method.clone(),
        url: tab.url_input.read(cx).text().to_string(),
        headers: tab.headers.iter().map(|row| KeyValuePair {
            key: row.key_input.read(cx).text().to_string(),
            value: row.value_input.read(cx).text().to_string(),
            enabled: row.enabled,
            description: None,
        }).collect(),
        // ... same for params, body
    }
}
```

### 5.2 `populate_tab()` — RequestDefinition -> Entity<InputState>

Reverse direction when opening a saved request:

```rust
pub fn populate_tab_from_request(
    req: &RequestDefinition, window: &mut Window, cx: &mut App,
) -> TabState {
    let url_input = cx.new(|cx| {
        InputState::new(window, cx)
            .default_value(&req.url)  // set once, no further cloning
    });
    // ... create Entity<InputState> for each field
}
```

---

## Phase 6: File Changes Summary

### Files to modify:

| File | Change |
|---|---|
| `Cargo.toml` | Add `bytes = "1"` to workspace deps |
| `crates/reqforge-core/Cargo.toml` | Add `bytes.workspace = true` |
| `crates/reqforge-core/src/models/response.rs` | `Vec<u8>` -> `Bytes`, remove `body_text` field, add `body_text()` method |
| `crates/reqforge-core/src/http/client.rs` | Remove `.to_vec()`, use `Bytes` directly |
| `crates/reqforge-core/src/env/interpolator.rs` | Internal `replace()` returns `Cow<str>` |
| `crates/reqforge-core/src/models/history.rs` | Update `ResponseSnapshot::from()` for new `HttpResponse` |
| `crates/reqforge-core/src/lib.rs` | Update callers of `body_text` |
| All tests referencing `body_text` field | Update to use `.body_text()` method |
| `crates/reqforge-app/Cargo.toml` | Replace deps with `gpui-component = "0.5"` |
| `crates/reqforge-app/src/main.rs` | Rewrite with `App::new().run()`, gpui-component init |
| `crates/reqforge-app/src/app_state.rs` | Rewrite as GPUI Entity-based state |

### Files to create (new):

| File | Purpose |
|---|---|
| `crates/reqforge-app/src/ui/root.rs` | RootView with three-panel layout |
| `crates/reqforge-app/src/ui/sidebar.rs` | Tree-based collection browser |
| `crates/reqforge-app/src/ui/tab_bar.rs` | Tab component for open requests |
| `crates/reqforge-app/src/ui/request_editor.rs` | Rewrite: real GPUI components |
| `crates/reqforge-app/src/ui/response_viewer.rs` | Rewrite: real GPUI components |
| `crates/reqforge-app/src/ui/key_value_editor.rs` | Rewrite: real GPUI components |
| `crates/reqforge-app/src/ui/env_selector.rs` | Rewrite: real GPUI Select |
| `crates/reqforge-app/src/ui/env_editor_modal.rs` | Rewrite: real GPUI Dialog |
| `crates/reqforge-app/src/bridge.rs` | Core <-> UI type conversion |

### Files to delete:

| File | Reason |
|---|---|
| `crates/reqforge-app/src/ui/body_editor.rs` | Merged into request_editor |

---

## Phase 7: Implementation Order

Execute in this exact order:

1. **Phase 0** — Zero-copy core refactor (`Bytes`, `Cow`, remove `body_text` field)
2. **Run `cargo test -p reqforge-core`** — all 120+ tests must pass
3. **Phase 1** — Add gpui-component dep, verify `cargo check`
4. **Phase 2** — Rewrite `app_state.rs` as GPUI Entity
5. **Phase 3** — Rewrite `main.rs` bootstrap
6. **Phase 4.1-4.2** — Sidebar + Tab Bar (structural layout)
7. **Phase 4.3** — Request Editor (the core UI)
8. **Phase 4.4** — Response Viewer
9. **Phase 4.5-4.6** — Environment Selector + Modal
10. **Phase 5** — Bridge layer (build_request / populate_tab)
11. **Wire up Send button** — async execution with GPUI's cx.spawn
12. **Run `cargo build`** — verify everything compiles
13. **Manual test** — launch app, send a request, verify response renders

---

## Verification

1. `cargo test -p reqforge-core` — all 120+ tests pass (zero-copy refactor didn't break anything)
2. `cargo check -p reqforge-app` — compiles with gpui-component
3. `cargo run -p reqforge-app` — window opens with three-panel layout
4. Manual: type URL -> select method -> click Send -> see response
5. Manual: switch environments, verify variable interpolation
6. Manual: open/close tabs, verify state management

---

## Zero-Copy Architecture Summary

| Layer | Pattern | Why |
|---|---|---|
| HTTP response body | `bytes::Bytes` (refcounted) | Zero-cost clone, borrow as `&str` |
| Body display | `body_text() -> Option<&str>` | Borrows from Bytes, no allocation |
| Input text | `Entity<InputState>` (Rope-backed) | gpui-component manages text internally, no String cloning |
| Core -> UI | Read `Entity<InputState>` via `.read(cx).text()` | Borrow, don't own |
| UI -> Core | `build_request()` allocates `String` only at ownership boundary | Single allocation point |
| Interpolation | `Cow<str>` fast-path when no `{{vars}}` | Skip alloc when string unchanged |
| Shared state | `Entity<AppState>` (GPUI refcounted) | No `Arc<RwLock>` — GPUI handles concurrency |
