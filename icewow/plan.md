# Plan: Restructure Layout to Match Postman

## Current Layout

```
┌──────────────────────────────────────────────┐
│  [ Tab 1 ]  [ Tab 2 ]  [ + ]                │  ← tab strip (full width, spaced chips)
├──────────────────────────────────────────────┤
│  [GET ▾] [  URL input                ] [Send]│  ← url bar (full width)
├──────────────┬───────────────────────────────┤
│  Sidebar     │  Main Panel                   │
│  (320px)     │  - method badge + title       │
│  - project   │  - headers editor             │
│  - tree      │  - body editor                │
│              │  - response                   │
└──────────────┴───────────────────────────────┘
```

Problems vs Postman:
- Tabs span full width (above sidebar) instead of being inside the content area
- URL bar spans full width (above sidebar) instead of being inside the content area
- No request name / save row
- Tabs have spacing/gaps between them (chip-style with rounded borders)
- Request section tabs (Params, Headers, Body) are inside main_panel, not a proper tab strip
- Response section is inline with request content, not a separate bottom section

## Target Layout (Postman-style)

```
┌─────────────┬──────────────────────────────────────────────────┐
│  Sidebar    │ [GET Get Users] [GET List Products] [+]          │  ← tabs (flush, method colored)
│  (280px)    ├──────────────────────────────────────────────────┤
│             │  Untitled Request                        [Save]  │  ← request name row
│  - project  ├──────────────────────────────────────────────────┤
│  - tree     │ [GET▾] [ URL input              ] [Send]         │  ← url bar
│             ├──────────────────────────────────────────────────┤
│             │ [Params] [Headers (2)] [Body]                    │  ← request section tabs
│             ├──────────────────────────────────────────────────┤
│             │  Key              │  Value                       │  ← request tab content
│             │  ─────────────────┼──────────                    │
│             │                   │                              │
│             ├──────────────────────────────────────────────────┤
│             │  Response                                        │
│             │  [200 OK] [150ms]                                │
│             │  [Body] [Cookies] [Headers]    ← response tabs   │
│             │  (response body)                                 │
└─────────────┴──────────────────────────────────────────────────┘
```

Key differences from current:
1. Sidebar is a full-height left panel — everything else is to its right
2. Tabs are inside the right content area, flush against each other (no spacing, no rounded chips — flat tabs with bottom border for active)
3. Each tab shows method badge prefix (colored "GET"/"POST" text before title)
4. New "request name + save" row below tabs
5. URL bar moves inside the right content area
6. Request section tabs (Params/Headers/Body) become a proper horizontal tab strip with count badges
7. Response is a distinct bottom section with its own tab strip (Body/Cookies/Headers)
8. "No tab" empty state when nothing is open

## Detailed Changes

### Phase 1: Restructure top-level layout (`app.rs` view)

**Current `view()` flow:**
```
column [
  tabs (full width)
  url_bar (full width)
  row [ sidebar | main_panel ]
]
```

**New `view()` flow:**
```
row [
  sidebar (280px, full height)
  column [                         ← right content area
    tabs (flush style)
    request_name_row               ← NEW
    url_bar
    request_section_tabs           ← refactored from main_panel
    request_content                ← body/headers/params content
    response_section               ← split out from main_panel
  ]
]
```

Changes to `app.rs`:
- Move `view_url_bar()` so it only fills the right side (already works once we restructure the outer layout)
- Add `view_request_name_row()` — shows request title + Save button
- Restructure `view()` to put sidebar first, then a column with everything else
- Remove top-level spacing/padding that creates gaps between sections

### Phase 2: Flatten tabs (`tabs.rs`)

Current tabs are rounded chips with 4px spacing. Postman tabs are flat, flush against each other, with an active indicator on the bottom border.

Changes to `tabs.rs`:
- Remove `.spacing(4)` from tabs_row — tabs sit flush
- Remove drop-zone spacing widgets between tabs (or make them 0-width when not dragging)
- Change tab chip style: no rounded corners, no border — instead use a bottom-border highlight for active tab
- Add method badge text (colored "GET"/"POST" etc.) before tab title
- Close `×` button repositioned for flat tab style — right-aligned inside the tab, subtle until hover

