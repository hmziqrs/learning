# Agent Notes

- Treat `docs/zerocopy.md` as the primary architecture guide; read it first.
- Prefer “zero-copy first” designs (borrowed data, slices/`Cow`, avoid unnecessary `clone`/allocations).
- When choosing Rust/std/crate APIs, verify current best practices via Context7 and keep changes compatible with this workspace’s Rust edition (2024).
