# Fix Missing Features - Critical Issues

## Problem Summary

Looking at the screenshot, there are 3 critical issues:

1. ❌ **"No active tab"** shows instead of URL input when no request is open
2. ❌ **"Add parameter functionality coming soon"** - add buttons aren't functional
3. ❌ **No way to create a request from the collection** - right-click doesn't work

---

## Issue 1: No Way to Open a Request ❗ CRITICAL

**What the screenshot shows:**
- "My Collection" exists in sidebar
- But NO requests visible under it
- No way to create a request

**Root cause:** The collection is empty. Need to add a request to it first.

### Fix: Add "New Request" to collection context menu

**Problem location:** Right-click on "My Collection" should show "New Request" but it doesn't work.

Check if:
- [ ] `sidebar.rs:231` - `on_item_click()` only handles request clicks, not collection clicks
- [ ] `sidebar.rs:550-598` - Context menu rendering exists but might not be wired to tree items

**Quick workaround:** Use the tab bar "+" button if visible, OR:

**Manual fix:**
1. Check if the tree items have `on_mouse_down(MouseButton::Right, ...)` handlers
2. The context menu logic exists (`render_context_menu()`) but needs to be triggered
3. The tree rendering (line 770+) needs right-click handlers on each item

---

## Issue 2: "No active tab" Empty State

**File:** `crates/reqforge-app/src/ui/request_editor.rs:794-806`

When `app_state.active_tab()` returns `None`, the URL input shows "No active tab".

### Solutions:

**Option A: Show a better empty state**
```rust
.unwrap_or_else(|| {
    div()
        .flex_1()
        .flex()
        .items_center()
        .justify_center()
        .child(
            v_flex()
                .gap_2()
                .items_center()
                .child("No request selected")
                .child("Right-click a collection to create a new request")
        )
})
```

**Option B: Auto-create a scratch tab on startup**

In `main.rs`, after creating `AppState`, add:
```rust
// Create a default scratch tab if no tabs exist
app_state.update(cx, |state, cx| {
    if state.tabs.is_empty() {
        let req = RequestDefinition::new("Untitled Request", HttpMethod::GET, "");
        let collection_id = state.core.collections.first()
            .map(|c| c.id)
            .unwrap_or_else(Uuid::new_v4);
        state.create_tab_from_request(&req, collection_id, window, cx);
    }
});
```

---

## Issue 3: "Add parameter functionality coming soon"

**File:** `crates/reqforge-app/src/ui/request_editor.rs:902-915`

The button exists but shows placeholder text instead of being functional.

### Fix: Wire the add button

Replace lines 902-915 with:
```rust
.child(
    h_flex()
        .p_2()
        .child(
            Button::new("add-param")
                .label("Add Parameter")
                .on_click(cx.listener(|this, _, window, cx| {
                    this.app_state.update(cx, |state, cx| {
                        if let Some(tab) = state.active_tab_mut() {
                            let key_input = cx.new(|cx| InputState::new(window, cx));
                            let value_input = cx.new(|cx| InputState::new(window, cx));
                            tab.params.push(crate::app_state::KeyValueRow {
                                id: Uuid::new_v4(),
                                enabled: true,
                                key_input,
                                value_input,
                            });
                        }
                        cx.notify();
                    });
                }))
        )
)
```

Do the same for headers tab (find similar "coming soon" text around line 950+).

---

## Issue 4: Tab Bar Not Visible

**Expected:** Tab bar should show "Untitled Request" when a tab is open.

**Actual:** Screenshot shows "No tabs open" (which is correct — no tabs ARE open).

**Fix:** Once Issue 1 is solved (create a request), tabs will appear automatically.

---

## Priority Fix Order

### 1. Enable Right-Click → New Request ⭐ HIGHEST PRIORITY

**File:** `crates/reqforge-app/src/ui/sidebar.rs`

The tree rendering (lines 770-900) creates list items but doesn't wire context menu:

**Add to the `list::ListItem` around line 846:**
```rust
.on_mouse_down(
    MouseButton::Right,
    cx.listener(move |this, event, _window, cx| {
        // Get click position
        let position = event.position;
        this.on_item_right_click(metadata.clone(), position.x, position.y, cx);
    })
)
```

This will trigger the existing `on_item_right_click()` method and show the context menu.

### 2. Wire "Add Parameter" and "Add Header" buttons

See Issue 3 above — replace placeholder divs with real `Button` components.

### 3. Better empty state messaging

See Issue 2 Option A — clearer instructions when no tab is open.

---

## Verification Checklist

- [ ] Right-click "My Collection" → see "New Request" menu item
- [ ] Click "New Request" → tab opens with URL input visible
- [ ] Click "Add Parameter" → new empty row appears
- [ ] Type in parameter key/value → text persists
- [ ] Click "Add Header" → new empty row appears
- [ ] Type URL → text persists
- [ ] Click "Send" → request executes

---

## Quick Test: Manual Request Creation

If right-click doesn't work, try this workaround:

**Option 1: Use CLI to create a request**
```bash
# Edit .reqforge/collections/<collection-id>.json
# Add a request manually to the "requests" object
```

**Option 2: Seed data in main.rs**
```rust
// After ReqForgeCore::open()
if core.collections.is_empty() {
    let mut collection = Collection::new("My Collection");
    let request = RequestDefinition::new("Test Request", HttpMethod::GET, "https://httpbin.org/get");
    collection.add_request(request);
    core.store.save_collection(&collection).unwrap();
}
```

This will give you a pre-populated collection on next run.

---

## Root Cause Analysis

The implementation is **95% complete** but has 3 small gaps:

1. ✅ Collection creation works
2. ✅ Tab management works
3. ✅ Input widgets work
4. ❌ **Right-click context menu not wired to tree items**
5. ❌ **"Add parameter/header" buttons show placeholder text**
6. ❌ **Empty state doesn't guide user to create first request**

Once the right-click handler is added, the entire workflow will work end-to-end.
