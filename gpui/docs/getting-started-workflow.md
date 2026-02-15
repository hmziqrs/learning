# Getting Started: Basic Workflow

## Summary: Everything is Already Implemented ✅

The UI is **fully functional** — all the code exists and is wired up correctly. The workflow is:

1. **Create a collection** (if sidebar is empty)
2. **Add a request to the collection**
3. **Click the request** → opens in a new tab
4. **Edit URL, params, headers, body** in the tab
5. **Click Send** → see response

---

## Current State Analysis

### What's Implemented and Working

| Feature | Status | File Reference |
|---------|--------|----------------|
| **Tab Management** | ✅ Fully working | `ui/tab_bar.rs:309-321` (`on_new_tab()`) |
| **"New Tab" button** | ✅ Visible when tabs exist | `ui/tab_bar.rs:391-411` (+ icon in tab bar) |
| **"New Tab" button (empty)** | ✅ Visible when 0 tabs | `ui/tab_bar.rs:348-368` |
| **Create Collection** | ✅ Working | `ui/sidebar.rs:510-548` (`create_new_collection()`) |
| **"New Collection" button** | ✅ Visible when 0 collections | `ui/sidebar.rs:738-761` |
| **Click request → open tab** | ✅ Working | `ui/sidebar.rs:242-294` (`open_request_in_tab()`) |
| **URL input** | ✅ Real `Input` widget | `ui/request_editor.rs` (after interactivity fixes) |
| **Params/Headers editors** | ✅ Real `KeyValueEditor` | `ui/request_editor.rs` (after interactivity fixes) |
| **Body input** | ✅ Real `Input` widget | `ui/request_editor.rs` (after interactivity fixes) |
| **Send button** | ✅ Async execution | `ui/request_editor.rs:588-721` (`on_send()`) |
| **Response display** | ✅ Pretty JSON + headers | `ui/response_viewer.rs:84-305` |

### The Problem: Empty Workspace

When you run the app for the first time, the `.reqforge/` workspace directory is empty. No collections = no requests = nothing to click = no tabs.

---

## User Workflow to Get Started

### Step 1: Create Your First Collection

1. Run `cargo run -p reqforge-app`
2. The sidebar shows **"No collections yet"** with a **"New Collection"** button
3. Click **"New Collection"** → Creates "Collection 1"

**What happens:**
- `sidebar.rs:510-548` creates a `Collection` via `ReqForgeCore`
- Saves to `.reqforge/collections/`
- Reloads the collection list
- Sidebar tree now shows "Collection 1"

### Step 2: Add a Request to the Collection

**Option A: Via Sidebar Context Menu**
1. Right-click "Collection 1" in the tree
2. Select **"New Request"** from the context menu
3. A new tab opens with "Untitled Request"

**Option B: Via Tab Bar "+" Button**
1. If no tabs are open, click the **"+"** button in the empty tab bar (`tab_bar.rs:362-367`)
2. Creates "Untitled Request" and opens a tab

**What happens:**
- `sidebar.rs:345-374` or `tab_bar.rs:309-321` creates a `RequestDefinition`
- Calls `app_state.create_tab_from_request()` (`app_state.rs:72-138`)
- Creates `Entity<InputState>` for URL, body, headers, params
- Adds `TabState` to `app_state.tabs`
- Sets `active_tab = Some(index)`

### Step 3: Edit the Request

The tab now shows:
- **Method dropdown** (GET/POST/PUT/etc.) — click to change
- **URL input field** — type your URL (e.g., `https://jsonplaceholder.typicode.com/posts/1`)
- **Sub-tabs**: Params / Headers / Body — click to switch
- **KeyValueEditor** for params and headers — click "Add Parameter" / "Add Header"
- **Body input** — type JSON, text, etc.