Changes to `styles.rs`:
- `tab_chip()`: change from rounded bordered chips to flat tabs — bottom border only for active, transparent otherwise. Background: subtle hover for inactive, slightly lighter for active.
- `tab_strip()`: remove rounded corners, make it a flat bar with a bottom border line
- `tab_insert()`: make thinner when inactive (1-2px)

### Phase 3: Add request name row

New section between tabs and URL bar showing:
```
[method badge] Request Name                    [Save]
```

This can live in `main_panel.rs` or as a new function in `app.rs`.

Changes:
- Add `Message::RequestNameChanged(String)` if we want editable names (or keep read-only for now)
- Add `Message::SaveRequest` (placeholder, no-op for now since this is UI-only)
- New view function `view_request_name_row()` in `main_panel.rs`

### Phase 4: Refactor main_panel into sections

Currently `main_panel.rs` is one big function that includes: title, headers editor, body editor, and response display all in one scrollable container.

Split into:

1. **`view_request_section_tabs()`** — horizontal tab strip: Params | Headers (N) | Body
   - Add `RequestTab` enum to model: `Params`, `Headers`, `Body`
   - Add `active_request_tab: RequestTab` to `Tab`
   - Add `Message::SetRequestTab(RequestTab)`
   - Show count badges next to Headers/Params when non-empty (e.g., "Headers (3)")

2. **`view_request_content()`** — renders the content for the active request tab
   - **Params tab**: key-value editor for query params
     - Add `query_params: Vec<(String, String)>` to `Tab` model
     - Add messages: `AddQueryParam`, `UpdateQueryParamKey(usize, String)`, `UpdateQueryParamValue(usize, String)`, `RemoveQueryParam(usize)`
     - Table layout with Key | Value columns
   - **Headers tab**: existing headers editor (restyled as table)
   - **Body tab**: existing body type selector + body editor

3. **`view_response_section()`** — the response display, separated by a visual divider
   - Header row: status badge + elapsed time
   - Response tabs: Body | Cookies | Headers
     - Add `ResponseTab` enum: `Body`, `Cookies`, `Headers`
     - Add `active_response_tab: ResponseTab` to `AppState` (shared, not per-tab)
     - Body tab: response body text (existing)
     - Cookies tab: empty placeholder for now
     - Headers tab: response headers list (existing, moved here)
   - Has its own scrollable container

### Phase 5: Sidebar adjustments

Minor tweaks:
- Reduce width from 320px to 280px (Postman uses ~260-280px)
- Remove outer rounded border — sidebar should feel like a structural panel, not a card
- Add a right-side border/separator line instead of the rounded container look
- Remove outer padding so the sidebar stretches edge-to-edge vertically

### Phase 6: Empty states

When no tab is selected, the right content area should show a centered empty state:
- Postman shows an illustration + "Enter the URL and click Send to get a response"
- We'll show a simpler text-based placeholder: "Open a request or create a new tab to get started"
- No URL bar, no section tabs — just the tab strip (with only the `+` button) and the empty state

### Phase 7: Proper shadcn/Tailwind Theme System

The current theme relies on Iced's auto-generated palette (`palette.background.weak`, `palette.secondary.strong`, etc.) which produces muddy, low-contrast colors that don't match a real design system. We need to replace this with a proper shadcn-style token system using exact Tailwind color values.

**Iced supports alpha/opacity** via `Color::from_rgba(r, g, b, a)` (f32 0.0–1.0) — we already use this for `modal_backdrop` and shadows. This lets us replicate shadcn v4's opacity-based borders (`white @ 10%`, `white @ 15%`).

#### Current Problems

