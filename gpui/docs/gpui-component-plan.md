# Plan: ReqForge UI with gpui-component + Zero-Copy Architecture

## Context

ReqForge has a solid headless core (`reqforge-core`, 120+ tests) but the GPUI UI is blocked because raw GPUI from Zed's monorepo has unresolvable dependency conflicts. The `gpui-component` crate (v0.5.1 on crates.io) bundles GPUI and provides 60+ production components (Input, Select, Tab, Table, Tree, Button, Dialog, Sidebar, etc.). This plan replaces the stub UI with a real gpui-component UI and refactors the core+app boundary to be zero-copy from the start.

---

## Phase 0: Zero-Copy Core Refactor

Refactor `reqforge-core` models and HTTP client to eliminate unnecessary allocations at the core-to-UI boundary. This happens **before** UI work so all downstream code benefits.

### 0.1 Add `bytes` crate to workspace dependencies

- [ ] Add `bytes = "1"` to `[workspace.dependencies]` in `Cargo.toml` (workspace root)
- [ ] Add `bytes.workspace = true` to `crates/reqforge-core/Cargo.toml`

### 0.2 Refactor `HttpResponse` to use `bytes::Bytes`

**File:** `crates/reqforge-core/src/models/response.rs`

- [ ] Change `body: Vec<u8>` field to `body: Bytes`
- [ ] Remove `body_text: Option<String>` field entirely
- [ ] Add `body_text(&self) -> Option<&str>` method that borrows from `Bytes` via `std::str::from_utf8`
- [ ] Update `pretty_body()` to call `self.body_text()` instead of `self.body_text.as_ref()`

Target struct:
```rust
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Bytes,              // was Vec<u8> — now refcounted, cheap clone
    pub size_bytes: usize,
    pub elapsed: Duration,
}
```

### 0.3 Update `HttpEngine::execute()` to use `Bytes` directly

**File:** `crates/reqforge-core/src/http/client.rs`

- [ ] Remove `.to_vec()` call on response bytes — use `Bytes` directly from `response.bytes().await?`
- [ ] Remove `body_text` local variable and the `String::from_utf8` allocation
- [ ] Remove `body_text` field from `HttpResponse` construction
- [ ] Assign `body: Bytes` directly in the returned `HttpResponse`

### 0.4 Refactor `Interpolator::replace()` to use `Cow<str>`

**File:** `crates/reqforge-core/src/env/interpolator.rs`

- [ ] Change `replace()` return type from `String` to `Cow<'a, str>`
- [ ] Add fast-path: if `!input.contains("{{")`, return `Cow::Borrowed(input)` (zero alloc)
- [ ] Wrap the regex slow-path in `Cow::Owned(...)`
- [ ] Update `resolve()` and `resolve_pairs()` to call `.into_owned()` or `.to_string()` where needed (these already return owned types)

### 0.5 Update all callers of `HttpResponse`

- [ ] Grep for `.body_text` field access across the entire codebase
- [ ] Update `crates/reqforge-core/src/models/history.rs` — `ResponseSnapshot::from()` to use `.body_text()` method
- [ ] Update `crates/reqforge-app/src/ui/response_viewer.rs` — use `.body_text()` method
- [ ] Update any test files that reference `.body_text` as a field to use `.body_text()` method call
- [ ] Update any `.body.to_vec()` / `.body.clone()` patterns to use `Bytes::clone()` (refcount bump)

### 0.6 Verify

- [ ] Run `cargo test -p reqforge-core` — all 120+ tests must pass
- [ ] Run `cargo check --workspace` — no compile errors

---

## Phase 1: gpui-component Dependency Setup

### 1.1 Update `reqforge-app/Cargo.toml`

**File:** `crates/reqforge-app/Cargo.toml`

- [ ] Remove `parking_lot` dependency
- [ ] Remove `dirs` dependency
- [ ] Remove `tokio` direct dependency
- [ ] Remove commented-out GPUI git dependency
- [ ] Add `gpui-component = "0.5"`
- [ ] Add `bytes = "1"`
- [ ] Keep `reqforge-core`, `serde`, `serde_json`, `uuid` deps

