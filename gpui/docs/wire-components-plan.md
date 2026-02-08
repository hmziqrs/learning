# Plan: Wire Real Components into RootView (Postman-like Layout)

## Context

The app has **fully implemented components** (`SidebarPanel`, `RequestTabBar`, `RequestEditor`, `ResponseViewer`, `EnvSelector`) — but `RootView` doesn't use any of them. Instead, `root.rs` renders everything inline with placeholder text ("Sidebar Panel - Click collections to expand", "No Request Selected", etc.). The real components are dead code. This plan wires them in and establishes the Postman-like layout:

```
┌──────────────────────────────────────────────────────┐
│ [Sidebar]  │  [EnvSelector]              top bar     │
│            ├─────────────────────────────────────────│
│  Collections│  [Tab1] [Tab2] [Tab3]      tab bar     │
│  tree       ├─────────────────────────────────────────│
│             │  [GET ▼] [URL input] [Send] url bar    │
│             │  [Params] [Headers] [Body]  sub-tabs   │
│             │  (key-value content)        editor     │
│             ├─────────────────────────────────────────│
│             │  200 OK · 125ms · 4.2KB    status bar  │
│             │  [Body] [Headers]           resp tabs  │
│             │  { "data": ... }            resp body  │
└──────────────────────────────────────────────────────┘
```

## Root Cause

**`root.rs:386-395`** — `RootView::render()` calls `self.render_sidebar(cx)` and `self.render_main_area(cx)`, which are inline methods returning placeholder divs. The real `Entity<SidebarPanel>`, `Entity<RequestTabBar>`, `Entity<RequestEditor>`, `Entity<ResponseViewer>`, `Entity<EnvSelector>` are never created or rendered.

## Changes

### Step 1: Update `RootView` struct to hold component entities

**File:** `crates/reqforge-app/src/ui/root.rs`

Add entity fields to `RootView`:
```rust
pub struct RootView {
    app_state: Entity<AppState>,
    sidebar: Entity<SidebarPanel>,
    tab_bar: Entity<RequestTabBar>,
    request_editor: Entity<RequestEditor>,
    response_viewer: Entity<ResponseViewer>,
    env_selector: Entity<EnvSelector>,
}
```

### Step 2: Create all entities in `RootView::new()`

**File:** `crates/reqforge-app/src/ui/root.rs`

In `new()`, create each component entity using `cx.new()`:
```rust
pub fn new(app_state: Entity<AppState>, cx: &mut Context<Self>) -> Self {
    let sidebar = cx.new(|cx| SidebarPanel::new(app_state.clone(), cx));
    let tab_bar = cx.new(|cx| RequestTabBar::new(app_state.clone(), cx));
    let request_editor = cx.new(|cx| RequestEditor::new(app_state.clone(), cx));
    let response_viewer = cx.new(|_cx| ResponseViewer::new(app_state.clone()));
    let env_selector = cx.new(|cx| EnvSelector::new(app_state.clone(), cx));
    Self { app_state, sidebar, tab_bar, request_editor, response_viewer, env_selector }
}
```

### Step 3: Replace `render()` with Postman-like layout

**File:** `crates/reqforge-app/src/ui/root.rs`

Delete all the inline render methods (`render_sidebar`, `render_env_selector`, `render_request_editor`, `render_response_viewer`, `render_main_area`). Replace `Render::render()` with:

```rust
fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    h_flex()
        .size_full()
        .bg(cx.theme().background)
        .text_color(cx.theme().foreground)
        // Left: Sidebar (fixed 300px)
        .child(
            div()
                .w(px(300.0))
                .h_full()
                .border_r_1()
                .border_color(cx.theme().border)
                .child(self.sidebar.clone())
        )
        // Right: Main area (flex-1)
        .child(
            v_flex()
                .flex_1()
                .h_full()
                // Top bar with env selector
                .child(
                    h_flex()
                        .h(px(40.0))
                        .px_2()
                        .items_center()
                        .justify_between()
                        .border_b_1()
                        .border_color(cx.theme().border)
                        .child(div().text_sm().font_semibold().child("ReqForge"))
                        .child(self.env_selector.clone())
                )
                // Tab bar
                .child(self.tab_bar.clone())
                // Request editor (top half)
                .child(
                    div()
                        .flex_1()
                        .min_h(px(200.0))
                        .child(self.request_editor.clone())
                )
                // Response viewer (bottom half)
                .child(
                    div()
                        .flex_1()
                        .min_h(px(200.0))
                        .child(self.response_viewer.clone())
                )
        )
}
```

### Step 4: Add missing imports to `root.rs`

Add imports for the component types:
```rust
use super::{SidebarPanel, RequestTabBar, RequestEditor, ResponseViewer, EnvSelector};
```

### Step 5: Update `main.rs` if needed

The current `main.rs` creates `RootView` with `ui::RootView::new(app_state, cx)`. The signature doesn't change — `new()` still takes `(Entity<AppState>, &mut Context<Self>)`. No changes expected, but verify compilation.

## Files Modified

| File | Change |
|------|--------|
| `crates/reqforge-app/src/ui/root.rs` | Rewrite: add entity fields, create in `new()`, replace render with real components |

That's it — **one file change**. All the components already exist and work.

## Verification

1. `cargo check -p reqforge-app` — must compile cleanly
2. `cargo test -p reqforge-app` — existing tests pass
3. `cargo run -p reqforge-app` — window shows:
   - Left sidebar with collection tree (folders, requests, method badges)
   - Tab bar across top of main area
   - Request editor with method dropdown, URL input, Send button, sub-tabs
   - Response viewer with status bar, body/headers tabs
   - Environment selector dropdown in top-right