1. **Badge text unreadable** — GET/PUT/PATCH text blends into dark tinted background. Auto-generated strong/weak pairs have poor contrast.
2. **Sidebar tree rows invisible** — `palette.secondary.weak` is a muddy gray barely distinguishable from the near-black background.
3. **No surface hierarchy** — sidebar, content area, and cards all use near-identical auto-generated grays.
4. **Flat text hierarchy** — everything is either full white or nothing. No muted/secondary text.
5. **Borders barely visible** — auto-generated `palette.background.strong` is too close to the background.

#### Design: shadcn v4 Zinc Dark Token System

shadcn maps Tailwind zinc shades to semantic roles. We port this directly to Rust constants.

**shadcn dark theme variable → Tailwind shade → hex:**

```
--background          → zinc-950  → #09090b
--foreground          → zinc-50   → #fafafa
--card                → zinc-900  → #18181b
--card-foreground     → zinc-50   → #fafafa
--popover             → zinc-900  → #18181b
--popover-foreground  → zinc-50   → #fafafa
--primary             → zinc-200  → #e4e4e7   (v4 uses lighter primary)
--primary-foreground  → zinc-900  → #18181b
--secondary           → zinc-800  → #27272a
--secondary-foreground→ zinc-50   → #fafafa
--muted               → zinc-800  → #27272a
--muted-foreground    → zinc-400  → #a1a1aa
--accent              → zinc-800  → #27272a
--accent-foreground   → zinc-50   → #fafafa
--destructive         → red-400   → #f87171   (v4 uses bright red)
--border              → white@10% → rgba(255,255,255,0.10)
--input               → white@15% → rgba(255,255,255,0.15)
--ring                → zinc-500  → #71717a
--sidebar             → zinc-900  → #18181b
--sidebar-foreground  → zinc-50   → #fafafa
--sidebar-accent      → zinc-800  → #27272a
--sidebar-border      → white@10% → rgba(255,255,255,0.10)
```

#### Tailwind Zinc Scale (complete reference)

```
zinc-50:  #fafafa    zinc-500: #71717a
zinc-100: #f4f4f5    zinc-600: #52525b
zinc-200: #e4e4e7    zinc-700: #3f3f46
zinc-300: #d4d4d8    zinc-800: #27272a
zinc-400: #a1a1aa    zinc-900: #18181b
                     zinc-950: #09090b
```

#### Tailwind Color Palettes (for method badges, status, accents)

```
Green (GET):
  400: #4ade80   500: #22c55e   600: #16a34a   900: #14532d   950: #052e16

Yellow (POST):
  400: #facc15   500: #eab308   600: #ca8a04   900: #713f12   950: #422006

Sky (PUT):
  400: #38bdf8   500: #0ea5e9   600: #0284c7   900: #0c4a6e   950: #082f49

Red (DELETE):
  400: #f87171   500: #ef4444   600: #dc2626   900: #7f1d1d   950: #450a0a

Violet (PATCH):
  400: #a78bfa   500: #8b5cf6   600: #7c3aed   900: #4c1d95   950: #2e1065

Blue (primary accent):
  400: #60a5fa   500: #3b82f6   600: #2563eb   700: #1d4ed8   900: #1e3a8a   950: #172554

Orange (warning alt):
  400: #fb923c   500: #f97316   600: #ea580c   900: #7c2d12   950: #431407

Emerald (success alt):
  400: #34d399   500: #10b981   600: #059669   900: #064e3b   950: #022c22
```

### Changes to `theme.rs` — Full Rewrite

Replace the minimal theme file with a comprehensive token system:

