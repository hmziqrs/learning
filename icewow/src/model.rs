use iced::Point;

pub type FolderId = u64;
pub type RequestId = u64;
pub type TabId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl HttpMethod {
    pub const ALL: [HttpMethod; 5] = [
        HttpMethod::Get,
        HttpMethod::Post,
        HttpMethod::Put,
        HttpMethod::Delete,
        HttpMethod::Patch,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponseData {
    pub status_code: u16,
    pub body: String,
    pub elapsed_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeNode {
    Folder(FolderNode),
    Request(RequestNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderNode {
    pub id: FolderId,
    pub name: String,
    pub expanded: bool,
    pub children: Vec<TreeNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestNode {
    pub id: RequestId,
    pub name: String,
    pub url: String,
    pub method: HttpMethod,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tab {
    pub id: TabId,
    pub request_id: Option<RequestId>,
    pub title: String,
    pub url_input: String,
    pub method: HttpMethod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragKind {
    Folder(FolderId),
    Request(RequestId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarDropTarget {
    Before {
        parent: Option<FolderId>,
        index: usize,
    },
    After {
        parent: Option<FolderId>,
        index: usize,
    },
    InsideFolder {
        folder_id: FolderId,
        index: usize,
    },
}

impl SidebarDropTarget {
    pub fn parent(self) -> Option<FolderId> {
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
        source_parent: Option<FolderId>,
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
    Folder(FolderId),
    Request(RequestId),
    Tab(TabId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextMenuTarget {
    ProjectRoot,
    Folder(FolderId),
    Request(RequestId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressKind {
    Sidebar {
        kind: DragKind,
        source_parent: Option<FolderId>,
        source_index: usize,
    },
    Tab {
        tab_id: TabId,
        source_index: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickAction {
    SelectRequest(RequestId),
    SelectTab(TabId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PendingLongPress {
    pub token: u64,
    pub kind: PressKind,
    pub click_action: Option<ClickAction>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub project_name: String,
    pub tree_root: Vec<TreeNode>,
    pub tabs: Vec<Tab>,
    pub active_tab: Option<TabId>,
    pub url_input: String,
    pub drag_state: Option<DragState>,
    pub pending_long_press: Option<PendingLongPress>,
    pub open_context_menu: Option<ContextMenuTarget>,
    pub context_menu_position: Option<Point>,
    pub delete_dialog: Option<DeleteDialog>,
    pub pointer_position: Point,
    pub window_size: iced::Size,
    pub next_press_token: u64,
    pub next_folder_id: FolderId,
    pub next_request_id: RequestId,
    pub next_tab_id: TabId,
    pub response: Option<ResponseData>,
    pub loading: bool,
}

impl AppState {
    pub fn sample() -> Self {
        let mut state = Self {
            project_name: "Workspace Project".to_string(),
            tree_root: vec![],
            tabs: vec![],
            active_tab: None,
            url_input: String::new(),
            drag_state: None,
            pending_long_press: None,
            open_context_menu: None,
            context_menu_position: None,
            delete_dialog: None,
            pointer_position: Point::new(0.0, 0.0),
            window_size: iced::Size::new(1300.0, 820.0),
            next_press_token: 1,
            next_folder_id: 1,
            next_request_id: 1,
            next_tab_id: 1,
            response: None,
            loading: false,
        };

        let users_folder = FolderNode {
            id: state.alloc_folder_id(),
            name: "Users API".to_string(),
            expanded: true,
            children: vec![
                TreeNode::Request(RequestNode {
                    id: state.alloc_request_id(),
                    name: "Get Users".to_string(),
                    url: "https://api.example.com/users".to_string(),
                    method: HttpMethod::Get,
                }),
                TreeNode::Request(RequestNode {
                    id: state.alloc_request_id(),
                    name: "Create User".to_string(),
                    url: "https://api.example.com/users".to_string(),
                    method: HttpMethod::Post,
                }),
                TreeNode::Folder(FolderNode {
                    id: state.alloc_folder_id(),
                    name: "Admin".to_string(),
                    expanded: true,
                    children: vec![TreeNode::Request(RequestNode {
                        id: state.alloc_request_id(),
                        name: "Delete User".to_string(),
                        url: "https://api.example.com/admin/users/1".to_string(),
                        method: HttpMethod::Delete,
                    })],
                }),
            ],
        };

        let catalog_folder = FolderNode {
            id: state.alloc_folder_id(),
            name: "Catalog".to_string(),
            expanded: true,
            children: vec![TreeNode::Request(RequestNode {
                id: state.alloc_request_id(),
                name: "List Products".to_string(),
                url: "https://api.example.com/products".to_string(),
                method: HttpMethod::Get,
            })],
        };

        state.tree_root = vec![
            TreeNode::Folder(users_folder),
            TreeNode::Folder(catalog_folder),
            TreeNode::Request(RequestNode {
                id: state.alloc_request_id(),
                name: "Health Check".to_string(),
                url: "https://api.example.com/health".to_string(),
                method: HttpMethod::Get,
            }),
        ];

        let get_users = state
            .find_request(1)
            .map(|r| (r.id, r.name.clone(), r.url.clone(), r.method))
            .unwrap_or((
                0,
                "Get Users".to_string(),
                "https://api.example.com/users".to_string(),
                HttpMethod::Get,
            ));

        let list_products = state
            .find_request(4)
            .map(|r| (r.id, r.name.clone(), r.url.clone(), r.method))
            .unwrap_or((
                0,
                "List Products".to_string(),
                "https://api.example.com/products".to_string(),
                HttpMethod::Get,
            ));

        let tab_a = Tab {
            id: state.alloc_tab_id(),
            request_id: Some(get_users.0),
            title: get_users.1,
            url_input: get_users.2,
            method: get_users.3,
        };
        let tab_b = Tab {
            id: state.alloc_tab_id(),
            request_id: Some(list_products.0),
            title: list_products.1,
            url_input: list_products.2,
            method: list_products.3,
        };

        state.tabs = vec![tab_a, tab_b];
        state.active_tab = state.tabs.first().map(|tab| tab.id);
        state.sync_url_input_from_active_tab();

        state
    }

    pub fn alloc_folder_id(&mut self) -> FolderId {
        let id = self.next_folder_id;
        self.next_folder_id += 1;
        id
    }

    pub fn alloc_request_id(&mut self) -> RequestId {
        let id = self.next_request_id;
        self.next_request_id += 1;
        id
    }

    pub fn alloc_tab_id(&mut self) -> TabId {
        let id = self.next_tab_id;
        self.next_tab_id += 1;
        id
    }

    pub fn alloc_press_token(&mut self) -> u64 {
        let token = self.next_press_token;
        self.next_press_token += 1;
        token
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        let active = self.active_tab?;
        self.tabs.iter_mut().find(|tab| tab.id == active)
    }

    pub fn active_tab_ref(&self) -> Option<&Tab> {
        let active = self.active_tab?;
        self.tabs.iter().find(|tab| tab.id == active)
    }

    pub fn sync_url_input_from_active_tab(&mut self) {
        self.url_input = self
            .active_tab_ref()
            .map(|tab| tab.url_input.clone())
            .unwrap_or_default();
    }

    pub fn fallback_active_tab(&mut self) {
        if let Some(active) = self.active_tab {
            if self.tabs.iter().any(|tab| tab.id == active) {
                return;
            }
        }

        self.active_tab = self.tabs.first().map(|tab| tab.id);
        self.sync_url_input_from_active_tab();
    }

    pub fn find_request(&self, request_id: RequestId) -> Option<&RequestNode> {
        fn search(nodes: &[TreeNode], request_id: RequestId) -> Option<&RequestNode> {
            for node in nodes {
                match node {
                    TreeNode::Request(request) if request.id == request_id => return Some(request),
                    TreeNode::Folder(folder) => {
                        if let Some(found) = search(&folder.children, request_id) {
                            return Some(found);
                        }
                    }
                    TreeNode::Request(_) => {}
                }
            }
            None
        }

        search(&self.tree_root, request_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeRef {
    Folder(FolderId),
    Request(RequestId),
}
