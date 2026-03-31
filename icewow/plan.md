# Plan: Extract Backend into `icewow-engine` Sub-Crate

## Context

IceWow is a Postman-like API client with iced 0.14 UI. Currently a single crate mixing UI and backend (reqwest HTTP). The goal is to extract all backend logic into a separate `icewow-engine` crate that will house HTTP, GraphQL, Protocol Buffers, WebSocket, and local persistence — keeping the main crate as a pure iced UI layer.

## Target Structure

```
icewow/
  Cargo.toml              # workspace root
  Cargo.lock
  src/                     # UI crate stays at root (cargo run still works)
    main.rs, app.rs, model.rs, tree_ops.rs, ui/
  engine/                  # backend sub-crate
    Cargo.toml
    src/
      lib.rs               # public API re-exports
      error.rs             # Error type
      http/
        mod.rs             # re-exports
        method.rs          # HttpMethod enum (moved from model.rs)
        response.rs        # Response struct (moved from model.rs as ResponseData)
        client.rs          # Client struct wrapping reqwest (new)
```

The `engine/` crate has **ZERO UI framework dependencies** — no iced, no DRUID, nothing. It's a pure async Rust library with its own domain types for errors, requests, and responses. Any UI framework could consume it.

## Framework-Agnostic Design Principles

- **Own error types** — `engine::Error` is a proper enum (not `String`), with variants like `Http(reqwest::Error)`, `Timeout`, `InvalidUrl`. The UI crate maps these to `String` at the call site.
- **Own response type** — `engine::Response` is a plain data struct. No UI-specific derives or traits.
- **Clean public API** — `engine::Client` is the sole entry point. `Client::send(url, method)` mirrors the current `send_request` signature.

### Deferred (future enhancements, not part of this extraction)
- `Request` builder (body, headers, content types)
- GraphQL, Protocol Buffers, WebSocket support
- Local persistence

## What Moves to `engine/`

From `src/model.rs`:
- `HttpMethod` enum, including `ALL` const, `as_str()`, and `Display` impl (lines 7-40) → `engine/src/http/method.rs`
- `ResponseData` struct (lines 42-47) → `engine/src/http/response.rs`, renamed to `Response`

Note: the remaining types in `model.rs` (`AppState`, `DragState`, `PendingLongPress`, etc.) stay in the UI crate — they depend on `iced::Point` and `iced::Size`.

From `src/app.rs`:
- `send_request()` async fn (lines 716-738) → `engine/src/http/client.rs` as `Client::send()`

From `Cargo.toml`:
- `reqwest` dependency → `engine/Cargo.toml`

## New Types in `engine/`

- `Error` — proper enum: `Http(reqwest::Error)`, `Timeout`, `InvalidUrl(String)`. UI-friendly `Display` impl.
- `Client` — wraps `reqwest::Client`, provides `send()` method that takes url + method (matching current `send_request` signature)

Note: a `Request` builder (body, headers, content types) is a future enhancement — not part of this extraction. Keep the initial API surface minimal: `Client::send(url, method) -> Result<Response, Error>`.

## Changes to UI Crate (`src/`)

1. **`Cargo.toml`** — add `icewow-engine = { path = "engine" }`, remove `reqwest`
2. **`model.rs`** — remove `HttpMethod` and `ResponseData`, re-export from `icewow_engine` (e.g. `pub use icewow_engine::{HttpMethod, Response as ResponseData}`) so downstream `crate::model::HttpMethod` paths keep working
3. **`app.rs`** — replace `send_request()` with `icewow_engine::Client::new().send()`, map `engine::Error` to `String` at the call site (preserving current `Result<ResponseData, String>` in the UI layer)
4. **`ui/styles.rs`** — `HttpMethod` import unchanged (still `crate::model::HttpMethod` via re-export)
5. **`ui/main_panel.rs`** — fully-qualified path `crate::model::HttpMethod::Get` (line 11) unchanged via re-export
6. **`tree_ops.rs`** — test imports unchanged via re-export

## Steps

1. Update root `Cargo.toml` to workspace (`members = [".", "engine"]`) and create `engine/Cargo.toml` (reqwest + tokio, no iced) — do both together so `cargo check` can resolve the subcrate
2. Write `engine/src/error.rs` — `Error` enum
3. Write `engine/src/http/method.rs` — move `HttpMethod` (including `ALL`, `as_str()`, `Display`)
4. Write `engine/src/http/response.rs` — move `ResponseData` as `Response`
5. Write `engine/src/http/client.rs` — `Client` with `send(url, method)` (extracted from `app.rs:send_request`)
6. Write `engine/src/http/mod.rs` — re-exports
7. Write `engine/src/lib.rs` — public API re-exports
8. `cargo check -p icewow-engine` — verify engine crate compiles in isolation
9. Update UI crate `Cargo.toml` — add `icewow-engine` dep, remove `reqwest`
10. Update `src/model.rs` — remove `HttpMethod` and `ResponseData`, add `pub use icewow_engine::{HttpMethod, Response as ResponseData}`
11. Update `src/app.rs` — use `icewow_engine::Client`, remove `send_request()`, map `engine::Error` → `String` at call site
12. `cargo check` — verify both crates compile together
13. `cargo test` — tree_ops tests still pass
14. `cargo run` — manual smoke test: send a request, verify response displays

## Verification

- `cargo check` — both crates compile
- `cargo test` — tree_ops unit tests pass
- `cargo run` — UI launches, can send HTTP requests, response displays correctly