```rust
use iced::Color;
use iced::theme::Palette;
use iced::Theme;

// ============================================================
// Iced Palette (feeds auto-generation — we override most of it
// in style functions, but Iced still needs this for defaults)
// ============================================================
const ICEWOW_DARK: Palette = Palette {
    background: Color::from_rgb8(0x09, 0x09, 0x0b),  // zinc-950
    text:       Color::from_rgb8(0xfa, 0xfa, 0xfa),  // zinc-50
    primary:    Color::from_rgb8(0x3b, 0x82, 0xf6),  // blue-500
    success:    Color::from_rgb8(0x22, 0xc5, 0x5e),  // green-500
    warning:    Color::from_rgb8(0xea, 0xb3, 0x08),  // yellow-500
    danger:     Color::from_rgb8(0xef, 0x44, 0x44),  // red-500
};

pub fn theme() -> Theme {
    Theme::custom("IceWow Dark", ICEWOW_DARK)
}

// ============================================================
// Tailwind Zinc Scale
// ============================================================
pub mod zinc {
    use iced::Color;
    pub const S50:  Color = Color::from_rgb8(0xfa, 0xfa, 0xfa);
    pub const S100: Color = Color::from_rgb8(0xf4, 0xf4, 0xf5);
    pub const S200: Color = Color::from_rgb8(0xe4, 0xe4, 0xe7);
    pub const S300: Color = Color::from_rgb8(0xd4, 0xd4, 0xd8);
    pub const S400: Color = Color::from_rgb8(0xa1, 0xa1, 0xaa);
    pub const S500: Color = Color::from_rgb8(0x71, 0x71, 0x7a);
    pub const S600: Color = Color::from_rgb8(0x52, 0x52, 0x5b);
    pub const S700: Color = Color::from_rgb8(0x3f, 0x3f, 0x46);
    pub const S800: Color = Color::from_rgb8(0x27, 0x27, 0x2a);
    pub const S900: Color = Color::from_rgb8(0x18, 0x18, 0x1b);
    pub const S950: Color = Color::from_rgb8(0x09, 0x09, 0x0b);
}

// ============================================================
// Tailwind Color Scales (shades used in the app)
// ============================================================
pub mod green {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0x4a, 0xde, 0x80);
    pub const S500: Color = Color::from_rgb8(0x22, 0xc5, 0x5e);
    pub const S600: Color = Color::from_rgb8(0x16, 0xa3, 0x4a);
    pub const S950: Color = Color::from_rgb8(0x05, 0x2e, 0x16);
}

pub mod yellow {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0xfa, 0xcc, 0x15);
    pub const S500: Color = Color::from_rgb8(0xea, 0xb3, 0x08);
    pub const S600: Color = Color::from_rgb8(0xca, 0x8a, 0x04);
    pub const S950: Color = Color::from_rgb8(0x42, 0x20, 0x06);
}

pub mod sky {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0x38, 0xbd, 0xf8);
    pub const S500: Color = Color::from_rgb8(0x0e, 0xa5, 0xe9);
    pub const S600: Color = Color::from_rgb8(0x02, 0x84, 0xc7);
    pub const S950: Color = Color::from_rgb8(0x08, 0x2f, 0x49);
}

pub mod red {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0xf8, 0x71, 0x71);
    pub const S500: Color = Color::from_rgb8(0xef, 0x44, 0x44);
    pub const S600: Color = Color::from_rgb8(0xdc, 0x26, 0x26);
    pub const S900: Color = Color::from_rgb8(0x7f, 0x1d, 0x1d);
    pub const S950: Color = Color::from_rgb8(0x45, 0x0a, 0x0a);
}

pub mod violet {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0xa7, 0x8b, 0xfa);
    pub const S500: Color = Color::from_rgb8(0x8b, 0x5c, 0xf6);
    pub const S600: Color = Color::from_rgb8(0x7c, 0x3a, 0xed);
    pub const S950: Color = Color::from_rgb8(0x2e, 0x10, 0x65);
}

pub mod blue {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0x60, 0xa5, 0xfa);
    pub const S500: Color = Color::from_rgb8(0x3b, 0x82, 0xf6);
    pub const S600: Color = Color::from_rgb8(0x25, 0x63, 0xeb);
    pub const S700: Color = Color::from_rgb8(0x1d, 0x4e, 0xd8);
    pub const S950: Color = Color::from_rgb8(0x17, 0x25, 0x54);
}

pub mod orange {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0xfb, 0x92, 0x3c);
    pub const S500: Color = Color::from_rgb8(0xf9, 0x73, 0x16);
    pub const S600: Color = Color::from_rgb8(0xea, 0x58, 0x0c);
    pub const S950: Color = Color::from_rgb8(0x43, 0x14, 0x07);
}

pub mod emerald {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0x34, 0xd3, 0x99);
    pub const S500: Color = Color::from_rgb8(0x10, 0xb9, 0x81);
    pub const S600: Color = Color::from_rgb8(0x05, 0x96, 0x69);
    pub const S950: Color = Color::from_rgb8(0x02, 0x2c, 0x22);
}

// ============================================================
// shadcn Semantic Tokens (mapped from zinc scale)
// ============================================================

/// App background — zinc-950
pub const BACKGROUND: Color = zinc::S950;
/// Primary text — zinc-50
pub const FOREGROUND: Color = zinc::S50;

/// Card / popover surface — zinc-900
pub const CARD: Color = zinc::S900;
/// Card text — zinc-50
pub const CARD_FOREGROUND: Color = zinc::S50;

/// Primary accent — blue-500 (we use blue, not white like shadcn default)
pub const PRIMARY: Color = blue::S500;
/// Text on primary — zinc-50
pub const PRIMARY_FOREGROUND: Color = zinc::S50;

/// Secondary surfaces — zinc-800
pub const SECONDARY: Color = zinc::S800;
/// Secondary text — zinc-50
pub const SECONDARY_FOREGROUND: Color = zinc::S50;

/// Muted surfaces — zinc-800
pub const MUTED: Color = zinc::S800;
/// Muted text — zinc-400
pub const MUTED_FOREGROUND: Color = zinc::S400;

/// Accent surface (hover bg) — zinc-800
pub const ACCENT: Color = zinc::S800;
/// Accent text — zinc-50
pub const ACCENT_FOREGROUND: Color = zinc::S50;

/// Destructive — red-500
pub const DESTRUCTIVE: Color = red::S500;
/// Destructive foreground — zinc-50
pub const DESTRUCTIVE_FOREGROUND: Color = zinc::S50;

/// Border — white at 10% opacity (shadcn v4 style)
pub const BORDER: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.10 };
/// Input border — white at 15% opacity
pub const INPUT: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.15 };
/// Focus ring — zinc-500
pub const RING: Color = zinc::S500;

/// Sidebar background — zinc-900
pub const SIDEBAR: Color = zinc::S900;
/// Sidebar text — zinc-50
pub const SIDEBAR_FOREGROUND: Color = zinc::S50;
/// Sidebar accent (hover/selected) — zinc-800
pub const SIDEBAR_ACCENT: Color = zinc::S800;
/// Sidebar border — white at 10%
pub const SIDEBAR_BORDER: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.10 };

// ============================================================
// Opacity helpers — replicate shadcn's `hsl(var(--x) / 0.5)`
// ============================================================

/// Apply alpha to any color: `with_alpha(zinc::S50, 0.5)` = zinc-50 at 50%
pub const fn with_alpha(c: Color, a: f32) -> Color {
    Color { r: c.r, g: c.g, b: c.b, a }
}

// Pre-defined opacity variants used across styles
/// White at 5% — very subtle hover highlights
pub const WHITE_5: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.05 };
/// White at 10% — borders (shadcn --border)
pub const WHITE_10: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.10 };
/// White at 15% — input borders (shadcn --input)
pub const WHITE_15: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.15 };
/// White at 20% — stronger borders, dividers
pub const WHITE_20: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.20 };

// ============================================================
// Method badge colors (explicit high-contrast)
// Pattern: -400 for text, -950 for bg, -600 for border
// ============================================================

pub struct MethodColors {
    pub text: Color,
    pub bg: Color,
    pub border: Color,
}

pub fn method_colors(method: crate::model::HttpMethod) -> MethodColors {
    use crate::model::HttpMethod;
    match method {
        HttpMethod::Get => MethodColors {
            text: green::S400, bg: green::S950, border: green::S600,
        },
        HttpMethod::Post => MethodColors {
            text: yellow::S400, bg: yellow::S950, border: yellow::S600,
        },
        HttpMethod::Put => MethodColors {
            text: sky::S400, bg: sky::S950, border: sky::S600,
        },
        HttpMethod::Delete => MethodColors {
            text: red::S400, bg: red::S950, border: red::S600,
        },
        HttpMethod::Patch => MethodColors {
            text: violet::S400, bg: violet::S950, border: violet::S600,
        },
    }
}

/// Just the text color for a method (for inline colored "GET" labels in tabs)
pub fn method_text_color(method: crate::model::HttpMethod) -> Color {
    method_colors(method).text
}
```

