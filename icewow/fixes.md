# Fixes & Remaining Work

Tracked issues, deferred items, and cleanup tasks from the architecture refactor.

---

## Dead Code Warnings (all future API — keep, suppress)

All 26 compiler warnings are future API surface, not dead weight. Suppressed with `#[allow(dead_code)]`.

### State/feature API (used when features grow)

- `TabsMsg::SelectTab` (`features/tabs.rs:15`) — cross-feature communication; needed when another feature programmatically switches tabs via `Task::done(Message::Tabs(TabsMsg::SelectTab(id)))`
- `TreeArena::set_name` (`state/tree.rs:340`) — needed for inline folder/request renaming in the sidebar
- `TabStore::ordered` (`state/tabs.rs:55`) — simpler iterator for cases not needing indices (export, search all tabs)
- `TabStore::is_empty` (`state/tabs.rs:70`) — standard collection API for empty-state guards
- `TabStore::position` (`state/tabs.rs:83`) — tab drag-drop position calculations
- `TreeArena::get_mut`, `contains`, `children_of` (`state/tree.rs`) — used in tests, flagged only in non-test builds

### App-level API (no UI to trigger yet)

- `Message::SetDensity` / `Message::SetFontScale` (`app.rs:33-34`) — Phase 5 additions, need settings panel
- `Density::Compact` / `Density::Spacious` (`scale.rs:4,6`) — needed when density switching UI exists
- `UiScale::icon_lg` / `UiScale::RADIUS_LG` (`scale.rs:48,114`) — scale palette completeness

### Theme palette constants

Unused `S500`, `S100`-`S300`, `S600`, `S700` across color modules in `theme.rs`, plus `CARD_FOREGROUND`, `MUTED`, `DESTRUCTIVE`, `RING`, `SIDEBAR_FOREGROUND`, `WHITE_15`, `WHITE_20`. Part of the shadcn palette — will be used as features grow. Add `#[allow(dead_code)]` at module level on `theme.rs`.

---

## Phase 4: View Decoupling (remaining items)

### Already completed (update plan checkboxes)

- [x] Move view functions into feature `view.rs` files — all features have views in their module
- [x] Extract generic `kv_editor` into `components/editors.rs`
- [x] Feature views return `Element<FeatureMsg>`, root maps with `.map()` — editor, response, tabs, sidebar all done
- [x] Show dirty indicator on tab chip — `features/tabs.rs:93-97`
- [x] Narrow signatures for editor (`&Tab, &UiScale`), response (`&Tab, &UiScale`), tabs (`&TabStore, &Option<DragState>, &UiScale`)

### Still outstanding

- [x] `view_sidebar` takes full `&AppState` — documented with comment; narrowing deferred (not worth ergonomic cost)
- [x] `view_url_bar` lives on `PostmanUiApp` — documented with comment explaining cross-feature nature
- [x] `overlays.rs` imports `crate::app::Message` directly — FIXME comment added; deferred pending design decision
- [x] `modal_card` style accepts `_scale` — documented as reserved for future density-aware modal sizing

---

## Hardcoded Literals (deferred from Phase 1)

### Tracked in plan

- [x] `badges.rs` — now takes `&UiScale`, uses `scale.text_body()`, `scale.pad_badge_method()`, `scale.pad_badge_status()`
- [ ] `buttons.rs` `menu_button`, `danger_button`, `secondary_button` — no explicit padding set (Iced default); add `&UiScale` and `scale.pad_button()` or `scale.pad_chip()`
- [x] `sidebar.rs` `empty_folder_state` — now takes `&UiScale`, uses `scale.text_small()`, `scale.space_xs()`, `scale.space_sm()`
- [x] `tabs.rs` method label — now uses `scale.text_xs()` (new method added to `UiScale`)

### NOT tracked in plan (oversight)

- [x] `overlays.rs` `delete_modal` — `.spacing(14)` → `scale.space_lg()`, `.padding(18)` → `scale.pad_modal()` (new method)
- [x] `sidebar.rs` folder_row/request_row — `.padding([3.0, 0.0])` → `scale.space_xs()`
- [x] `sidebar.rs` drop_line — `.padding([0.0, 6.0])` → `scale.space_sm()`; now takes `&UiScale`
- [ ] `tabs.rs:153` `tab_drop_zone` — `.width(Length::Fixed(16.0))` / `.width(Length::Fixed(2.0))` and `.height(Length::Fixed(28.0))` hardcoded structural dimensions

---

## Design Gaps

### SaveRequest only persists name/url/method

`editor.rs:148-169` copies `title`, `url_input`, `method` from tab draft to `TreeArena` via `update_request_from_draft`. Headers, body_text, body_type, form_pairs, and query_params are NOT saved because `NodeData::Request` only stores `{ name, url, method }`.

**Impact**: User edits headers/body, clicks Save, closes tab, reopens request — headers/body are gone.

**Resolution**: When persistence (#21) is built, expand the data model with a separate request body store (e.g., `HashMap<NodeId, RequestBody>` in `AppState` or on-disk). The tree should remain lightweight for sidebar rendering. Until then, document this as a known limitation.

### No `tree_ops.rs` — all logic in `TreeArena` methods

This is correct and intentional. Noting here so nobody recreates it.

---

## Structural Widths (keep as-is)

These are layout dimensions, not user-facing text. Not worth scaling:

- `20.0` — icon container width in sidebar rows
- `18.0` — folder icon container width
- `36.0` — method label container width in sidebar
- `28.0` — tab drop zone height
- `240.0` — drag preview width
- `280.0` — sidebar width (`UiScale::SIDEBAR_WIDTH`)
- `210.0` — context menu width (`UiScale::CONTEXT_MENU_WIDTH`)
- `380.0` — modal width (`UiScale::MODAL_WIDTH`)
- `200.0` — response min height (`UiScale::RESPONSE_MIN_HEIGHT`)
