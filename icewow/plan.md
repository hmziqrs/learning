# Plan: Component Wrappers & Request Builder

## Context

Backend extraction into `icewow-engine` is complete. The engine currently supports only `Client::send(url, method)` — no body, headers, or content types. The UI has 17 centralized style functions in `ui/styles.rs` but no component abstractions — every call site manually wires widget + style closure, causing duplication (e.g., `handle_button` style applied 5 times across files).

Two tracks, in order: **component wrappers first** (cleans up the codebase and creates building blocks), then **request builder** (adds real functionality using those components).

---

## Track 1: Component Wrappers

### Problem

Repeated patterns across UI files:
- `button("⋯").padding([2,6]).on_press(...).style(|t,s| styles::handle_button(t,s))` — 5 occurrences
- `button(text).padding([...]).on_press(...).style(|t,s| styles::menu_button(t,s))` — 5 occurrences
- `button(text).padding([...]).on_press(...).style(|t,s| styles::danger_button(t,s))` — 3 occurrences
- `button(text).padding([...]).on_press(...).style(|t,s| styles::secondary_button(t,s))` — 2 occurrences
- `container(content).padding([...]).style(|t| styles::method_badge(t, method))` — 2 occurrences
- `container(text("")).height(Length::Fixed(...)).width(Length::Fill).style(...)` — drop_line pattern repeated

### Target Structure

Add a `ui/components.rs` module with small builder functions:

```
ui/
  mod.rs
  components.rs    ← NEW: reusable widget builders
  styles.rs        ← unchanged: raw style functions
  sidebar.rs       ← uses components
  tabs.rs          ← uses components
  main_panel.rs    ← uses components
```

### Components to Create

1. **Buttons** — each returns `Button<'a, Message>` pre-styled, caller adds `.on_press()`:
   - `icon_button(label: &str) -> Button<Message>` — uses `handle_button` style, small padding
   - `menu_button(label: &str) -> Button<Message>` — uses `menu_button` style
   - `action_button(label: &str, style: ActionStyle) -> Button<Message>` — `ActionStyle` enum: `Secondary`, `Danger`, `Primary` (new, for send button)

These are thin wrappers — not a full component framework. Just enough to deduplicate the current codebase. Badge and panel wrappers are deferred to Track 2 where they'll have real motivation (multiple call sites).

### Files Changed

- `ui/components.rs` — new file with component functions
- `ui/mod.rs` — add `pub mod components;`
- `ui/sidebar.rs` — replace raw button/style patterns with component calls
- `ui/tabs.rs` — replace raw button/style patterns with component calls
- `ui/main_panel.rs` — replace raw container/style patterns with component calls
- `ui/mod.rs` (delete_modal, drag_preview_overlay) — replace patterns with component calls

### Verification

- `cargo check` — compiles
- `cargo test` — tests pass
- `cargo run` — visual regression: UI looks identical to before

---

## Track 2: Request Builder & Response Improvements

### Problem

Engine `Client::send()` only takes `url + method`. No way to set:
- Request body (JSON, form data, raw text)
- Headers (Content-Type, Authorization, custom)
- Content type selection

Response only captures `status_code`, `body`, `elapsed_ms` — no response headers.

### 2A: Engine — Request Builder

#### Target API

```rust
// engine/src/http/request.rs
pub struct Request {
    url: String,
    method: HttpMethod,
    headers: Vec<(String, String)>,
    body: Option<RequestBody>,
}

pub enum RequestBody {
    Raw(String),
    Json(serde_json::Value),
    Form(Vec<(String, String)>),
}

impl Request {
    pub fn new(url: String, method: HttpMethod) -> Self { ... }
    pub fn header(mut self, key: String, value: String) -> Self { ... }
    pub fn body(mut self, body: RequestBody) -> Self { ... }
    pub fn json(mut self, value: serde_json::Value) -> Self { ... }
    pub fn raw_body(mut self, text: String) -> Self { ... }
    pub fn form(mut self, pairs: Vec<(String, String)>) -> Self { ... }
}
```

