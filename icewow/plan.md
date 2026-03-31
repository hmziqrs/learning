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

### Phase 7: Theme overhaul

The current theme has several visual problems:

#### Problem 1: Badge text unreadable
Method badges (GET, PUT, etc.) use `palette.success.strong.color` as text on `palette.success.weak.color` as background. Iced auto-generates `.strong` and `.weak` from the base palette color, but the auto-generated contrast ratios are poor — the text blends into the dark tinted background, especially for GET (green-on-dark-green) and PUT (sky-on-dark-sky).

**Fix:** Stop relying on Iced's auto-generated strong/weak pairs for badges. Define explicit high-contrast text colors per method. Use bright/saturated text on a very subtle dark-tinted background:

```
GET     → text: #4ade80 (green-400)   bg: #052e16 (green-950)   border: #16a34a (green-600)
POST    → text: #facc15 (yellow-400)  bg: #422006 (yellow-950)  border: #ca8a04 (yellow-600)
PUT     → text: #38bdf8 (sky-400)     bg: #082f49 (sky-950)     border: #0284c7 (sky-600)
DELETE  → text: #f87171 (red-400)     bg: #450a0a (red-950)     border: #dc2626 (red-600)
PATCH   → text: #a78bfa (violet-400)  bg: #2e1065 (violet-950)  border: #7c3aed (violet-600)
```

The key: -400 shades for text (bright, saturated), -950 shades for background (very dark, barely tinted), -600 for borders. This gives clear contrast on the dark theme.

