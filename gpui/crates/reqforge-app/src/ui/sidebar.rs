//! Sidebar panel component - displays collection tree.
//!
//! This is a stub implementation that demonstrates the component structure
//! without requiring GPUI rendering.

use reqforge_core::models::{collection::Collection, request::HttpMethod};
use std::collections::HashSet;
use uuid::Uuid;

/// Represents a node in the collection tree.
#[derive(Debug, Clone)]
pub enum TreeNode {
    Folder {
        id: Uuid,
        name: String,
        expanded: bool,
        children: Vec<TreeNode>,
    },
    Request {
        id: Uuid,
        name: String,
        method: HttpMethod,
        url: String,
    },
}

/// Sidebar panel component.
///
/// Displays the hierarchical collection tree with folders and requests.
/// Supports expand/collapse for folders and actions for creating new items.
pub struct SidebarPanel {
    /// Root nodes of the collection tree
    pub root_nodes: Vec<TreeNode>,
    /// Set of expanded folder IDs
    pub expanded_folders: HashSet<Uuid>,
    /// Currently selected item ID
    pub selected_id: Option<Uuid>,
    /// Hover state for UI feedback
    pub hovered_id: Option<Uuid>,
}

impl SidebarPanel {
    /// Create a new sidebar panel.
    pub fn new() -> Self {
        Self {
            root_nodes: Vec::new(),
            expanded_folders: HashSet::new(),
            selected_id: None,
            hovered_id: None,
        }
    }

    /// Load collections into the sidebar tree.
    pub fn load_collections(&mut self, collections: &[Collection]) {
        self.root_nodes.clear();
        for collection in collections {
            let mut children = Vec::new();

            // Convert requests to tree nodes
            for (id, request) in &collection.requests {
                children.push(TreeNode::Request {
                    id: *id,
                    name: request.name.clone(),
                    method: request.method.clone(),
                    url: request.url.clone(),
                });
            }

            self.root_nodes.push(TreeNode::Folder {
                id: collection.id,
                name: collection.name.clone(),
                expanded: self.expanded_folders.contains(&collection.id),
                children,
            });
        }
    }

    /// Render the sidebar to console (stub implementation).
    pub fn render(&self) {
        println!();
        println!("┌─────────────────────────────────────┐");
        println!("│          Collections               │");
        println!("├─────────────────────────────────────┤");

        if self.root_nodes.is_empty() {
            println!("│  (No collections)                  │");
        } else {
            self.render_nodes(&self.root_nodes, 0);
        }

        println!("├─────────────────────────────────────┤");
        println!("│ [+] New Request  [N] New Folder     │");
        println!("└─────────────────────────────────────┘");
    }

    /// Recursively render tree nodes with indentation.
    fn render_nodes(&self, nodes: &[TreeNode], depth: usize) {
        let indent = "  ".repeat(depth);

        for node in nodes {
            match node {
                TreeNode::Folder { id, name, expanded: _, children } => {
                    let icon = if self.expanded_folders.contains(id) {
                        "▼"
                    } else {
                        "▶"
                    };
                    let marker = if self.selected_id == Some(*id) {
                        "►"
                    } else {
                        " "
                    };

                    println!("│ {}{} {} 📁 {}", indent, marker, icon, name);

                    if self.expanded_folders.contains(id) && !children.is_empty() {
                        self.render_nodes(children, depth + 1);
                    }
                }
                TreeNode::Request { id, name, method, url } => {
                    let marker = if self.selected_id == Some(*id) {
                        "►"
                    } else {
                        " "
                    };
                    let method_str = format!("{:6}", method.to_string());
                    println!("│ {}{} {} {} {}", indent, marker, method_str, name, url);
                }
            }
        }
    }

    /// Toggle folder expansion state.
    pub fn toggle_folder(&mut self, folder_id: Uuid) -> bool {
        if self.expanded_folders.contains(&folder_id) {
            self.expanded_folders.remove(&folder_id);
            false
        } else {
            self.expanded_folders.insert(folder_id);
            true
        }
    }

    /// Set the selected item.
    pub fn set_selected(&mut self, id: Option<Uuid>) {
        self.selected_id = id;
    }

    /// Handle new request action.
    pub fn on_new_request(&self, collection_id: Uuid) {
        println!();
        println!("➕ Action: Create new request in collection {}", collection_id);
        println!("   → Would open request creation dialog");
    }

    /// Handle new folder action.
    pub fn on_new_folder(&self) {
        println!();
        println!("📁 Action: Create new collection/folder");
        println!("   → Would open folder creation dialog");
    }

    /// Handle item selection.
    pub fn on_select(&self, id: Uuid) {
        println!();
        println!("✓ Selected item: {}", id);
        println!("   → Would update active tab or preview");
    }

    /// Handle double-click to open.
    pub fn on_double_click(&self, id: Uuid) {
        println!();
        println!("⚡ Action: Open item {} in new tab", id);
        println!("   → Would create new tab and switch to it");
    }

    /// Find a node by ID recursively.
    pub fn find_node(&self, id: Uuid) -> Option<&TreeNode> {
        self.find_node_in_nodes(&self.root_nodes, id)
    }

    fn find_node_in_nodes<'a>(&self, nodes: &'a [TreeNode], id: Uuid) -> Option<&'a TreeNode> {
        for node in nodes {
            match node {
                TreeNode::Folder { id: node_id, children, .. } => {
                    if *node_id == id {
                        return Some(node);
                    }
                    if let Some(found) = self.find_node_in_nodes(children, id) {
                        return Some(found);
                    }
                }
                TreeNode::Request { id: node_id, .. } => {
                    if *node_id == id {
                        return Some(node);
                    }
                }
            }
        }
        None
    }

    /// Expand all folders.
    pub fn expand_all(&mut self) {
        for node in &self.root_nodes {
            if let TreeNode::Folder { id, .. } = node {
                self.expanded_folders.insert(*id);
            }
        }
    }

    /// Collapse all folders.
    pub fn collapse_all(&mut self) {
        self.expanded_folders.clear();
    }
}

impl Default for SidebarPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sidebar_creation() {
        let sidebar = SidebarPanel::new();
        assert!(sidebar.root_nodes.is_empty());
        assert!(sidebar.expanded_folders.is_empty());
    }

    #[test]
    fn test_toggle_folder() {
        let mut sidebar = SidebarPanel::new();
        let id = Uuid::new_v4();

        // First toggle should expand
        assert!(sidebar.toggle_folder(id));
        assert!(sidebar.expanded_folders.contains(&id));

        // Second toggle should collapse
        assert!(!sidebar.toggle_folder(id));
        assert!(!sidebar.expanded_folders.contains(&id));
    }

    #[test]
    fn test_selection() {
        let mut sidebar = SidebarPanel::new();
        let id = Uuid::new_v4();

        sidebar.set_selected(Some(id));
        assert_eq!(sidebar.selected_id, Some(id));

        sidebar.set_selected(None);
        assert_eq!(sidebar.selected_id, None);
    }
}
