# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build --workspace                                    # build all
cargo test -p reqforge-core                                # core tests (120 unit + 6 doc)
cargo test -p reqforge-core -- test_basic_interpolation    # single test
cargo test --workspace                                     # all tests
cargo check -p reqforge-app                                # type-check app (fast)
cargo run -p reqforge-app                                  # run GUI (stub)
cargo run -p reqforge-cli                                  # run CLI
```

## Architecture

Cargo workspace, edition 2024. Three crates:

- `reqforge-core` (lib) — all domain logic, zero UI deps
- `reqforge-app` (bin) — GUI shell, migrating to `gpui-component`
- `reqforge-cli` (bin) — CLI via clap, thin wrapper over core

### Core (`reqforge-core`)

`ReqForgeCore` facade in `lib.rs` — the only API surface for UI crates. Owns `HttpEngine`, `JsonStore`, `RequestHistory` (behind `RwLock`), in-memory `Vec<Collection>` + `Vec<Environment>`.

Data flow: `RequestDefinition` → `Interpolator::resolve()` → `HttpEngine::execute()` → `HttpResponse`

Key modules: `models/` (all domain types), `http/client.rs` (reqwest engine + validation), `env/interpolator.rs` (`{{var}}` replacement), `store/json_store.rs` (file persistence), `history.rs`, `templates/`, `validation.rs`, `import_export.rs`.

### App (`reqforge-app`) — in transition

Current: `Arc<RwLock>` + `parking_lot` + `tokio::main` + println stubs. Being replaced per `docs/gpui-component-plan.md`:

- `gpui-component = "0.5"` (bundles GPUI + 60 components)
- `Entity<AppState>` replaces `Arc<RwLock>` — GPUI handles concurrency + re-render
- `App::new().run()` + `cx.spawn()` replaces `tokio::main`
- `Entity<InputState>` (Rope-backed) manages text — no `String` cloning until save/send
- `bridge.rs` = single ownership boundary: `build_request_from_tab()` / `populate_tab_from_request()`

## Design Rules

**Read `docs/zerocopy.md` before writing any code.** It is the primary architecture guide. Key references:

- `zerocopy.md:7-18` — no-copy boundary rule. Red-flag calls: `to_vec()`, `to_string()`, `clone()` on buffers
- `zerocopy.md:22-38` — own buffer once (`Bytes`), then only slice. Never mutate after handing out borrows
- `zerocopy.md:65-80` — `Cow<str>` for borrow fast-path, own on slow-path
- `zerocopy.md:109-122` — lifetime discipline. Never return borrows to temporaries
- `zerocopy.md:126-139` — no self-referential structs. Prefer "owner outside + views returned"
- `zerocopy.md:145-149` — postpone UTF-8 decode. Keep bytes as bytes until display

**Project-specific zero-copy rules:**
- `HttpResponse.body` is `Bytes`, not `Vec<u8>`. `body_text()` is a **method** (`-> Option<&str>`) — not a field. Do not add `body_text: Option<String>`.
- `Interpolator::replace()` returns `Cow<str>` — borrows when no `{{vars}}`, allocates only when substituting.
- Allocate `String` only in bridge layer (`build_request_from_tab`) at the ownership boundary.

**Other rules:**
- Core stays headless — zero UI deps in `reqforge-core`
- Propagate errors with `thiserror` + `Result`. No `unwrap`/`expect` outside tests.
- Single shared `reqwest::Client` in `HttpEngine`. Use `Response::bytes()` + `from_utf8`, not `Response::text()`.

## Current State

Core: complete, 120 tests. App: stub UI. Active plan: `docs/gpui-component-plan.md` — follow its checklist and phase order. Phase 0 (zero-copy core refactor) comes before any UI work.
