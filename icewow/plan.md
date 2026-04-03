# IceWow Architecture Review & Scaling Recommendations

## Context

IceWow is a Postman-like API client built with Iced 0.14 (Elm architecture). Currently ~3,580 lines across 19 files. The app works well at its current scope but several architectural decisions will become painful as features grow (environments, auth, collections, history, workspaces, pre-request scripts, etc.).

---

## Part 1: Shortcomings & Scaling Pitfalls

### Tier 1: Data Bugs (will cause incorrect behavior)

**1. Duplicated URL state** (`model.rs:161` + `model.rs:36`)
- `AppState.url_input` duplicates `Tab.url_input`
- `UrlChanged` at `app.rs:205-209` writes to both, but `sync_url_input_from_active_tab()` only copies tab->global
- Risk: desync on tab switch if any code path forgets to sync

**2. Global response shared across tabs** (`model.rs:173`)
- Single `Option<ResponseData>` for all tabs
- Send request on Tab A, switch to Tab B, response arrives -> overwrites Tab B's view
- `loading: bool` is also global -- can't show per-tab loading spinners

**3. Orphaned tab->request references** (`model.rs:34`)
- `Tab.request_id: Option<RequestId>` can point to deleted requests
- Deletion at `app.rs:157-159` uses `retain()` to clean up, but only catches the delete path -- any other code that removes tree nodes (future refactors) must remember to also clean tabs

**4. `#[allow(dead_code)]` on `find_parent_and_index`** (`tree_ops.rs:68`)
- Suggests incomplete refactoring -- function exists but is unused

### Tier 2: Performance (degrades with scale)

**5. O(n) tab lookup every frame** (`model.rs:344-352`)
- `active_tab_mut()`/`active_tab_ref()` linear-scan `Vec<Tab>` by ID
- Called from `view()` (every frame) and most `update()` arms
- With 50+ open tabs, this adds up

**6. O(n) tree search for every request lookup** (`model.rs:372-389`)
- `find_request()` does full recursive traversal
- Called on tab open (`app.rs:661`), drag preview (`app.rs:700`), etc.

**7. O(3n) drag-drop move** (`tree_ops.rs:104-145`)
- `move_node()` does: `remove_folder/request` (scan 1) -> `is_descendant` (scan 2) -> `insert_node` (scan 3)
- Three full tree traversals per drop operation

**8. Unfiltered pointer subscription** (`app.rs:516-524`)
- Every mouse move dispatches `PointerMoved` even when nothing is being dragged
- At 60fps = 3,600 messages/minute doing nothing useful when idle

**9. Recursive tree with no depth limit**
- `FolderNode.children: Vec<TreeNode>` is unbounded recursion
- Deep nesting (100+ levels) risks stack overflow in all 8 recursive `tree_ops` functions

### Tier 3: Maintainability (slows development)

**10. Monolithic Message enum -- 58 variants flat** (`app.rs:18-76`)
- Adding one new key-value collection (like auth params) requires 4+ new variants
- No grouping -- tree ops, HTTP config, drag-drop, UI state all mixed

**11. Monolithic update() -- 425 lines** (`app.rs:88-513`)
- Single match statement handles everything
- Repeated patterns: `UpdateFormKey/Value`, `UpdateHeaderKey/Value`, `UpdateQueryParamKey/Value` are copy-paste at `app.rs:426-507`

**12. 8+ recursive tree traversals, each reimplements recursion** (`tree_ops.rs` + `app.rs:805-883`)
- `tree_ops.rs`: `insert_node`, `remove_folder`, `remove_request`, `is_descendant`, `find_parent_and_index`, `move_node`, `collect_request_ids`, `set_folder_expanded` -- all manual recursion
- `app.rs` adds 4 more private helpers: `children_len`, `folder_expanded`, `find_folder_name`, `update_request_method`
- No shared visitor/iterator pattern

**13. 3 identical key-value editors copy-pasted** (`ui/main_panel.rs:208-239, 241-272, 305-329`)
- Params, headers, and form pairs are structurally identical but each is its own function with its own message variants

**14. Magic numbers scattered everywhere**
- Sidebar width `280.0` (only in `sidebar.rs:18`)
- Response height `200.0` (`main_panel.rs:188`)
- Tree indent `16.0` (11 locations in `sidebar.rs`)
- 8 different text sizes (10, 11, 12, 13, 14, 15, 16, 20) across all files
- Padding tuples `[4,12]`, `[6,10]`, `[8,12]`, `[4,10]` -- no consistent scale

**15. All view functions take `&PostmanUiApp`** (full state access)
- Can't test or reuse components in isolation
- `view_request_name_row(app)` only needs current tab, but can read anything

**16. Style functions ignore `&Theme` parameter** (`ui/styles.rs`)
- All 25 functions accept `_theme: &Theme` but use hardcoded constants from `theme.rs`
- Works fine for single theme, but blocks runtime theme switching

### Tier 4: Missing Infrastructure

