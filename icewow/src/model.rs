use iced::Point;

pub use icewow_engine::HttpMethod;
pub use crate::ui::scale::UiScale;
pub use crate::state::tree::NodeId;

pub type TabId = u64;

// ── Tab ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tab {
    pub id: TabId,
    pub request_id: Option<NodeId>,
    pub title: String,
    pub url_input: String,
    pub method: HttpMethod,
    pub body_type: BodyType,
    pub body_text: String,
    pub form_pairs: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub active_request_tab: RequestTab,
    pub query_params: Vec<(String, String)>,
    // Per-tab state (moved from AppState)
    pub response: Option<ResponseData>,
    pub loading: bool,
    pub active_response_tab: ResponseTab,
    pub dirty: bool,
}

// ── Enums ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyType {
    None,
    Raw,
    Json,
    Form,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragKind {
    Folder(NodeId),
    Request(NodeId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarDropTarget {
    Before {
        parent: Option<NodeId>,
        index: usize,
    },
    After {
        parent: Option<NodeId>,
        index: usize,
    },
    InsideFolder {
        folder_id: NodeId,
        index: usize,
    },
}

impl SidebarDropTarget {
    pub fn parent(self) -> Option<NodeId> {
        match self {
            Self::Before { parent, .. } | Self::After { parent, .. } => parent,
            Self::InsideFolder { folder_id, .. } => Some(folder_id),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DragState {
    Sidebar {
        kind: DragKind,
        source_parent: Option<NodeId>,
        source_index: usize,
        hover: Option<SidebarDropTarget>,
    },
    Tabs {
        tab_id: TabId,
        source_index: usize,
        hover_index: Option<usize>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteDialog {
    Folder(NodeId),
    Request(NodeId),
    Tab(TabId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextMenuTarget {
    ProjectRoot,
    Folder(NodeId),
    Request(NodeId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressKind {
    Sidebar {
        kind: DragKind,
        source_parent: Option<NodeId>,
        source_index: usize,
    },
    Tab {
        tab_id: TabId,
        source_index: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickAction {
    SelectRequest(NodeId),
    SelectFolder(NodeId),
    SelectTab(TabId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PendingLongPress {
    pub token: u64,
    pub kind: PressKind,
    pub click_action: Option<ClickAction>,
}

// ── Re-export ResponseData from engine ─────────────────────────

pub use icewow_engine::Response as ResponseData;

// ── AppState ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AppState {
    pub project_name: String,
    pub tree: crate::state::TreeArena,
    pub tabs: crate::state::TabStore,
    pub drag_state: Option<DragState>,
    pub pending_long_press: Option<PendingLongPress>,
    pub open_context_menu: Option<ContextMenuTarget>,
    pub context_menu_position: Option<Point>,
    pub delete_dialog: Option<DeleteDialog>,
    pub pointer_position: Point,
    pub window_size: iced::Size,
    pub ui_scale: UiScale,
    pub next_press_token: u64,
    pub selected_folder: Option<NodeId>,
}

impl AppState {
    pub fn sample() -> Self {
        let mut tree = crate::state::TreeArena::new();
        let (get_users, _create_user, list_products, _admin_subfolder, _health_check) =
            tree.build_sample();

        let mut tabs = crate::state::TabStore::new();

        // Open initial tabs
        let get_users_data = tree.get(get_users);
        let get_users_info = get_users_data.and_then(|e| match &e.data {
            crate::state::tree::NodeData::Request { name, url, method } => {
                Some((name.clone(), url.clone(), *method))
            }
            _ => None,
        });

        if let Some((name, url, method)) = get_users_info {
            tabs.open_for_request(get_users, name, url, method);
        }

        let list_products_data = tree.get(list_products);
        let list_products_info = list_products_data.and_then(|e| match &e.data {
            crate::state::tree::NodeData::Request { name, url, method } => {
                Some((name.clone(), url.clone(), *method))
            }
            _ => None,
        });

        if let Some((name, url, method)) = list_products_info {
            tabs.open_for_request(list_products, name, url, method);
        }

        Self {
            project_name: "Workspace Project".to_string(),
            tree,
            tabs,
            drag_state: None,
            pending_long_press: None,
            open_context_menu: None,
            context_menu_position: None,
            delete_dialog: None,
            pointer_position: Point::new(0.0, 0.0),
            window_size: iced::Size::new(1300.0, 820.0),
            ui_scale: UiScale::default(),
            next_press_token: 1,
            selected_folder: None,
        }
    }

    pub fn alloc_press_token(&mut self) -> u64 {
        let token = self.next_press_token;
        self.next_press_token += 1;
        token
    }

    pub fn open_request_tab(&mut self, request_id: crate::state::tree::NodeId) {
        use crate::state::tree::NodeData;

        let info = self.tree.get(request_id).and_then(|entry| match &entry.data {
            NodeData::Request { name, url, method } => Some((name.clone(), url.clone(), *method)),
            _ => None,
        });

        if let Some((name, url, method)) = info {
            self.tabs.open_for_request(request_id, name, url, method);
        }
    }
}
