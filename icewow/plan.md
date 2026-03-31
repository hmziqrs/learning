# Plan: Custom Dark Theme (shadcn-inspired)

## Context

The app currently uses `Theme::CatppuccinMocha` — a dark theme with purple/magenta accents as the primary color. All style functions in `ui/styles.rs` read from `theme.extended_palette()`, so the purple bleeds into: send button, active tabs, drag highlights, drop lines, body type selectors, and method badges (PUT uses primary).

Iced supports custom themes via `Theme::custom(name, Palette)` where `Palette` has 6 colors: background, text, primary, success, warning, danger. The framework auto-generates the full extended palette (strong/weak/base variants) from those 6.

## Design: shadcn-inspired Dark Theme

shadcn/ui uses a neutral zinc/slate dark palette with a white primary accent. Key characteristics:
- Dark backgrounds using zinc/neutral tones (not blue or purple)
- White/light text
- White or near-white primary (for buttons, active states, highlights)
- Subtle borders using slightly lighter zinc
- Red for danger, green for success, amber for warning — standard and muted

### Color Palette

```
Background:  #09090b  (zinc-950 — near-black)
Text:        #fafafa  (zinc-50 — near-white)
Primary:     #fafafa  (zinc-50 — white accent, like shadcn)
Success:     #22c55e  (green-500 — standard green)
Warning:     #eab308  (yellow-500 — standard amber)
Danger:      #ef4444  (red-500 — standard red)
```

This means:
- **Send button, active tabs, drag highlights** → white/near-white backgrounds with dark text
- **Background panels** → dark zinc tones (iced generates weak/base/strong variants from #09090b)
- **Success badges (GET, 2xx)** → green
- **Warning badges (POST, 4xx)** → amber/yellow
- **Danger badges (DELETE, 5xx)** → red
- **PUT badges** → white (was purple via primary — now neutral)
- **Patch badges** → iced's auto-generated secondary from the palette

## Changes

### `src/ui/theme.rs` (NEW FILE)

Create a dedicated theme module with the palette definition:

```rust
use iced::Theme;

const ICEWOW_DARK: iced::Palette = iced::Palette {
    background: iced::Color::from_rgb8(0x09, 0x09, 0x0b),  // zinc-950
    text:       iced::Color::from_rgb8(0xfa, 0xfa, 0xfa),  // zinc-50
    primary:    iced::Color::from_rgb8(0xfa, 0xfa, 0xfa),  // zinc-50 (white accent)
    success:    iced::Color::from_rgb8(0x22, 0xc5, 0x5e),  // green-500
    warning:    iced::Color::from_rgb8(0xea, 0xb3, 0x08),  // yellow-500
    danger:     iced::Color::from_rgb8(0xef, 0x44, 0x44),  // red-500
};

pub fn theme() -> Theme {
    Theme::custom("IceWow Dark", ICEWOW_DARK)
}
```

Future theme variants (light mode, etc.) would live here as additional palette constants and functions.

### `src/app.rs`

Change `theme()` method (line 508):
```rust
// Before:
pub fn theme(&self) -> Theme { Theme::CatppuccinMocha }
// After:
pub fn theme(&self) -> Theme { crate::ui::theme::theme() }
```

### `src/ui/mod.rs`

Add `pub mod theme;` to module declarations.

### `src/ui/styles.rs` — NO CHANGES

All 18 style functions already use `theme.extended_palette()`. The custom theme's auto-generated extended palette will flow through automatically. This is the whole point of the palette-driven architecture.

### `src/ui/components.rs`

Update the doc comment on `secondary_button` from "purple themed" to "secondary themed" since it will no longer be purple.

## Verification

- `cargo check` — compiles
- `cargo test` — tests pass
- `cargo run` — visual check: dark zinc backgrounds, white primary accents, green/amber/red badges, no purple anywhere