Target:
```toml
[dependencies]
reqforge-core = { path = "../reqforge-core" }
gpui-component = "0.5"
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
bytes = "1"
```

### 1.2 Verify compilation

- [ ] Run `cargo check -p reqforge-app` — must compile cleanly
- [ ] Resolve any dependency conflicts if they arise

### 1.3 Update workspace root if needed

- [ ] If gpui-component re-exports `gpui`, add it to `[workspace.dependencies]` for shared use

---

## Phase 2: App State — Entity-Based Architecture

Replace `Arc<RwLock<AppState>>` with GPUI's `Entity<T>` system.

### 2.1 Rewrite `app_state.rs` as GPUI Entity

**File:** `crates/reqforge-app/src/app_state.rs`

- [ ] Remove `Arc<RwLock<>>` wrappers
- [ ] Define `AppState` struct with: `core: ReqForgeCore`, `tabs: Vec<TabState>`, `active_tab: Option<usize>`, `active_env_id: Option<Uuid>`
- [ ] Define `TabState` struct with: `request_id`, `collection_id`, `url_input: Entity<InputState>`, `method: HttpMethod`, `headers: Vec<KeyValueRow>`, `params: Vec<KeyValueRow>`, `body_input: Entity<InputState>`, `last_response: Option<HttpResponse>`, `is_loading: bool`, `is_dirty: bool`
- [ ] Add helper methods: `active_tab(&self)`, `active_tab_mut(&mut self)`

### 2.2 Create `KeyValueRow` view model

**File:** `crates/reqforge-app/src/app_state.rs` (same file)

- [ ] Define `KeyValueRow` struct with: `key_input: Entity<InputState>`, `value_input: Entity<InputState>`, `enabled: bool`

---

## Phase 3: Main Window Bootstrap

### 3.1 Rewrite `main.rs`

**File:** `crates/reqforge-app/src/main.rs`

- [ ] Replace `tokio::main` with `App::new().run()`
- [ ] Call `gpui_component::init(cx)` to register actions/themes
- [ ] Create `ReqForgeCore` instance via `ReqForgeCore::open()`
- [ ] Create `Entity<AppState>` via `cx.new()`
- [ ] Open window with `cx.open_window()` containing `RootView`

### 3.2 Create `RootView`

**File:** `crates/reqforge-app/src/ui/root.rs` (new file)

- [ ] Define `RootView` struct holding `Entity<AppState>` and sidebar entity
- [ ] Implement `Render` trait with `h_flex()` layout: sidebar (left) + main area (right)
- [ ] `render_sidebar()` — renders the `SidebarPanel`
- [ ] `render_main_area()` — renders tab bar + request editor + response viewer based on active tab

### 3.3 Update `ui/mod.rs`

- [ ] Add `pub mod root;` and re-export `RootView`
- [ ] Add module declarations for all new UI components

---

## Phase 4: UI Components (using gpui-component)

### 4.1 Sidebar — Collection Tree

**File:** `crates/reqforge-app/src/ui/sidebar.rs` (rewrite)

- [ ] Define `SidebarPanel` struct holding `Entity<AppState>`
- [ ] Implement `Render` using gpui-component's `Tree` component
- [ ] Map `Collection` → root tree node
- [ ] Map `Folder` → collapsible tree node
- [ ] Map `RequestDefinition` → leaf node with method badge + name
- [ ] Handle click → open request in tab (or focus existing)
- [ ] Context menu: New Request, New Folder, Rename, Delete

### 4.2 Tab Bar

**File:** `crates/reqforge-app/src/ui/tab_bar.rs` (rewrite)

- [ ] Define `RequestTabBar` struct holding `Entity<AppState>`
- [ ] Implement `Render` using gpui-component's `Tab` / `TabList`
- [ ] One tab per open request showing method badge + name
- [ ] Dirty indicator (dot) when `is_dirty == true`
- [ ] Close button per tab
- [ ] Click → switch active tab

### 4.3 Request Editor

**File:** `crates/reqforge-app/src/ui/request_editor.rs` (rewrite)

