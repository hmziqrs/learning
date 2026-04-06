# Fixes & Remaining Work

Tracked issues, deferred items, and cleanup tasks from the architecture refactor.

---

## Dead Code Warnings (all future API — keep, suppress)

All 26 compiler warnings are future API surface, not dead weight. Suppress with `#[allow(dead_code)]` where appropriate.

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

- [ ] `view_sidebar` takes full `&AppState` — needs `project_name`, `tree`, `tabs`, `drag_state`, `selected_folder`, `ui_scale` (6 slices). Narrowing may not be worth the ergonomic cost; decide and document.
- [ ] `view_url_bar` lives on `PostmanUiApp` (`app.rs:278-330`) — emits both `EditorMsg` and `HttpMsg`, inherently cross-feature. Either keep in `app.rs` with a comment explaining why, or move to a shared view helper.
- [ ] `overlays.rs` imports `crate::app::Message` directly — `delete_modal` and `drag_preview_overlay` return `Element<Message>` instead of a feature-specific type. Deferred from Phase 1; needs a design decision (dedicated `OverlayMsg` sub-enum, or accept the coupling).
- [ ] `modal_card` style (`styles.rs:251`) accepts `_scale: &UiScale` but ignores it — either use it or remove the parameter.

---

## Hardcoded Literals (deferred from Phase 1)

### Tracked in plan

- [ ] `badges.rs:9,22` — `.size(13)` should use `scale.text_body()`; badges have no `&UiScale` access yet
- [ ] `badges.rs:10` — `.padding([6, 10])` should use `scale.pad_button()`
- [ ] `badges.rs:23` — `.padding([4, 8])` should use `scale.pad_chip()`
- [ ] `buttons.rs` `menu_button`, `danger_button`, `secondary_button` — no explicit padding set (Iced default); add `&UiScale` and `scale.pad_button()` or `scale.pad_chip()`
- [ ] `sidebar.rs:508-513` `empty_folder_state` — `.size(12)`, `.spacing(2)`, `.padding([3.0, 0.0])`, `.padding([0.0, 6.0])`; static function with no scale access
- [ ] `tabs.rs:89` method label — `.size(11)` unique size between caption (10) and small (12); add `text_xs()` to `UiScale` when needed

### NOT tracked in plan (oversight)

- [ ] `overlays.rs:29` `delete_modal` — `.spacing(14)` should use `scale.space_lg()` (12 at default) or a new spacing token
- [ ] `overlays.rs:31` `delete_modal` — `.padding(18)` should use a scale method (closest is `scale.pad_panel()` = 10, may need a new `pad_modal()`)
- [ ] `sidebar.rs:415,481` — `.padding([3.0, 0.0])` on folder/request row content containers; structural but could use `scale.space_xs()`
- [ ] `sidebar.rs:425,493` — `.padding([0.0, scale.space_sm()])` OK, but drop_line at line 707 uses `.padding([0.0, 6.0])` hardcoded
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