Client gets a new method alongside existing `send()`:
```rust
impl Client {
    // Keep existing simple method
    pub async fn send(&self, url: String, method: HttpMethod) -> Result<Response, Error> { ... }

    // New: full request builder
    pub async fn execute(&self, request: Request) -> Result<Response, Error> { ... }
}
```

#### Engine Changes

- `engine/Cargo.toml` — add `serde_json` dependency (for `RequestBody::Json`)
- `engine/src/http/request.rs` — new file: `Request` struct, `RequestBody` enum, builder methods
- `engine/src/http/client.rs` — add `execute(&self, request: Request)` method
- `engine/src/http/response.rs` — add `headers: Vec<(String, String)>` to `Response`
- `engine/src/http/mod.rs` — re-export `Request`, `RequestBody`
- `engine/src/lib.rs` — re-export `Request`, `RequestBody`

### 2B: UI — Request Editor

#### Tab Struct Changes

```rust
pub struct Tab {
    pub id: TabId,
    pub request_id: Option<RequestId>,
    pub title: String,
    pub url_input: String,
    pub method: HttpMethod,
    // New fields:
    pub body_type: BodyType,          // which body editor to show
    pub body_text: String,            // raw/JSON body content
    pub form_pairs: Vec<(String, String)>,  // form key-value pairs
    pub headers: Vec<(String, String)>,     // request headers
}

pub enum BodyType {
    None,
    Raw,
    Json,
    Form,
}
```

#### UI Design

Below the URL bar in `main_panel.rs`, add two sections:

**Headers editor** — key-value pair list:
- Each row: `[key input] [value input] [× remove button]`
- `[+ Add Header]` button at bottom
- Empty by default

**Body editor** — depends on `BodyType` selector:
- Row of buttons: `None | Raw | JSON | Form` (only one active)
- `None` → no editor shown (default, used for GET)
- `Raw` → single `text_editor` area
- `Json` → same `text_editor` area (Content-Type set automatically to `application/json`)
- `Form` → key-value pair list like headers (Content-Type set to `application/x-www-form-urlencoded`)

#### Message Variants

```rust
// Body
Message::SetBodyType(BodyType)
Message::UpdateBodyText(String)
Message::AddFormPair
Message::UpdateFormKey(usize, String)
Message::UpdateFormValue(usize, String)
Message::RemoveFormPair(usize)
// Headers
Message::AddHeader
Message::UpdateHeaderKey(usize, String)
Message::UpdateHeaderValue(usize, String)
Message::RemoveHeader(usize)
```

#### SendRequest Update

`SendRequest` handler builds a `Request` from the active tab's fields and calls `Client::execute()` instead of `Client::send()`. Content-Type header is auto-added based on `BodyType` if not already set by the user.

### 2C: UI — Response Display Improvements

Current response display is plain text in a scrollable container. Improvements:

- Show response headers in a collapsible section below the status badge
- Add `method_badge` component for the response status code (reuse `status_badge` style — color by status range: 2xx green, 3xx blue, 4xx orange, 5xx red)
- Badge wrappers (`method_badge`, `status_badge`) are created here in `ui/components.rs` since now there are real use cases

### Files Changed

- `engine/Cargo.toml` — add `serde_json`
- `engine/src/http/request.rs` — new file
- `engine/src/http/client.rs` — add `execute()`
- `engine/src/http/response.rs` — add `headers` field
- `engine/src/http/mod.rs` — re-exports
- `engine/src/lib.rs` — re-exports
- `src/model.rs` — `Tab` struct new fields, `BodyType` enum
- `src/app.rs` — new `Message` variants, updated `SendRequest` handler
- `src/ui/main_panel.rs` — headers editor, body editor, improved response view
- `src/ui/components.rs` — add `method_badge`, `status_badge` wrappers

### Verification

- `cargo check -p icewow-engine` — engine compiles with new types
- `cargo check` — both crates compile
- `cargo test` — existing tests pass
- `cargo run` — can send GET (unchanged), can type body text and send POST with body, response shows headers