### Changes to `styles.rs`

Migrate **all** style functions away from `theme.extended_palette()` to use `theme::*` tokens directly. The `&Theme` parameter is kept for API compatibility but most functions will ignore the palette.

Key rewrites:
- `method_badge()` → `theme::method_colors(method)` for text/bg/border
- `status_badge()` → explicit red/yellow/green -400/-950/-600 combos
- `tree_row(selected, hover)` → `SIDEBAR_ACCENT` selected, `WHITE_5` hover, transparent default
- `sidebar_panel()` → `SIDEBAR` bg, right-only border with `SIDEBAR_BORDER`
- `panel()` → `CARD` bg, `BORDER` border, reduced corner radius
- `tab_chip(active)` → transparent bg, `MUTED_FOREGROUND` text; active: `CARD` bg, `FOREGROUND` text, blue-500 bottom border
- `tab_strip()` → `BACKGROUND` bg, bottom border with `BORDER`
- `handle_button()` → `MUTED_FOREGROUND` text, `WHITE_5` hover bg
- `send_button()` → `PRIMARY` bg, `PRIMARY_FOREGROUND` text, `blue::S600` hover
- `secondary_button()` → `SECONDARY` bg, `SECONDARY_FOREGROUND` text, `ACCENT` hover
- `danger_button()` → `red::S900` bg, `DESTRUCTIVE_FOREGROUND` text, `red::S600` hover
- `context_menu()` → `CARD` bg, `BORDER` border
- `modal_card()` → `CARD` bg, `BORDER` border
- `modal_backdrop()` → `Color::from_rgba(0.0, 0.0, 0.0, 0.60)`
- Input fields → `with_alpha(WHITE_15)` border, `CARD` bg (inputs are visible against `BACKGROUND`)
- `body_type_button(active)` → active: `PRIMARY` text + blue-950 bg; inactive: `MUTED_FOREGROUND` text
- `drop_line(active)` → `PRIMARY` when active, transparent when not
- `drag_preview()` → `CARD` bg, `PRIMARY` border, shadow

