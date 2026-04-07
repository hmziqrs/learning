# Agent Notes

- Treat `docs/zerocopy.md` as the primary architecture guide; read it first.
- For a high-level system overview and crate boundaries, see `docs/plan.md`.
- Prefer “zero-copy first” designs (borrowed data, slices/`Cow`, avoid unnecessary `clone`/allocations).
- When choosing Rust/std/crate APIs, verify current best practices via Context7 and keep changes compatible with this workspace’s Rust edition (2024).
- `reqwest`: reuse a shared `Client`; set `Client::builder().connect_timeout(..).timeout(..)`; prefer `Response::bytes()` + `std::str::from_utf8` fast-path; use `Error::is_timeout()` for timeout handling.
- `tokio`: prefer structured concurrency patterns (`tokio::task`, `tokio::select!`, `tokio::time::timeout`) for cancellation/timeouts and “many tasks” orchestration.
- `uuid`: consider enabling feature `v7` and using `Uuid::now_v7()` for time-ordered IDs (vs `Uuid::new_v4()`).
- Keep `reqforge-core` headless/testable (no UI deps); put domain logic there and keep UI as an adapter layer.
- Avoid `unwrap`/`expect` outside tests; propagate errors with `thiserror` and typed `Result`s.
- Before landing changes, run `cargo test` and update docs when behavior/architecture changes.