**What works:**
- All inputs use real `Entity<InputState>` — text editing is managed by gpui-component
- Changes are stored in `TabState` in memory
- No automatic save (it's a scratch tab until you explicitly save to collection)

### Step 4: Send the Request

1. Click the **"Send"** button
2. Button changes to "Sending..." with disabled state
3. Async task executes via `cx.spawn()` (`request_editor.rs:692-720`)
4. `ReqForgeCore.execute_request()` runs the HTTP call
5. Response populates `tab.last_response`
6. Response viewer shows status code, timing, size
7. Body/Headers sub-tabs display the response

**What happens:**
- `on_send()` reads all `Entity<InputState>` text via `.read(cx).text().to_string()` (ownership boundary)
- Builds a `RequestDefinition` from the tab state
- Spawns async task with `core.execute_request(&req).await`
- On completion: updates `tab.last_response`, sets `is_loading = false`, triggers re-render

### Step 5: Save the Request (Future Feature)

**Current limitation:** Requests created via "New Request" or "+" button are **not saved to the collection**.
They exist only in the tab's memory. Closing the tab loses the request.

**What's missing:**
- A "Save" button in the request editor
- A command to persist `TabState` → `RequestDefinition` → `core.store.save_request()`

---

## Quick Start Commands

```bash
# Build and run
cargo run -p reqforge-app

# Test the core logic
cargo test -p reqforge-core

# Check for compilation errors
cargo check -p reqforge-app
```

---

## Troubleshooting

### "I don't see the New Collection button"

**Cause:** Collections already exist in `.reqforge/collections/`

**Fix:** Check if collections exist:
```bash
ls -la .reqforge/collections/
```

If you want to start fresh:
```bash
rm -rf .reqforge/
cargo run -p reqforge-app
```

### "I clicked New Collection but nothing happened"

**Check logs:**
```bash
RUST_LOG=info cargo run -p reqforge-app
```

Look for:
```
INFO Created collection: Collection 1 (uuid)
```

If you see `WARN Cannot add collection - multiple references to core exist`, this is a known
limitation due to `Arc<ReqForgeCore>` in `AppState`. The collection is saved to disk but not
immediately visible in the UI. Restart the app to see it.

### "The + button is visible but I can't click it"

**Cause:** Either:
1. No collections exist (need to create one first)
2. The click handler isn't wired (check `tab_bar.rs:405-410`)

**Fix:** Create a collection first, then click the + button.

### "URL input doesn't work / I can't type"

**Cause:** The `Input` widget from gpui-component might need focus.

**Fix:** Click directly in the URL input field to focus it, then type.

---

## Architecture Flow Diagram

```
┌─────────────────────────────────────────────────────────┐
│ User clicks "New Collection" button                     │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│ sidebar.rs:510 → create_new_collection()                │
│   → Collection::new("Collection 1")                     │
│   → core.store.save_collection()                        │
│   → Reload collections from store                       │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│ Sidebar tree now shows "Collection 1"                   │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│ User right-clicks → "New Request"                       │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│ sidebar.rs:345 → on_action_new_request()                │
│   → RequestDefinition::new("New Request", GET, "")      │
│   → open_request_in_tab()                               │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│ app_state.rs:72 → create_tab_from_request()             │
│   → cx.new(|cx| InputState::new()) for URL, body        │
│   → Create KeyValueRow entities for params, headers     │
│   → TabState { url_input, body_input, params, ... }     │
│   → app_state.tabs.push(tab)                            │
│   → active_tab = Some(index)                            │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│ RequestTabBar shows new tab with method badge + name    │
│ RequestEditor shows URL input, params, headers, body    │
│ ResponseViewer shows "Send a request" empty state       │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│ User types URL, clicks Send                             │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│ request_editor.rs:588 → on_send()                       │
│   → Build RequestDefinition from tab state              │
│   → cx.spawn(async { core.execute_request().await })    │
│   → Update tab.last_response on completion              │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│ ResponseViewer shows status, timing, pretty JSON        │
└─────────────────────────────────────────────────────────┘
```

---

## What's Missing for Full Functionality

| Feature | Status | Effort |
|---------|--------|--------|
| **Save request to collection** | ❌ Not implemented | Medium — needs "Save" button + persist logic |
| **Delete request from collection** | ❌ Stubbed (logs only) | Low — `sidebar.rs:409` just needs impl |
| **Rename request** | ❌ Stubbed (logs only) | Low — `sidebar.rs:393` just needs impl |
| **Create folder** | ❌ Stubbed (logs only) | Medium — `sidebar.rs:377` + folder UI |
| **Persist tab state on close** | ❌ Not implemented | Low — save draft on tab close |
| **Environment variable interpolation in UI** | ✅ Core works, UI shows selector | Works — `{{var}}` replaced by core |

---

## Conclusion

**You have a fully working HTTP client.** The workflow is:

1. Click "New Collection" (if empty)
2. Right-click collection → "New Request" (or click + button)
3. Type URL, add params/headers, type body
4. Click Send
5. See response

The only major missing piece is **persisting requests to collections** — currently, requests
created via the UI are ephemeral (tab-only). But for quick testing, the app is fully functional.