- [ ] Define `RequestEditor` struct holding `Entity<AppState>`
- [ ] URL bar row: `Select` (HTTP method) + `Input` (URL) + `Button` ("Send")
- [ ] Sub-tab bar: `Tab` components for Params / Headers / Body
- [ ] Params sub-tab → `KeyValueEditor` for query params
- [ ] Headers sub-tab → `KeyValueEditor` for headers
- [ ] Body sub-tab → multiline `Input` + content-type `Select`
- [ ] Wire Send button to `on_send()` handler

### 4.4 Response Viewer

**File:** `crates/reqforge-app/src/ui/response_viewer.rs` (rewrite)

- [ ] Define `ResponseViewer` struct holding `Entity<AppState>`
- [ ] Status bar: color-coded status code badge + elapsed time + response size
- [ ] Sub-tabs: Body / Headers
- [ ] Body tab: pretty-printed JSON via `response.pretty_body()`, fallback to raw text via `response.body_text()`
- [ ] Headers tab: table of response headers
- [ ] Zero-copy: borrow `&str` from `Bytes` via `body_text()` — no allocation

### 4.5 Environment Selector

**File:** `crates/reqforge-app/src/ui/env_selector.rs` (rewrite)

- [ ] Define `EnvSelector` struct holding `Entity<AppState>`
- [ ] Implement using gpui-component's `Select` dropdown
- [ ] List all environments from `core.environments`
- [ ] "No Environment" option
- [ ] Selection updates `app_state.active_env_id`
- [ ] "Manage Environments" option → opens env editor modal

### 4.6 Environment Editor Modal

**File:** `crates/reqforge-app/src/ui/env_editor_modal.rs` (rewrite)

- [ ] Define `EnvEditorModal` struct
- [ ] Use gpui-component's `Dialog`
- [ ] Left panel: list of environments
- [ ] Right panel: variable table (key/value `Input` + `Checkbox` enabled + `Switch` secret)
- [ ] Add/Remove/Rename environment buttons
- [ ] Save/Cancel footer

### 4.7 Key-Value Editor (Reusable)

**File:** `crates/reqforge-app/src/ui/key_value_editor.rs` (rewrite)

- [ ] Define `KeyValueEditor` struct with `Vec<KeyValueRow>` + callbacks
- [ ] Each row: `Input` (key) + `Input` (value) + `Checkbox` (enabled) + delete `Button`
- [ ] "Add Row" button at bottom
- [ ] Zero-copy: `Entity<InputState>` manages text internally, no String cloning until save/send

### 4.8 Cleanup

- [ ] Delete `crates/reqforge-app/src/ui/body_editor.rs` (merged into request_editor)

---

## Phase 5: Bridge Layer — Core <-> UI Conversion

### 5.1 `build_request_from_tab()`

**File:** `crates/reqforge-app/src/bridge.rs` (new file)

- [ ] Implement `build_request_from_tab(tab: &TabState, cx: &App) -> RequestDefinition`
- [ ] Read `Entity<InputState>` text via `.read(cx).text().to_string()` — single allocation point
- [ ] Map `KeyValueRow` → `KeyValuePair` for headers and params
- [ ] Map body input → `BodyType`

### 5.2 `populate_tab_from_request()`

**File:** `crates/reqforge-app/src/bridge.rs` (same file)

- [ ] Implement `populate_tab_from_request(req: &RequestDefinition, window: &mut Window, cx: &mut App) -> TabState`
- [ ] Create `Entity<InputState>` for URL, body, each header/param key-value pair
- [ ] Set default values from the `RequestDefinition` fields

### 5.3 Wire up bridge

- [ ] Add `pub mod bridge;` to `main.rs` or `lib.rs`
- [ ] Call `populate_tab_from_request()` when opening a request from sidebar
- [ ] Call `build_request_from_tab()` in the Send button handler

---

## Phase 6: Wire Up Send Button (Async Execution)

