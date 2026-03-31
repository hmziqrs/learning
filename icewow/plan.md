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

### Vibrant Method Badge Colors

Since `primary` is now white and `secondary` is auto-generated neutral gray, PUT and PATCH badges would lose their distinct identity if left on palette colors. Instead, use explicit vibrant colors for every HTTP method directly in `method_badge`:

```
GET     → #22c55e  (green-500)   — from palette success, no change
POST    → #eab308  (yellow-500)  — from palette warning, no change
PUT     → #38bdf8  (sky-400)     — vibrant blue, custom color
DELETE  → #ef4444  (red-500)     — from palette danger, no change
PATCH   → #a78bfa  (violet-400)  — vibrant purple, custom color
```

Each method gets a visually distinct, recognizable color — matching the convention users expect from API tools like Postman and Insomnia.

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

### `src/ui/styles.rs` — method_badge UPDATE

Replace the palette-derived colors for PUT and PATCH with explicit vibrant hex colors. GET/POST/DELETE continue to use the palette (green/amber/red match).

```rust
pub fn method_badge(theme: &Theme, method: HttpMethod) -> container::Style {
    let palette = theme.extended_palette();

    let (text, bg, border_color) = match method {
        HttpMethod::Get => (
            palette.success.strong.color,
            palette.success.weak.color,
            palette.success.base.color,
        ),
        HttpMethod::Post => (
            palette.warning.strong.color,
            palette.warning.weak.color,
            palette.warning.base.color,
        ),
        HttpMethod::Put => (
            Color::from_rgb8(0x38, 0xbd, 0xf8),  // sky-400
            Color::from_rgb8(0x0c, 0x2a, 0x3d),  // sky-950-ish (dark bg)
            Color::from_rgb8(0x0e, 0xa5, 0xe9),  // sky-500 (border)
        ),
        HttpMethod::Delete => (
            palette.danger.strong.color,
            palette.danger.weak.color,
            palette.danger.base.color,
        ),
        HttpMethod::Patch => (
            Color::from_rgb8(0xa7, 0x8b, 0xfa),  // violet-400
            Color::from_rgb8(0x2e, 0x1d, 0x4f),  // violet-950-ish (dark bg)
            Color::from_rgb8(0x8b, 0x5c, 0xf6),  // violet-500 (border)
        ),
    };

    container::Style {
        text_color: Some(text),
        background: Some(Background::Color(bg)),
        border: border::rounded(8).color(border_color).width(1),
        ..container::Style::default()
    }
}
```

The dark background tints (`0x0c2a3d`, `0x2e1d4f`) follow the same pattern as the palette-generated weak colors — a dark desaturated shade of the accent.

### `src/ui/components.rs`

Update the doc comment on `secondary_button` from "purple themed" to "secondary themed" since it will no longer be purple.

### `CLAUDE.md`

Update the styling description:
```
// Before:
The app uses the Catppuccin Mocha dark theme via `iced::Theme::CatppuccinMocha`.
// After:
The app uses a custom shadcn-inspired dark theme defined in `ui/theme.rs` via `Theme::custom()`.
```

## Design Notes

**Primary-as-white interactive states**: With white primary, the following use white-on-dark styling which gives a clean monochrome shadcn look:
- Active tab chips — white border, light gray background
- Body type buttons (active) — white text/border on gray
- Tree row drop highlight — white border on gray
- Drag preview border — white outline
- Drop lines and tab insertion markers — white

This is intentional and matches shadcn's minimal aesthetic. If these feel too subtle in practice during visual testing, the primary could be tinted slightly (e.g. `#e4e4e7` zinc-300) to add warmth without introducing a hue.

## Verification

- `cargo check` — compiles
- `cargo test` — tests pass
- `cargo run` — visual check: dark zinc backgrounds, white primary accents, vibrant colored method badges (green GET, amber POST, blue PUT, red DELETE, purple PATCH), no Catppuccin purple anywhere
