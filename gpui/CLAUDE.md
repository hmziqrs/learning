# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build --workspace          # build everything
cargo test -p reqforge-core      # run core tests (120 unit + 6 doc)
cargo test --workspace           # run all workspace tests
cargo test -p reqforge-core -- test_basic_interpolation  # run a single test by name
cargo check -p reqforge-app      # type-check the app crate (faster than build)
cargo run -p reqforge-app        # run the GUI app (currently stub UI)
cargo run -p reqforge-cli        # run the CLI tool
```

## Architecture

ReqForge is a Postman-like HTTP client. Three crates in a Cargo workspace (edition 2024):

```
reqforge-core (lib)    — All domain logic, zero UI deps. The single source of truth.
reqforge-app  (bin)    — GUI shell. Currently stub (println-based); being migrated to gpui-component.
reqforge-cli  (bin)    — CLI tool using clap. Thin wrapper over reqforge-core.
```

### reqforge-core internals

`ReqForgeCore` is the top-level facade (defined in `lib.rs`). UI crates only talk to this struct. It owns:
- `HttpEngine` — async HTTP execution via `reqwest` (shared `Client` with connect/request timeouts)
- `JsonStore` — file-based persistence (`collections/{id}.json`, `environments.json`, `history.json`)
- `RequestHistory` — behind `RwLock`, auto-records every sent request
- In-memory `Vec<Collection>` and `Vec<Environment>`

Key data flow: `RequestDefinition` → `Interpolator::resolve()` (replaces `{{var}}` placeholders) → `HttpEngine::execute()` → `HttpResponse`

Module map inside `reqforge-core/src/`:
- `models/` — `RequestDefinition`, `HttpResponse`, `Environment`, `Collection`, `Folder`, `RequestHistoryEntry`, `RequestTemplate`
- `http/client.rs` — `HttpEngine` + `HttpError`. Validates requests before sending.
- `env/interpolator.rs` — `{{variable}}` replacement in all string fields of a request
- `store/json_store.rs` — CRUD for collections and environments on disk
- `history.rs` — `RequestHistory` manager with add/replay/clear, max 100 entries
- `templates/` — 8 built-in request templates + custom template CRUD
- `validation.rs` — URL, header, body validation. Integrated into `HttpEngine::execute()`.
- `import_export.rs` — Native JSON, Postman v2.1, OpenAPI 3.x import; ZIP workspace export

### reqforge-app internals (in transition)

Currently uses `Arc<RwLock<ReqForgeCore>>` + `parking_lot` + `tokio::main` with println-based stub components. Being migrated to `gpui-component` crate (see `docs/gpui-component-plan.md`) which will replace this with GPUI's `Entity<T>` system and real UI components.

## Design Principles

**Read `docs/zerocopy.md` first** — it's the primary architecture guide for this project.

- **Zero-copy first:** borrow by default, copy only at ownership boundaries. Use `bytes::Bytes` for response bodies, `Cow<str>` for interpolation fast-paths, `&str` views over buffers instead of `String` allocations.
- **Core stays headless:** all domain logic lives in `reqforge-core` with zero UI deps. The app crate is a thin adapter layer.
- **Propagate errors, don't panic:** use `thiserror` typed errors and `Result`. No `unwrap`/`expect` outside tests.
- **Reuse the `reqwest::Client`:** `HttpEngine` holds a single shared client. Use `Response::bytes()` + `std::str::from_utf8` (not `Response::text()` which always allocates).
- **`{{variable}}` syntax:** the `Interpolator` resolves placeholders in URLs, headers, query params, and body content before execution.

## Current State & Next Steps

The core library is complete (120 tests). The app crate has stub UI components that demonstrate the architecture but don't render real windows. The active plan (`docs/gpui-component-plan.md`) migrates to `gpui-component = "0.5"` from crates.io and refactors `HttpResponse.body` from `Vec<u8>` to `bytes::Bytes`.