#### Problem 2: Sidebar tree rows — hard gray on dark background
The tree rows use `palette.secondary.weak.color` for selected state and `Color::TRANSPARENT` for unselected. The secondary palette is auto-generated from background (#09090b), which produces a muddy gray that barely differs from the background. The contrast between selected and unselected rows is almost invisible.

**Fix:** Define explicit surface colors in `theme.rs` for sidebar states:

```rust
pub const SURFACE_0: Color = Color::from_rgb8(0x0a, 0x0a, 0x0f);  // sidebar bg (near-black, slight blue)
pub const SURFACE_1: Color = Color::from_rgb8(0x14, 0x14, 0x1b);  // content bg / card bg
pub const SURFACE_2: Color = Color::from_rgb8(0x1e, 0x1e, 0x28);  // hover state / elevated
pub const SURFACE_3: Color = Color::from_rgb8(0x2a, 0x2a, 0x35);  // selected / active state
pub const BORDER:    Color = Color::from_rgb8(0x27, 0x27, 0x2f);  // subtle borders
pub const TEXT_DIM:  Color = Color::from_rgb8(0x71, 0x71, 0x7a);  // secondary/muted text (zinc-500)
pub const TEXT_BASE: Color = Color::from_rgb8(0xa1, 0xa1, 0xaa);  // normal text (zinc-400)
pub const TEXT_BRIGHT: Color = Color::from_rgb8(0xfa, 0xfa, 0xfa); // emphasis text (zinc-50)
```

Use these directly in style functions instead of palette-derived values for:
- `tree_row()`: use SURFACE_3 for selected, SURFACE_2 for hover, SURFACE_0 for default
- `sidebar_panel()`: use SURFACE_0 as background
- `panel()`: use SURFACE_1 as background
- All borders: use BORDER instead of `palette.background.strong.color`

#### Problem 3: Lack of visual hierarchy between surfaces
Everything uses the auto-generated background palette which produces surfaces that are too similar. Postman has clear distinction between sidebar (darkest), content area (slightly lighter), and cards/editors (even lighter).

**Fix:** The surface scale above (SURFACE_0 through SURFACE_3) creates a clear elevation hierarchy. Apply consistently:
- SURFACE_0: sidebar, tab strip background
- SURFACE_1: main content area, panels
- SURFACE_2: hover states, input field backgrounds
- SURFACE_3: selected/active states, pressed buttons

#### Problem 4: Text hierarchy is flat
All text is either the auto-generated `palette.background.base.text` (white) or nothing. No distinction between labels, content, and de-emphasized text.

**Fix:** Use the three text levels:
- TEXT_BRIGHT (#fafafa): titles, active tab labels, primary content
- TEXT_BASE (#a1a1aa): body text, tree item labels, input values
- TEXT_DIM (#71717a): secondary labels ("Headers", "Body"), placeholder text, timestamps

### Changes to `theme.rs`

Expand to export surface/text/border constants alongside the palette:

```rust
use iced::{Color, Theme};
use iced::theme::Palette;

// --- Palette (feeds Iced's auto-generation) ---
const ICEWOW_DARK: Palette = Palette {
    background: Color::from_rgb8(0x09, 0x09, 0x0b),
    text:       Color::from_rgb8(0xfa, 0xfa, 0xfa),
    primary:    Color::from_rgb8(0x3b, 0x82, 0xf6),
    success:    Color::from_rgb8(0x22, 0xc5, 0x5e),
    warning:    Color::from_rgb8(0xea, 0xb3, 0x08),
    danger:     Color::from_rgb8(0xef, 0x44, 0x44),
};

// --- Surfaces (elevation scale) ---
pub const SURFACE_0: Color = Color::from_rgb8(0x0a, 0x0a, 0x0f);
pub const SURFACE_1: Color = Color::from_rgb8(0x14, 0x14, 0x1b);
pub const SURFACE_2: Color = Color::from_rgb8(0x1e, 0x1e, 0x28);
pub const SURFACE_3: Color = Color::from_rgb8(0x2a, 0x2a, 0x35);

// --- Borders ---
pub const BORDER: Color = Color::from_rgb8(0x27, 0x27, 0x2f);

// --- Text hierarchy ---
pub const TEXT_BRIGHT: Color = Color::from_rgb8(0xfa, 0xfa, 0xfa);
pub const TEXT_BASE: Color   = Color::from_rgb8(0xa1, 0xa1, 0xaa);
pub const TEXT_DIM: Color    = Color::from_rgb8(0x71, 0x71, 0x7a);

// --- Method colors (explicit, high-contrast) ---
pub struct MethodColors {
    pub text: Color,
    pub bg: Color,
    pub border: Color,
}

pub fn method_colors(method: crate::model::HttpMethod) -> MethodColors {
    use crate::model::HttpMethod;
    match method {
        HttpMethod::Get => MethodColors {
            text:   Color::from_rgb8(0x4a, 0xde, 0x80),  // green-400
            bg:     Color::from_rgb8(0x05, 0x2e, 0x16),  // green-950
            border: Color::from_rgb8(0x16, 0xa3, 0x4a),  // green-600
        },
        HttpMethod::Post => MethodColors {
            text:   Color::from_rgb8(0xfa, 0xcc, 0x15),  // yellow-400
            bg:     Color::from_rgb8(0x42, 0x20, 0x06),  // yellow-950
            border: Color::from_rgb8(0xca, 0x8a, 0x04),  // yellow-600
        },
        HttpMethod::Put => MethodColors {
            text:   Color::from_rgb8(0x38, 0xbd, 0xf8),  // sky-400
            bg:     Color::from_rgb8(0x08, 0x2f, 0x49),  // sky-950
            border: Color::from_rgb8(0x02, 0x84, 0xc7),  // sky-600
        },
        HttpMethod::Delete => MethodColors {
            text:   Color::from_rgb8(0xf8, 0x71, 0x71),  // red-400
            bg:     Color::from_rgb8(0x45, 0x0a, 0x0a),  // red-950
            border: Color::from_rgb8(0xdc, 0x26, 0x26),  // red-600
        },
        HttpMethod::Patch => MethodColors {
            text:   Color::from_rgb8(0xa7, 0x8b, 0xfa),  // violet-400
            bg:     Color::from_rgb8(0x2e, 0x10, 0x65),  // violet-950
            border: Color::from_rgb8(0x7c, 0x3a, 0xed),  // violet-600
        },
    }
}

pub fn theme() -> Theme {
    Theme::custom("IceWow Dark", ICEWOW_DARK)
}
```

### Changes to `styles.rs`

Migrate all style functions to use `theme::SURFACE_*`, `theme::BORDER`, `theme::TEXT_*`, and `theme::method_colors()` instead of `palette.background.*` / `palette.secondary.*` / `palette.success.*` etc.

Key rewrites:
- `method_badge()` → use `theme::method_colors(method)` for text/bg/border
- `status_badge()` → similar explicit colors for 2xx/4xx/5xx
- `tree_row()` → SURFACE_3 selected, SURFACE_2 hover, transparent default
- `sidebar_panel()` → SURFACE_0 bg, right-only border with BORDER color
- `panel()` → SURFACE_1 bg, BORDER color, no rounded corners
- `tab_chip()` → SURFACE_2 active bg with bottom border in primary, TEXT_BRIGHT for active text, TEXT_BASE for inactive
- `tab_strip()` → SURFACE_0 bg, bottom border with BORDER
- `handle_button()` → TEXT_DIM color, SURFACE_2 hover
- All input fields → SURFACE_2 bg so they're visible against SURFACE_1 panels
- `context_menu()` → SURFACE_2 bg, BORDER border
- `modal_card()` → SURFACE_1 bg

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

### `ui/theme.rs`
- Add SURFACE_0/1/2/3 constants
- Add BORDER constant
- Add TEXT_BRIGHT/BASE/DIM constants
- Add `MethodColors` struct and `method_colors()` function
- Keep existing `theme()` function

### `ui/styles.rs`
- Rewrite `method_badge()` → use `theme::method_colors()`
- Rewrite `tree_row()` → use SURFACE constants
- Rewrite `sidebar_panel()` → SURFACE_0, right border only
- Rewrite `panel()` → SURFACE_1, no rounded corners
- Rewrite `tab_chip()` → flat tab with bottom border
- Rewrite `tab_strip()` → SURFACE_0, flat
- Rewrite all button styles → use TEXT/SURFACE constants
- Add `request_section_tab()` style for Params/Headers/Body
- Add `response_section_tab()` style
- Add `save_button()` style
- Add `content_area()` style for right-side content background

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
