# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo run          # Run the application
cargo build        # Debug build
cargo build --release  # Optimized build
cargo test         # Run tests (tree_ops unit tests)
cargo check        # Quick compile check without producing binary
```

## Architecture

IceWow is a Postman-like API request builder UI prototype built with [Iced 0.14](https://github.com/iced-rs/iced) (Elm-inspired Rust GUI framework). It is UI-only — request execution is intentionally not implemented.

### Component Overview

```
main.rs → PostmanUiApp
  ├── model.rs       — All data structures and application state
  ├── app.rs         — Message enum, update() handler, view(), subscriptions
  ├── tree_ops.rs    — Pure tree manipulation functions with unit tests
  └── ui/
      ├── mod.rs         — delete_modal, drag_preview_overlay
      ├── sidebar.rs     — Recursive tree rendering, drag-drop zones
      ├── tabs.rs        — Tab strip with reordering
      ├── main_panel.rs  — Content area (shows active tab/request info)
      ├── styles.rs      — Theme-aware styling functions
      └── theme.rs       — Custom shadcn-inspired dark theme palette
```

### Data Model (`model.rs`)

- `TreeNode`: enum of `FolderNode` (with children) or `RequestNode`
- `AppState`: top-level state holding the tree, open tabs, drag state, and overlay state
- `DragState`: tracks whether a sidebar item or tab is being dragged
- `PendingLongPress`: 220ms long-press gesture → initiates drag
- `SidebarDropTarget`: Before/After/InsideFolder — the three possible drop positions

### Message Flow

All interaction flows through `Message` variants in `app.rs`. The `update()` method handles every message and returns `Task`s for side effects (timers for long-press, scroll commands). Mouse/window events arrive via `subscription()`.

### Tree Operations (`tree_ops.rs`)

Pure functions — no side effects, easy to test. Key functions:
- `insert_node()` — insert at a `SidebarDropTarget` position
- `move_node()` — move with validation (prevents moving a folder into its own descendant)
- `remove_folder()` / `remove_request()` — deletion with cascade cleanup
- `set_folder_expanded()` — toggle expansion by id

### Drag-and-Drop

Long-press (220ms `PendingLongPress` timer) initiates drag. Mouse position is tracked via subscriptions. Drop zones are rendered as `drop_line()` hover targets in `sidebar.rs`. On pointer release, `move_node()` is called. A `drag_preview_overlay()` floats near the cursor during drag.

### Styling

All style functions are in `ui/styles.rs`. The app uses a custom shadcn-inspired dark theme defined in `ui/theme.rs` via `Theme::custom()`. Style functions accept `&Theme` and return `iced::widget::container::Style` or similar.
