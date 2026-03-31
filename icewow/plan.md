# Plan: Add `iconflow` with `lucide_icon` and `raw_icon`

## Goal

Integrate `iconflow` into the UI without introducing an enum bottleneck for every new icon.

The current UI hardcodes glyphs like `"⋯"`, `"×"`, `"▾"`, and `"▸"` directly in view code. We want to replace that with reusable icon components that fit the existing `src/ui/` architecture, while keeping icon usage low-friction.

## Direction

Use Lucide only for now. Enable only `pack-lucide` via feature flag to avoid bundling unused icon packs.

Do not introduce a required app-wide `AppIcon` enum. That adds friction without much benefit if feature code regularly needs new icons by name.

Instead, use a two-layer structure:

```text
lucide_icon(name) -> raw_icon(glyph, family)
```

This keeps the public API simple:

- `lucide_icon(...)` is the ergonomic wrapper used throughout the app
- `raw_icon(...)` is the low-level renderer for a resolved glyph and font family

The component names should explicitly reflect Lucide, since that is the only supported pack in this iteration.

## Architecture

### `src/ui/icons.rs`

This module owns all `iconflow` integration.

Responsibilities:

- expose Lucide font bytes for startup loading
- resolve a Lucide icon name into glyph metadata using `iconflow`
- provide the low-level `raw_icon(...)` renderer
- provide the `lucide_icon(...)` wrapper used by the rest of the UI

This keeps `Pack`, `Style`, `Size`, and `try_icon(...)` out of screen modules.

### `src/ui/components.rs`

This remains the reusable widget layer.

Responsibilities:

- build a generic `icon_button(...)` that accepts already-built icon content
- keep styling and widget composition centralized

### Screen modules

Modules like `sidebar.rs`, `tabs.rs`, and `main_panel.rs` should consume `lucide_icon(...)` and pass that into `icon_button(...)`, not raw Unicode glyphs and not direct `iconflow` calls.

## Proposed API

### `src/ui/icons.rs`

```rust
use iced::widget::Text;

pub fn lucide_icon(name: &str, size: u16) -> Text<'static>;

pub fn raw_icon(glyph: char, family: &'static str, size: u16) -> Text<'static>;
```

Return `Text<'static>`, not `Element`. The icon name is consumed during lookup — the returned widget owns a `String` from `glyph.to_string()` and does not borrow from `name`. Returning `Text` rather than `Element` lets callers compose directly inside `button(...)` and `row![...]` without `.into()`.

Font loading uses `iconflow::fonts()` directly — with the `pack-lucide` feature flag, this already returns only Lucide font assets. No wrapper needed.

## Behavior

### `lucide_icon(...)`

- accepts a Lucide icon name like `"ellipsis"`, `"x"`, `"chevron-down"`, `"chevron-right"`
- resolves it with `Pack::Lucide`
- uses a single default `Style` and `Size` unless a real need emerges for variants
- passes the resolved glyph and family to `raw_icon(...)`
- returns `Text<'static>` — the name is consumed during lookup, the widget owns the glyph string

### `raw_icon(...)`

- accepts the final `glyph` and `family`
- renders the actual `iced::widget::text(glyph.to_string())`
- applies `iced::font::Font::with_name(family)`
- returns `Text<'static>`

This split is useful because `raw_icon(...)` stays generic, while `lucide_icon(...)` owns the library-specific lookup.

## Failure Model

Do not add a silent fallback for bad icon names.

If a Lucide icon name is wrong or missing, fail fast at the lookup boundary with an explicit `expect(...)` or similarly strict handling. The icon name is developer-authored input, not user input, so hiding mistakes behind fallback glyphs is not helpful.

That means:

- no ASCII fallback glyphs
- no automatic substitution logic
- clear failure when a referenced icon does not exist

## Implementation Steps

### 1. Add the dependency

Update [Cargo.toml](/Users/hmziq/os/learning/icewow/Cargo.toml):

```toml
iconflow = { version = "1", features = ["pack-lucide"] }
```

Using `pack-lucide` only avoids bundling fonts for Bootstrap, Heroicons, and other unused packs. No engine changes are needed.

### 2. Add the icon module

Create [src/ui/icons.rs](/Users/hmziq/os/learning/icewow/src/ui/icons.rs).

Implement:

- `raw_icon(...)`
- `lucide_icon(...)`

Suggested internal flow:

```rust
pub fn lucide_icon(name: &str, size: u16) -> Text<'static> {
    let icon = try_icon(Pack::Lucide, name, Style::Regular, Size::Regular)
        .expect("missing lucide icon");

    let glyph = char::from_u32(icon.codepoint)
        .expect("invalid lucide codepoint");

    raw_icon(glyph, icon.family, size)
}

pub fn raw_icon(glyph: char, family: &'static str, size: u16) -> Text<'static> {
    text(glyph.to_string())
        .size(size)
        .font(iced::font::Font::with_name(family))
}
```

### 3. Export through `ui`

Update [src/ui/mod.rs](/Users/hmziq/os/learning/icewow/src/ui/mod.rs):

- add `pub mod icons;`

### 4. Load Lucide fonts at app startup

Update [src/app.rs](/Users/hmziq/os/learning/icewow/src/app.rs).

Add a free function:

```rust
fn load_icon_fonts() -> Task<Message> {
    Task::batch(
        iconflow::fonts()
            .iter()
            .map(|f| font::load(f.bytes).map(Message::IconFontLoaded)),
    )
}
```