- [ ] In `RequestEditor::on_send()`, call `build_request_from_tab()` to create `RequestDefinition`
- [ ] Set `tab.is_loading = true` + `cx.notify()`
- [ ] Use `cx.spawn()` for async execution (GPUI's runtime, not tokio)
- [ ] Call `core.execute_request(&req).await` inside the spawned future
- [ ] On completion: set `tab.last_response`, `tab.is_loading = false`, `cx.notify()`
- [ ] Handle errors gracefully (display in response viewer)

---

## Phase 7: Final Verification

- [ ] `cargo test -p reqforge-core` — all 120+ tests pass
- [ ] `cargo check -p reqforge-app` — compiles with gpui-component
- [ ] `cargo build --workspace` — full workspace builds
- [ ] `cargo run -p reqforge-app` — window opens with three-panel layout
- [ ] Manual: type URL → select method → click Send → see response
- [ ] Manual: switch environments, verify variable interpolation
- [ ] Manual: open/close tabs, verify state management

---

## File Changes Summary

### Files to modify:

- [ ] `Cargo.toml` — Add `bytes = "1"` to workspace deps
- [ ] `crates/reqforge-core/Cargo.toml` — Add `bytes.workspace = true`
- [ ] `crates/reqforge-core/src/models/response.rs` — `Vec<u8>` → `Bytes`, remove `body_text` field, add `body_text()` method
- [ ] `crates/reqforge-core/src/http/client.rs` — Remove `.to_vec()`, use `Bytes` directly
- [ ] `crates/reqforge-core/src/env/interpolator.rs` — Internal `replace()` returns `Cow<str>`
- [ ] `crates/reqforge-core/src/models/history.rs` — Update `ResponseSnapshot::from()` for new `HttpResponse`
- [ ] `crates/reqforge-core/src/lib.rs` — Update callers of `body_text`
- [ ] All tests referencing `body_text` field — Update to `.body_text()` method
- [ ] `crates/reqforge-app/Cargo.toml` — Replace deps with `gpui-component = "0.5"`
- [ ] `crates/reqforge-app/src/main.rs` — Rewrite with `App::new().run()`, gpui-component init
- [ ] `crates/reqforge-app/src/app_state.rs` — Rewrite as GPUI Entity-based state
- [ ] `crates/reqforge-app/src/ui/mod.rs` — Update module declarations

### Files to create:

- [ ] `crates/reqforge-app/src/ui/root.rs` — RootView with three-panel layout
- [ ] `crates/reqforge-app/src/ui/sidebar.rs` — Tree-based collection browser
- [ ] `crates/reqforge-app/src/ui/tab_bar.rs` — Tab component for open requests
- [ ] `crates/reqforge-app/src/ui/request_editor.rs` — Real GPUI request editor
- [ ] `crates/reqforge-app/src/ui/response_viewer.rs` — Real GPUI response viewer
- [ ] `crates/reqforge-app/src/ui/key_value_editor.rs` — Real GPUI key-value editor
- [ ] `crates/reqforge-app/src/ui/env_selector.rs` — Real GPUI Select dropdown
- [ ] `crates/reqforge-app/src/ui/env_editor_modal.rs` — Real GPUI Dialog
- [ ] `crates/reqforge-app/src/bridge.rs` — Core ↔ UI type conversion

### Files to delete:

- [ ] `crates/reqforge-app/src/ui/body_editor.rs` — Merged into request_editor

---

## Zero-Copy Architecture Summary

| Layer | Pattern | Why |
|---|---|---|
| HTTP response body | `bytes::Bytes` (refcounted) | Zero-cost clone, borrow as `&str` |
| Body display | `body_text() -> Option<&str>` | Borrows from Bytes, no allocation |
| Input text | `Entity<InputState>` (Rope-backed) | gpui-component manages text internally, no String cloning |
| Core → UI | Read `Entity<InputState>` via `.read(cx).text()` | Borrow, don't own |
| UI → Core | `build_request()` allocates `String` only at ownership boundary | Single allocation point |
| Interpolation | `Cow<str>` fast-path when no `{{vars}}` | Skip alloc when string unchanged |
| Shared state | `Entity<AppState>` (GPUI refcounted) | No `Arc<RwLock>` — GPUI handles concurrency |
