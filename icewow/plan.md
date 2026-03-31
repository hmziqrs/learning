# Plan: Component Wrappers & Request Builder

## Context

Backend extraction into `icewow-engine` is complete. The engine currently supports only `Client::send(url, method)` — no body, headers, or content types. The UI has 17 centralized style functions in `ui/styles.rs` but no component abstractions — every call site manually wires widget + style closure, causing duplication (e.g., `handle_button` style applied 5 times across files).

Two tracks, in order: **component wrappers first** (cleans up the codebase and creates building blocks), then **request builder** (adds real functionality using those components).

---

## Track 1: Component Wrappers — DONE

### Problem

Repeated patterns across UI files:
- `button("⋯").padding([2,6]).on_press(...).style(|t,s| styles::handle_button(t,s))` — 5 occurrences
- `button(text).padding([...]).on_press(...).style(|t,s| styles::menu_button(t,s))` — 5 occurrences
- `button(text).padding([...]).on_press(...).style(|t,s| styles::danger_button(t,s))` — 3 occurrences
- `button(text).padding([...]).on_press(...).style(|t,s| styles::secondary_button(t,s))` — 2 occurrences
- `container(content).padding([...]).style(|t| styles::method_badge(t, method))` — 2 occurrences
- `container(text("")).height(Length::Fixed(...)).width(Length::Fill).style(...)` — drop_line pattern repeated

### Implementation

**`src/ui/components.rs`** (new file) — 6 component functions:
| Component | Line | Purpose |
|---|---|---|
| `icon_button(label)` | :8 | Small icon button (⋯, ×, ▾, ▸) using `handle_button` style |
| `menu_button(label)` | :15 | Context menu item using `menu_button` style |
| `danger_button(label)` | :21 | Destructive action using `danger_button` style |
| `secondary_button(label)` | :27 | Secondary action using `secondary_button` style |
| `method_badge(method)` | :33 | HTTP method pill (GET, POST, etc.) using `method_badge` style |
| `status_badge(status_code)` | :40 | Response status pill using `status_badge` style |

**`src/ui/styles.rs`** — new style function:
| Style | Line | Purpose |
|---|---|---|
| `body_type_button(theme, status, active)` | :372 | Body type selector button (active/hover/default states) |

**Files refactored** (all raw button/style patterns replaced with component calls):
- `src/ui/sidebar.rs` — `icon_button` (×4), `menu_button` (×5), `danger_button` (×2)
- `src/ui/tabs.rs` — `icon_button` (×1), `secondary_button` (×1)
- `src/ui/mod.rs` — `secondary_button` (×1), `danger_button` (×1)
- `src/ui/main_panel.rs` — `method_badge` (×1), `status_badge` (×1), `body_type_button` via styles (×1)

No raw `styles::handle_button`/`menu_button`/`danger_button`/`secondary_button` closures remain outside `components.rs`.

---

## Track 2: Request Builder & Response Improvements — DONE

### 2A: Engine — Request Builder — DONE

#### `engine/src/http/request.rs` (new file)
| Item | Line | Description |
|---|---|---|
| `struct Request` | :3 | url, method, headers, body fields |
| `enum RequestBody` | :10 | `Raw(String)`, `Json(serde_json::Value)`, `Form(Vec<(String, String)>)` |
| `Request::new()` | :18 | Constructor with url + method |
| `Request::header()` | :27 | Builder: add header pair |
| `Request::body()` | :32 | Builder: set body from RequestBody |
| `Request::json()` | :37 | Builder: set JSON body |
| `Request::raw_body()` | :42 | Builder: set raw text body |
| `Request::form()` | :47 | Builder: set form-encoded body |

#### `engine/src/http/client.rs`
| Method | Line | Description |
|---|---|---|
| `Client::new()` | :11 | Wraps `reqwest::Client` |
| `Client::send()` | :17 | Simple url+method, delegates to `execute()` |
| `Client::execute()` | :22 | Full request with headers, body; extracts response headers |

#### `engine/src/http/response.rs`
| Field | Line | Description |
|---|---|---|
| `headers: Vec<(String, String)>` | :6 | Response headers extracted from reqwest |

#### `engine/Cargo.toml`
- `serde_json = "1"` added

#### Re-exports
- `engine/src/http/mod.rs` (:8) — `pub use request::{RequestBody, Request}`
- `engine/src/lib.rs` (:5) — `pub use http::{Client, HttpMethod, Request, RequestBody, Response}`

### 2B: UI — Request Editor — DONE

#### `src/model.rs`
| Item | Line | Description |
|---|---|---|
| `Tab.body_type` | :38 | `BodyType` — which body editor to show |
| `Tab.body_text` | :39 | `String` — raw/JSON body content |
| `Tab.form_pairs` | :40 | `Vec<(String, String)>` — form key-value pairs |
| `Tab.headers` | :41 | `Vec<(String, String)>` — request headers |
| `enum BodyType` | :45 | `None`, `Raw`, `Json`, `Form` |

All 3 `Tab` constructors updated with new fields: model.rs (:263-266, :274-277), app.rs (:172-176, :515-519).

#### `src/app.rs`
Message variants (lines :57-66):
- `SetBodyType(BodyType)`, `UpdateBodyText(String)`, `AddFormPair`, `UpdateFormKey(usize, String)`, `UpdateFormValue(usize, String)`, `RemoveFormPair(usize)`
- `AddHeader`, `UpdateHeaderKey(usize, String)`, `UpdateHeaderValue(usize, String)`, `RemoveHeader(usize)`

All 10 message handlers implemented (lines :391-449).

`send_engine_request()` (line :795) builds `icewow_engine::Request` from tab fields and calls `Client::execute()`. Auto-sets Content-Type for JSON and Form body types.

#### `src/ui/main_panel.rs`
| Function | Line | Description |
|---|---|---|
| `view_main_panel()` | :9 | Shows headers editor + body editor when tab is active |
| `view_headers_editor()` | :88 | Key-value pair list with add/remove |
| `view_body_editor()` | :125 | BodyType selector (None/Raw/JSON/Form) + appropriate editor |

### 2C: UI — Response Display Improvements — DONE

- Response headers displayed below status badge (main_panel.rs :56-69)
- `components::method_badge()` used for request method badge (main_panel.rs :22)
- `components::status_badge()` used for response status code (main_panel.rs :48)
- `styles::body_type_button()` extracted to styles.rs (:372), replacing inline closure