With `pack-lucide` enabled, `iconflow::fonts()` returns only Lucide font assets — no filtering needed.

Add a message variant:

```rust
IconFontLoaded(Result<(), font::Error>)
```

Handle it in `update()` — no app state change needed, just accept the result:

```rust
Message::IconFontLoaded(_) => Task::none(),
```

Update `PostmanUiApp::new()` to return the font-loading task instead of `Task::none()`:

```rust
pub fn new() -> (Self, Task<Message>) {
    (
        Self { state: AppState::new() },
        load_icon_fonts(),
    )
}
```

This keeps font bootstrap in the app layer and icon lookup in the UI layer.

### 5. Refactor reusable icon widgets

Update [src/ui/components.rs](/Users/hmziq/os/learning/icewow/src/ui/components.rs).

Replace the current string-glyph `icon_button` with one that accepts pre-built content:

```rust
pub fn icon_button<'a>(
    icon: impl Into<Element<'a, Message>>,
) -> widget::Button<'a, Message> {
    button(icon)
        .padding([2, 6])
        .style(|theme, status| styles::handle_button(theme, status))
}
```

Using `impl Into<Element>` instead of `Text<'a>` directly avoids fighting iced's `Text<'a, Theme, Renderer>` generic params — `Text` already implements `Into<Element>`.

Typical usage:

```rust
components::icon_button(icons::lucide_icon("ellipsis", 16))
components::icon_button(icons::lucide_icon("x", 16))
```

No other button helpers (`menu_button`, `danger_button`, `secondary_button`) need changes — they accept string labels for text content, which is correct for their use cases.

This keeps:

- `icon_button(...)` generic as a reusable UI primitive
- `lucide_icon(...)` as the convenience wrapper that hides family and pack details
- `raw_icon(...)` as the only place that renders by glyph and family

### 6. Replace hardcoded glyph usage

Update:

- [src/ui/sidebar.rs](/Users/hmziq/os/learning/icewow/src/ui/sidebar.rs)
- [src/ui/tabs.rs](/Users/hmziq/os/learning/icewow/src/ui/tabs.rs)
- [src/ui/main_panel.rs](/Users/hmziq/os/learning/icewow/src/ui/main_panel.rs) if needed

Complete glyph inventory and replacements:

| Glyph | File(s) | Lucide replacement |
|-------|---------|-------------------|
| `"⋯"` | `sidebar.rs` (×3) | `"ellipsis"` |
| `"×"` | `tabs.rs` (×1), `main_panel.rs` (×3) | `"x"` |
| `"▾"` | `sidebar.rs` (×1) | `"chevron-down"` |
| `"▸"` | `sidebar.rs` (×1) | `"chevron-right"` |
| `"•"` | `sidebar.rs` (×1) | `"circle"` |
| `"📦"` | `sidebar.rs` (×1) | `"package"` |

## Naming Recommendation

To reflect the current scope clearly:

- use `lucide_icon(...)`, not `icon(...)`
- use `icon_button(...)` as the generic component primitive
- use `load_icon_fonts()` — with `pack-lucide` feature flag scoping is handled at the dependency level, not the function name

`raw_icon(...)` can stay generic because it is below the pack-specific wrapper.

## Acceptance Criteria

The work is complete when:

- `iconflow` is declared in [Cargo.toml](/Users/hmziq/os/learning/icewow/Cargo.toml) with `features = ["pack-lucide"]`
- Lucide fonts load from [src/app.rs](/Users/hmziq/os/learning/icewow/src/app.rs) during startup via `load_icon_fonts()`
- [src/ui/icons.rs](/Users/hmziq/os/learning/icewow/src/ui/icons.rs) is the only place that directly uses `iconflow::{fonts, try_icon, Pack, Size, Style}`
- the UI can render a Lucide icon by string name without adding enum entries
- [src/ui/components.rs](/Users/hmziq/os/learning/icewow/src/ui/components.rs) exposes a reusable generic `icon_button(impl Into<Element>)`
- all 11 hardcoded Unicode glyph instances across sidebar, tabs, and main_panel are replaced
- bad icon names fail clearly instead of silently degrading

## Non-Goals

This plan does not include:

- multi-pack support
- a generic pack abstraction
- icon fallback behavior
- replacing every text button with an icon

## Recommended Order

1. add `iconflow = { version = "1", features = ["pack-lucide"] }` to [Cargo.toml](/Users/hmziq/os/learning/icewow/Cargo.toml)
2. create [src/ui/icons.rs](/Users/hmziq/os/learning/icewow/src/ui/icons.rs)
3. wire `load_icon_fonts()` and `Message::IconFontLoaded` into [src/app.rs](/Users/hmziq/os/learning/icewow/src/app.rs)
4. refactor [src/ui/components.rs](/Users/hmziq/os/learning/icewow/src/ui/components.rs) so `icon_button(...)` accepts `impl Into<Element>` instead of `&str`
5. replace all 11 raw glyph instances in [src/ui/sidebar.rs](/Users/hmziq/os/learning/icewow/src/ui/sidebar.rs), [src/ui/tabs.rs](/Users/hmziq/os/learning/icewow/src/ui/tabs.rs), and [src/ui/main_panel.rs](/Users/hmziq/os/learning/icewow/src/ui/main_panel.rs)
6. `cargo check` to verify compilation
7. run the app and confirm Lucide icons render correctly