**17. No persistence** -- state lost on close, `sample()` recreated each session
**18. No undo/redo** -- all mutations irreversible
**19. No validation** -- URLs not parsed, JSON not validated, IDs not verified before use
**20. HTTP engine incomplete** -- `Error::Timeout` defined but never raised (`engine/src/error.rs`), no retry, no connection pooling config, response always loaded fully into memory
**21. Cookies tab shown in UI but no cookie extraction** (`ResponseTab::Cookies` exists, no handler)

---

## Part 2: Recommended Architecture

### Folder Structure

```
src/
|-- main.rs                    # Entry point (unchanged)
|-- app.rs                     # Thin router: delegates to feature handlers
|
|-- state/                     # All data structures & indexed stores
|   |-- mod.rs                 # Re-exports AppState
|   |-- tree.rs                # TreeArena (HashMap-based tree with O(1) lookup)
|   |-- tabs.rs                # TabStore (HashMap<TabId, Tab> + ordered Vec<TabId>)
|   |-- workspace.rs           # WorkspaceState (project name, selected folder, UI flags)
|   |-- drag.rs                # DragState, PendingLongPress, drop targets
|   +-- ids.rs                 # IdAllocator (unified ID generation, persistence-ready)
|
|-- features/                  # One module per domain, each owns Message + update + view
|   |-- sidebar/
|   |   |-- mod.rs             # pub SidebarMsg enum + update fn
|   |   |-- view.rs            # view_sidebar(), context_menu, tree rendering
|   |   +-- drag.rs            # Sidebar drag-drop logic (finish_sidebar_drag, auto-scroll)
|   |-- editor/
|   |   |-- mod.rs             # pub EditorMsg enum + update fn
|   |   +-- view.rs            # URL bar, request tabs, body/headers/params editors
|   |-- response/
|   |   |-- mod.rs             # pub ResponseMsg enum + update fn
|   |   +-- view.rs            # Response display (body, headers, cookies)
|   |-- tabs/
|   |   |-- mod.rs             # pub TabsMsg enum + update fn
|   |   +-- view.rs            # Tab strip rendering + tab drag
|   +-- http/
|       +-- mod.rs             # pub HttpMsg enum + update fn (send request, handle result)
|
|-- ui/                        # Stateless shared primitives (no feature/state imports)
|   |-- mod.rs                 # Re-exports
|   |-- tokens.rs              # NEW: spacing scale, text sizes, layout constants, radii
|   |-- theme.rs               # Color palette (unchanged)
|   |-- styles.rs              # Style functions (use tokens instead of magic numbers)
|   |-- components.rs          # Reusable widgets: kv_editor, method_badge, modal, etc.
|   +-- icons.rs               # Icon helpers
|
engine/                        # HTTP engine crate (unchanged)
```

### Key Design Decisions

#### A. TreeArena replaces recursive `Vec<TreeNode>`

```rust
// state/tree.rs
pub struct TreeArena {
    nodes: HashMap<NodeId, TreeEntry>,
    root_children: Vec<NodeId>,    // ordered display list
    next_id: u64,
}

pub struct TreeEntry {
    pub data: NodeData,            // Folder { name, expanded } | Request { name, url, method }
    pub parent: Option<NodeId>,    // back-pointer for O(depth) ancestor check
    pub children: Vec<NodeId>,     // ordered children (empty for requests)
}
```

**Why**: Eliminates all 12 recursive traversal functions. `find_request` becomes `self.nodes.get(&id)` -- O(1). `is_ancestor` walks parent pointers -- O(depth) not O(n). `move_node` removes from source parent's `children` vec and inserts into target's -- O(children_count) not O(tree_size). No more `node.clone()` during recursion.

#### B. Split Message enum into feature sub-enums

```rust
// app.rs -- thin router
pub enum Message {
    Sidebar(SidebarMsg),
    Editor(EditorMsg),
    Response(ResponseMsg),
    Tabs(TabsMsg),
    Http(HttpMsg),
    // 3-4 truly global variants
    PointerMoved(Point),
    PointerReleased,
    WindowResized(Size),
    IconFontLoaded(Result<(), font::Error>),
}

// Root update() shrinks to ~30 lines of delegation:
pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::Sidebar(msg) => sidebar::update(&mut self.state, msg),
        Message::Editor(msg) => editor::update(&mut self.state, msg),
        // ...
        Message::PointerMoved(pos) => { self.state.drag.pointer = pos; Task::none() }
        Message::PointerReleased => self.handle_pointer_release(),
    }
}
```

**Why**: Each feature module can grow independently. Adding "environments" is a new `features/environments/` directory with its own `EnvironmentsMsg` -- zero changes to other features.

#### C. Per-tab response & loading state

```rust
// state/tabs.rs
pub struct Tab {
    // ... existing fields ...
    pub response: Option<ResponseData>,  // MOVED from AppState
    pub loading: bool,                   // MOVED from AppState
    pub active_response_tab: ResponseTab, // MOVED from AppState
}
```

**Why**: Fixes the race condition. Each tab tracks its own response independently. Remove `AppState.url_input` entirely -- always read from active tab.

#### D. TabStore with O(1) lookup