## New Model Additions (`model.rs`)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestTab {
    Params,
    Headers,
    Body,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseTab {
    Body,
    Cookies,
    Headers,
}
```

Add to `Tab`:
```rust
pub active_request_tab: RequestTab,
pub query_params: Vec<(String, String)>,
```

Add to `AppState`:
```rust
pub active_response_tab: ResponseTab,
```

Default `RequestTab::Params`, `ResponseTab::Body`.

## New Messages (`app.rs`)

```rust
Message::SetRequestTab(RequestTab),
Message::SetResponseTab(ResponseTab),
Message::SaveRequest,                          // no-op placeholder
Message::RequestNameChanged(String),           // editable request name
Message::AddQueryParam,
Message::UpdateQueryParamKey(usize, String),
Message::UpdateQueryParamValue(usize, String),
Message::RemoveQueryParam(usize),
```

## File-by-File Summary

### `model.rs`
- Add `RequestTab` enum (Params, Headers, Body)
- Add `ResponseTab` enum (Body, Cookies, Headers)
- Add `active_request_tab: RequestTab` and `query_params: Vec<(String, String)>` to `Tab`
- Add `active_response_tab: ResponseTab` to `AppState`
- Initialize defaults in all constructors

### `app.rs`
- Add new Message variants (see above)
- Restructure `view()`: `row![ sidebar, column![ tabs, name_row, url_bar, section_tabs, content, response ] ]`
- Add empty state view when no tab is active
- Remove top-level `spacing(8)` and `padding(8)`
- `update()`: handle all new messages

### `ui/tabs.rs`
- Remove spacing between tabs (flush layout)
- Add method badge text (colored method name) before tab title
- Change tab_drop_zone to be minimal/zero width when not dragging
- Flat tab style instead of rounded chips
- `×` close button: subtle, right-aligned

### `ui/main_panel.rs`
- Split into: `view_request_name_row()`, `view_request_section_tabs()`, `view_request_content()`, `view_response_section()`
- Remove the old monolithic `view_main_panel()`
- Each section is a separate pub function called from `app.rs` view
- Params tab: new query_params key-value table
- Response section gets its own tab strip (Body/Cookies/Headers)

### `ui/sidebar.rs`
- Reduce width to 280px
- Change from rounded card to flat panel with right border only

### `ui/theme.rs` — Full Rewrite
- Complete Tailwind zinc scale (`zinc::S50` through `zinc::S950`)
- Color modules: `green`, `yellow`, `sky`, `red`, `violet`, `blue`, `orange`, `emerald` with relevant shades
- shadcn semantic tokens: `BACKGROUND`, `FOREGROUND`, `CARD`, `PRIMARY`, `SECONDARY`, `MUTED`, `ACCENT`, `DESTRUCTIVE`, `BORDER`, `INPUT`, `RING`, `SIDEBAR`, etc.
- Opacity-based colors: `WHITE_5`, `WHITE_10`, `WHITE_15`, `WHITE_20` using `Color { r, g, b, a }`
- `with_alpha(color, alpha)` const helper for ad-hoc opacity
- `MethodColors` struct + `method_colors()` + `method_text_color()` for high-contrast badge/tab colors
- Keep `theme()` function returning `Theme::custom()`

### `ui/styles.rs` — Full Rewrite
- Migrate **every** style function from `theme.extended_palette()` to `theme::*` semantic tokens
- `method_badge()` → `theme::method_colors()` for text/bg/border
- `status_badge()` → explicit tailwind color combos for 2xx/4xx/5xx
- `tree_row()` → `SIDEBAR_ACCENT` selected, `WHITE_5` hover, transparent default
- `sidebar_panel()` → `SIDEBAR` bg, right-only border with `SIDEBAR_BORDER`
- `panel()` → `CARD` bg, `BORDER`, reduced radius
- `tab_chip()` → flat tab, `MUTED_FOREGROUND` inactive, `FOREGROUND` + blue bottom-border active
- `tab_strip()` → `BACKGROUND` bg, `BORDER` bottom
- All button styles → use semantic tokens (`PRIMARY`, `SECONDARY`, `DESTRUCTIVE`, etc.)
- Input fields → `CARD` bg, `INPUT` border (white@15%)
- Add `request_section_tab()`, `response_section_tab()`, `save_button()`, `content_area()`
- `modal_backdrop()` → `rgba(0,0,0,0.60)`, `modal_card()` → `CARD` bg
- `drop_line()` / `drag_preview()` → `PRIMARY` accent

### `ui/components.rs`
- Add `save_button()` component
- Update `method_badge()` to use new theme colors

## Verification

- `cargo check` — compiles
- `cargo test` — tree_ops tests still pass
- `cargo run` — visual check:
  - Sidebar on left, full height, flat panel with right border, clearly distinct surface color
  - Tabs flush inside right content area, flat style with bottom-border active indicator, method colored prefix
  - Request name row with Save button
  - URL bar below name row
  - Params / Headers / Body tab strip with count badges
  - Tab content area (key-value table)
  - Response section at bottom with own tab strip (Body/Cookies/Headers)
  - Empty state shown when no tab is open
  - Clear text hierarchy (bright/base/dim)
  - Readable method badges with high-contrast text
  - Visible distinction between sidebar, content area, and selected tree rows
  - No outer padding gaps, no rounded card borders on structural panels
