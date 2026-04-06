use std::collections::HashMap;

use icewow_engine::HttpMethod;

use crate::model::SidebarDropTarget;

/// Unified ID for both folders and requests in the tree.
pub type NodeId = u64;

/// Maximum nesting depth for folders in the tree.
pub const MAX_DEPTH: usize = 32;

/// The flat, HashMap-based tree that replaces recursive `Vec<TreeNode>`.
///
/// All lookups are O(1). Ancestor checks are O(depth) via parent back-pointers.
#[derive(Debug, Clone)]
pub struct TreeArena {
    nodes: HashMap<NodeId, TreeEntry>,
    root_children: Vec<NodeId>,
    next_id: NodeId,
}

#[derive(Debug, Clone)]
pub struct TreeEntry {
    pub data: NodeData,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeData {
    Folder { name: String, expanded: bool },
    Request { name: String, url: String, method: HttpMethod },
}

impl TreeArena {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root_children: Vec::new(),
            next_id: 1,
        }
    }

    fn alloc_id(&mut self) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn alloc_folder_id(&mut self) -> NodeId {
        self.alloc_id()
    }

    pub fn alloc_request_id(&mut self) -> NodeId {
        self.alloc_id()
    }

    // ── Read operations (all O(1)) ─────────────────────────────

    pub fn get(&self, id: NodeId) -> Option<&TreeEntry> {
        self.nodes.get(&id)
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut TreeEntry> {
        self.nodes.get_mut(&id)
    }

    #[allow(dead_code)]
    pub fn contains(&self, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    pub fn root_children(&self) -> &[NodeId] {
        &self.root_children
    }

    #[allow(dead_code)]
    pub fn children_of(&self, id: NodeId) -> Option<&[NodeId]> {
        self.nodes.get(&id).map(|e| e.children.as_slice())
    }

    pub fn folder_name(&self, id: NodeId) -> Option<&str> {
        match &self.nodes.get(&id)?.data {
            NodeData::Folder { name, .. } => Some(name),
            _ => None,
        }
    }

    pub fn is_folder(&self, id: NodeId) -> bool {
        matches!(self.nodes.get(&id).map(|e| &e.data), Some(NodeData::Folder { .. }))
    }

    pub fn is_expanded(&self, id: NodeId) -> Option<bool> {
        match &self.nodes.get(&id)?.data {
            NodeData::Folder { expanded, .. } => Some(*expanded),
            _ => None,
        }
    }

    pub fn children_len(&self, parent: Option<NodeId>) -> usize {
        match parent {
            None => self.root_children.len(),
            Some(id) => self.nodes.get(&id).map(|e| e.children.len()).unwrap_or(0),
        }
    }

    pub fn find_parent_and_index(&self, id: NodeId) -> Option<(Option<NodeId>, usize)> {
        let entry = self.nodes.get(&id)?;
        let parent = entry.parent;
        let siblings: &[NodeId] = match parent {
            None => &self.root_children,
            Some(pid) => &self.nodes.get(&pid)?.children,
        };
        let index = siblings.iter().position(|&sid| sid == id)?;
        Some((parent, index))
    }

    // ── Write operations ───────────────────────────────────────

    /// Returns the depth of `id` (root children = depth 0).
    pub fn depth_of(&self, id: NodeId) -> Option<usize> {
        let mut depth = 0;
        let mut current = id;
        loop {
            let entry = self.nodes.get(&current)?;
            match entry.parent {
                Some(pid) => {
                    depth += 1;
                    current = pid;
                }
                None => return Some(depth),
            }
        }
    }

    /// Insert a node. Returns `false` if inserting as a child of `parent`
    /// would exceed `MAX_DEPTH`.
    pub fn insert(&mut self, parent: Option<NodeId>, index: usize, id: NodeId, data: NodeData) -> bool {
        // Depth check: parent at depth D means child would be at D+1
        if let Some(pid) = parent {
            let parent_depth = self.depth_of(pid).unwrap_or(0);
            if parent_depth + 1 >= MAX_DEPTH {
                return false;
            }
        }

        let entry = TreeEntry {
            data,
            parent,
            children: Vec::new(),
        };
        self.nodes.insert(id, entry);

        match parent {
            None => {
                let idx = index.min(self.root_children.len());
                self.root_children.insert(idx, id);
            }
            Some(pid) => {
                if let Some(parent_entry) = self.nodes.get_mut(&pid) {
                    let idx = index.min(parent_entry.children.len());
                    parent_entry.children.insert(idx, id);
                }
            }
        }

        true
    }

    pub fn remove(&mut self, id: NodeId) -> bool {
        if !self.nodes.contains_key(&id) {
            return false;
        }

        // Collect all descendants
        let mut to_remove = vec![id];
        let mut stack = vec![id];
        while let Some(nid) = stack.pop() {
            if let Some(entry) = self.nodes.get(&nid) {
                for &child_id in &entry.children {
                    stack.push(child_id);
                    to_remove.push(child_id);
                }
            }
        }

        // Remove from parent's children list
        if let Some(entry) = self.nodes.get(&id) {
            if let Some(parent_id) = entry.parent {
                if let Some(parent_entry) = self.nodes.get_mut(&parent_id) {
                    parent_entry.children.retain(|&cid| cid != id);
                }
            } else {
                self.root_children.retain(|&cid| cid != id);
            }
        }

        for nid in to_remove {
            self.nodes.remove(&nid);
        }

        true
    }

    pub fn set_expanded(&mut self, id: NodeId, expanded: bool) -> bool {
        if let Some(entry) = self.nodes.get_mut(&id) {
            if let NodeData::Folder { expanded: ref mut e, .. } = entry.data {
                *e = expanded;
                return true;
            }
        }
        false
    }

    pub fn update_request_from_draft(
        &mut self,
        id: NodeId,
        name: &str,
        url: &str,
        method: HttpMethod,
    ) -> bool {
        if let Some(entry) = self.nodes.get_mut(&id) {
            if let NodeData::Request { name: ref mut n, url: ref mut u, method: ref mut m } = entry.data {
                *n = name.to_string();
                *u = url.to_string();
                *m = method;
                return true;
            }
        }
        false
    }

    /// Move a node to a new position determined by a SidebarDropTarget.
    pub fn move_node(&mut self, source: NodeId, target: SidebarDropTarget) -> bool {
        if !self.nodes.contains_key(&source) {
            return false;
        }

        // Prevent moving a folder into itself or its descendants
        if self.is_folder(source) {
            let target_parent = target.parent();
            if target_parent == Some(source) {
                return false;
            }
            if let Some(tp) = target_parent {
                if self.is_ancestor(source, tp) {
                    return false;
                }
            }
        }

        // Get source position info before mutation
        let (source_parent, source_index) = match self.find_parent_and_index(source) {
            Some(pos) => pos,
            None => return false,
        };

        // Remove from current parent
        match source_parent {
            None => { self.root_children.remove(source_index); }
            Some(pid) => {
                if let Some(parent) = self.nodes.get_mut(&pid) {
                    parent.children.remove(source_index);
                } else {
                    return false;
                }
            }
        };

        // Determine target parent and index
        let (target_parent, mut target_index) = match target {
            SidebarDropTarget::Before { parent, index } => (parent, index),
            SidebarDropTarget::After { parent, index } => (parent, index.saturating_add(1)),
            SidebarDropTarget::InsideFolder { folder_id, index } => (Some(folder_id), index),
        };

        // Adjust index for same-parent moves
        if source_parent == target_parent && source_index < target_index {
            target_index = target_index.saturating_sub(1);
        }

        // Update source entry's parent
        if let Some(entry) = self.nodes.get_mut(&source) {
            entry.parent = target_parent;
        }

        // Insert into target parent
        match target_parent {
            None => {
                let idx = target_index.min(self.root_children.len());
                self.root_children.insert(idx, source);
            }
            Some(pid) => {
                if let Some(parent_entry) = self.nodes.get_mut(&pid) {
                    let idx = target_index.min(parent_entry.children.len());
                    parent_entry.children.insert(idx, source);
                }
            }
        }

        true
    }

    /// O(depth) ancestor check using parent pointers.
    pub fn is_ancestor(&self, ancestor: NodeId, descendant: NodeId) -> bool {
        if ancestor == descendant {
            return true;
        }
        let mut current = descendant;
        while let Some(entry) = self.nodes.get(&current) {
            match entry.parent {
                Some(parent) => {
                    if parent == ancestor {
                        return true;
                    }
                    current = parent;
                }
                None => return false,
            }
        }
        false
    }

    /// Collect all request IDs in the subtree rooted at `id`.
    pub fn collect_request_ids(&self, id: NodeId) -> Vec<NodeId> {
        let mut result = Vec::new();
        let mut stack = vec![id];
        while let Some(nid) = stack.pop() {
            if let Some(entry) = self.nodes.get(&nid) {
                match &entry.data {
                    NodeData::Request { .. } => result.push(nid),
                    NodeData::Folder { .. } => {
                        stack.extend(entry.children.iter().copied());
                    }
                }
            }
        }
        result
    }

    #[allow(dead_code)]
    pub fn set_name(&mut self, id: NodeId, name: String) -> bool {
        if let Some(entry) = self.nodes.get_mut(&id) {
            match &mut entry.data {
                NodeData::Folder { name: ref mut n, .. } => *n = name,
                NodeData::Request { name: ref mut n, .. } => *n = name,
            }
            return true;
        }
        false
    }

    // ── Sample data ────────────────────────────────────────────

    pub fn build_sample(&mut self) -> (NodeId, NodeId, NodeId, NodeId, NodeId) {
        let users_folder = self.alloc_folder_id();
        self.insert(None, 0, users_folder, NodeData::Folder {
            name: "Users API".to_string(),
            expanded: true,
        });

        let admin_subfolder = self.alloc_folder_id();
        self.insert(Some(users_folder), 2, admin_subfolder, NodeData::Folder {
            name: "Admin".to_string(),
            expanded: true,
        });

        let catalog_folder = self.alloc_folder_id();
        self.insert(None, 1, catalog_folder, NodeData::Folder {
            name: "Catalog".to_string(),
            expanded: true,
        });

        let empty_folder = self.alloc_folder_id();
        self.insert(None, 3, empty_folder, NodeData::Folder {
            name: "New Folder".to_string(),
            expanded: true,
        });

        let get_users = self.alloc_request_id();
        self.insert(Some(users_folder), 0, get_users, NodeData::Request {
            name: "Get Users".to_string(),
            url: "https://api.example.com/users".to_string(),
            method: HttpMethod::Get,
        });

        let create_user = self.alloc_request_id();
        self.insert(Some(users_folder), 1, create_user, NodeData::Request {
            name: "Create User".to_string(),
            url: "https://api.example.com/users".to_string(),
            method: HttpMethod::Post,
        });

        let delete_user = self.alloc_request_id();
        self.insert(Some(admin_subfolder), 0, delete_user, NodeData::Request {
            name: "Delete User".to_string(),
            url: "https://api.example.com/admin/users/1".to_string(),
            method: HttpMethod::Delete,
        });

        let list_products = self.alloc_request_id();
        self.insert(Some(catalog_folder), 0, list_products, NodeData::Request {
            name: "List Products".to_string(),
            url: "https://api.example.com/products".to_string(),
            method: HttpMethod::Get,
        });

        let health_check = self.alloc_request_id();
        self.insert(None, 2, health_check, NodeData::Request {
            name: "Health Check".to_string(),
            url: "https://api.example.com/health".to_string(),
            method: HttpMethod::Get,
        });

        (get_users, create_user, list_products, admin_subfolder, health_check)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> (TreeArena, NodeId, NodeId, NodeId, NodeId, NodeId) {
        let mut tree = TreeArena::new();

        let folder_a = tree.alloc_folder_id();
        tree.insert(None, 0, folder_a, NodeData::Folder {
            name: "A".to_string(),
            expanded: true,
        });

        let r1 = tree.alloc_request_id();
        tree.insert(Some(folder_a), 0, r1, NodeData::Request {
            name: "r1".to_string(),
            url: "u1".to_string(),
            method: HttpMethod::Get,
        });
        let r2 = tree.alloc_request_id();
        tree.insert(Some(folder_a), 1, r2, NodeData::Request {
            name: "r2".to_string(),
            url: "u2".to_string(),
            method: HttpMethod::Get,
        });

        let subfolder = tree.alloc_folder_id();
        tree.insert(Some(folder_a), 2, subfolder, NodeData::Folder {
            name: "A-Child".to_string(),
            expanded: true,
        });
        let r3 = tree.alloc_request_id();
        tree.insert(Some(subfolder), 0, r3, NodeData::Request {
            name: "r3".to_string(),
            url: "u3".to_string(),
            method: HttpMethod::Get,
        });

        let folder_b = tree.alloc_folder_id();
        tree.insert(None, 1, folder_b, NodeData::Folder {
            name: "B".to_string(),
            expanded: true,
        });

        (tree, folder_a, folder_b, subfolder, r1, r2)
    }

    #[test]
    fn move_request_within_same_folder() {
        let (mut tree, folder_a, _, _, r1, _) = sample_tree();

        let moved = tree.move_node(r1, SidebarDropTarget::After {
            parent: Some(folder_a),
            index: 1,
        });
        assert!(moved);

        let children = tree.children_of(folder_a).unwrap().to_vec();
        assert_eq!(children.len(), 3);
        assert_eq!(children[1], r1);
    }

    #[test]
    fn move_request_across_folders() {
        let (mut tree, _, folder_b, _, _, r2) = sample_tree();

        let moved = tree.move_node(r2, SidebarDropTarget::InsideFolder {
            folder_id: folder_b,
            index: 0,
        });
        assert!(moved);

        let (parent, index) = tree.find_parent_and_index(r2).unwrap();
        assert_eq!(parent, Some(folder_b));
        assert_eq!(index, 0);
    }

    #[test]
    fn move_folder_across_branches() {
        let (mut tree, _, folder_b, subfolder, _, _) = sample_tree();

        let moved = tree.move_node(subfolder, SidebarDropTarget::InsideFolder {
            folder_id: folder_b,
            index: 0,
        });
        assert!(moved);

        let (parent, index) = tree.find_parent_and_index(subfolder).unwrap();
        assert_eq!(parent, Some(folder_b));
        assert_eq!(index, 0);
    }

    #[test]
    fn reject_move_folder_into_descendant() {
        let (mut tree, folder_a, _, _, _, _) = sample_tree();

        let children = tree.children_of(folder_a).unwrap();
        let subfolder_id = children[2];

        let moved = tree.move_node(folder_a, SidebarDropTarget::InsideFolder {
            folder_id: subfolder_id,
            index: 0,
        });
        assert!(!moved);
        assert_eq!(tree.find_parent_and_index(folder_a), Some((None, 0)));
    }

    #[test]
    fn delete_folder_removes_subtree() {
        let (mut tree, folder_a, _, _, r1, _) = sample_tree();

        let removed = tree.remove(folder_a);
        assert!(removed);
        assert!(!tree.contains(r1));
        assert!(tree.children_of(folder_a).is_none());
    }

    #[test]
    fn delete_request_removes_only_target() {
        let (mut tree, _, _, _, r1, r2) = sample_tree();

        let removed = tree.remove(r1);
        assert!(removed);
        assert!(!tree.contains(r1));
        assert!(tree.contains(r2));
    }

    #[test]
    fn is_ancestor_walks_parent_pointers() {
        let (tree, folder_a, _, subfolder, _, _) = sample_tree();

        assert!(tree.is_ancestor(folder_a, subfolder));
        assert!(!tree.is_ancestor(subfolder, folder_a));
        assert!(tree.is_ancestor(folder_a, folder_a));
    }

    #[test]
    fn collect_request_ids_gathers_subtree() {
        let (tree, folder_a, _, _, r1, r2) = sample_tree();

        let ids = tree.collect_request_ids(folder_a);
        assert!(ids.contains(&r1));
        assert!(ids.contains(&r2));
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn insert_rejects_exceeding_max_depth() {
        let mut tree = TreeArena::new();
        let mut current_parent = None;
        let mut last_folder = None;

        for i in 0..MAX_DEPTH {
            let id = tree.alloc_folder_id();
            let ok = tree.insert(current_parent, 0, id, NodeData::Folder {
                name: format!("d{i}"),
                expanded: true,
            });
            assert!(ok, "insert at depth {i} should succeed");
            current_parent = Some(id);
            last_folder = Some(id);
        }

        // One more should fail
        let extra = tree.alloc_folder_id();
        let ok = tree.insert(last_folder, 0, extra, NodeData::Folder {
            name: "too-deep".to_string(),
            expanded: true,
        });
        assert!(!ok, "insert at depth MAX_DEPTH should be rejected");
    }

    #[test]
    fn depth_of_returns_correct_depth() {
        let (tree, folder_a, _, subfolder, _, _) = sample_tree();

        assert_eq!(tree.depth_of(folder_a), Some(0));
        assert_eq!(tree.depth_of(subfolder), Some(1));
    }
}