```rust
pub struct TabStore {
    tabs: HashMap<TabId, Tab>,
    order: Vec<TabId>,            // display order
    active: Option<TabId>,
}

impl TabStore {
    pub fn active(&self) -> Option<&Tab> {
        self.active.and_then(|id| self.tabs.get(&id))
    }
    pub fn active_mut(&mut self) -> Option<&mut Tab> {
        self.active.and_then(|id| self.tabs.get_mut(&id))
    }
    pub fn ordered(&self) -> impl Iterator<Item = &Tab> {
        self.order.iter().filter_map(|id| self.tabs.get(id))
    }
}
```

**Why**: `active_tab_mut()` goes from O(n) linear scan to O(1) HashMap lookup.

#### E. Design tokens replace magic numbers

```rust
// ui/tokens.rs
pub mod text {
    pub const CAPTION: f32 = 10.0;
    pub const SMALL: f32 = 12.0;
    pub const BODY: f32 = 13.0;
    pub const LABEL: f32 = 14.0;
    pub const TITLE: f32 = 16.0;
    pub const HEADING: f32 = 20.0;
}

pub mod spacing {
    pub const XS: f32 = 2.0;
    pub const SM: f32 = 4.0;
    pub const MD: f32 = 8.0;
    pub const LG: f32 = 12.0;
    pub const XL: f32 = 16.0;
}

pub mod layout {
    pub const SIDEBAR_WIDTH: f32 = 280.0;
    pub const TREE_INDENT: f32 = 16.0;
    pub const RESPONSE_MIN_HEIGHT: f32 = 200.0;
    pub const MODAL_WIDTH: f32 = 380.0;
    pub const CONTEXT_MENU_WIDTH: f32 = 210.0;
}

pub mod radius {
    pub const SM: f32 = 4.0;
    pub const MD: f32 = 6.0;
    pub const LG: f32 = 8.0;
}

pub mod pad {
    pub const CHIP: [u16; 2] = [4, 10];
    pub const BUTTON: [u16; 2] = [6, 12];
    pub const INPUT: u16 = 10;
    pub const PANEL: u16 = 10;
}
```

#### F. Generic key-value editor eliminates copy-paste

```rust
// ui/components.rs
pub fn kv_editor<'a, M: Clone + 'a>(
    pairs: &'a [(String, String)],
    key_placeholder: &str,
    value_placeholder: &str,
    on_key: impl Fn(usize, String) -> M + 'a,
    on_value: impl Fn(usize, String) -> M + 'a,
    on_remove: impl Fn(usize) -> M + 'a,
    on_add: M,
) -> Element<'a, M>
```

Replaces `view_params_editor`, `view_headers_editor`, and the form pair editor body.

#### G. Narrow view function signatures

```rust
// Before (every function sees everything):
pub fn view_request_name_row(app: &PostmanUiApp) -> Element<Message>

// After (only what's needed):
pub fn view_request_name_row(tab: &Tab) -> Element<EditorMsg>
```

Each feature's view returns `Element<FeatureMsg>`. Root `view()` maps them: `.map(Message::Editor)`.

---

## Part 3: Implementation Phases

Each phase produces a compiling, working app.

### Phase 1: Design Tokens (low risk, immediate cleanup)
- Create `ui/tokens.rs` with all constants
- Replace every magic number across `sidebar.rs`, `main_panel.rs`, `styles.rs`, `mod.rs`, `tabs.rs`
- Zero functional change, pure refactor

### Phase 2: Data Model (medium risk, highest value)
- Build `TreeArena` in `state/tree.rs` with methods: `get`, `get_mut`, `insert`, `remove`, `move_node`, `is_ancestor`, `children`, `walk`
- Build `TabStore` in `state/tabs.rs` with HashMap + ordered vec
- Move `response`, `loading`, `active_response_tab` into `Tab`
- Remove `AppState.url_input`
- Delete `tree_ops.rs` entirely -- all logic now in `TreeArena` methods
- Update `app.rs` update/view to use new data structures

### Phase 3: Message Split & Feature Modules
- Create `features/` directory structure
- Extract sub-enums: `SidebarMsg`, `EditorMsg`, `ResponseMsg`, `TabsMsg`, `HttpMsg`
- Extract update handlers into feature modules
- Root `update()` becomes thin delegation

### Phase 4: View Decoupling
- Move view functions into feature `view.rs` files
- Narrow signatures from `&PostmanUiApp` to specific state slices
- Extract generic `kv_editor` component
- Each feature view returns `Element<FeatureMsg>`, root maps with `.map()`

### Phase 5: Polish
- Gate pointer subscription on `drag.is_active()` to avoid idle message spam
- Add depth limit to `TreeArena` insert (prevent runaway nesting)
- Clean up dead code (`find_parent_and_index`)

---

## Verification

After each phase:
- `cargo check` -- compiles
- `cargo test` -- tree operation tests pass (Phase 2 requires rewriting tests for TreeArena)
- `cargo run` -- visual smoke test: create folders/requests, drag-drop, open tabs, switch tabs, delete items, send request
