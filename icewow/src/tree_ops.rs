use crate::model::{FolderId, NodeRef, RequestId, SidebarDropTarget, TreeNode};

pub fn insert_node(
    nodes: &mut Vec<TreeNode>,
    parent: Option<FolderId>,
    index: usize,
    node: TreeNode,
) -> bool {
    match parent {
        None => {
            let idx = index.min(nodes.len());
            nodes.insert(idx, node);
            true
        }
        Some(parent_id) => insert_into_folder(nodes, parent_id, index, node),
    }
}

pub fn remove_folder(nodes: &mut Vec<TreeNode>, folder_id: FolderId) -> Option<TreeNode> {
    remove_matching(
        nodes,
        &|node| matches!(node, TreeNode::Folder(folder) if folder.id == folder_id),
    )
}

pub fn remove_request(nodes: &mut Vec<TreeNode>, request_id: RequestId) -> Option<TreeNode> {
    remove_matching(
        nodes,
        &|node| matches!(node, TreeNode::Request(request) if request.id == request_id),
    )
}

pub fn is_descendant(
    nodes: &[TreeNode],
    folder_id: FolderId,
    potential_child_folder_id: FolderId,
) -> bool {
    fn find_folder(nodes: &[TreeNode], folder_id: FolderId) -> Option<&[TreeNode]> {
        for node in nodes {
            if let TreeNode::Folder(folder) = node {
                if folder.id == folder_id {
                    return Some(&folder.children);
                }
                if let Some(found) = find_folder(&folder.children, folder_id) {
                    return Some(found);
                }
            }
        }
        None
    }

    fn contains_folder(nodes: &[TreeNode], folder_id: FolderId) -> bool {
        for node in nodes {
            if let TreeNode::Folder(folder) = node {
                if folder.id == folder_id || contains_folder(&folder.children, folder_id) {
                    return true;
                }
            }
        }
        false
    }

    find_folder(nodes, folder_id)
        .map(|children| contains_folder(children, potential_child_folder_id))
        .unwrap_or(false)
}

#[allow(dead_code)]
pub fn find_parent_and_index(
    nodes: &[TreeNode],
    node_ref: NodeRef,
) -> Option<(Option<FolderId>, usize)> {
    fn recurse(
        nodes: &[TreeNode],
        parent: Option<FolderId>,
        node_ref: NodeRef,
    ) -> Option<(Option<FolderId>, usize)> {
        for (index, node) in nodes.iter().enumerate() {
            match (node_ref, node) {
                (NodeRef::Folder(folder_id), TreeNode::Folder(folder))
                    if folder.id == folder_id =>
                {
                    return Some((parent, index));
                }
                (NodeRef::Request(request_id), TreeNode::Request(request))
                    if request.id == request_id =>
                {
                    return Some((parent, index));
                }
                (_, TreeNode::Folder(folder)) => {
                    if let Some(found) = recurse(&folder.children, Some(folder.id), node_ref) {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }
        None
    }

    recurse(nodes, None, node_ref)
}

pub fn move_node(
    nodes: &mut Vec<TreeNode>,
    source: NodeRef,
    source_parent: Option<FolderId>,
    source_index: usize,
    target: SidebarDropTarget,
) -> bool {
    let moved_node = match source {
        NodeRef::Folder(folder_id) => {
            let target_parent = target.parent();

            if target_parent == Some(folder_id) {
                return false;
            }

            if let Some(parent) = target_parent {
                if is_descendant(nodes, folder_id, parent) {
                    return false;
                }
            }

            remove_folder(nodes, folder_id)
        }
        NodeRef::Request(request_id) => remove_request(nodes, request_id),
    };

    let Some(node) = moved_node else {
        return false;
    };

    let (target_parent, mut target_index) = match target {
        SidebarDropTarget::Before { parent, index } => (parent, index),
        SidebarDropTarget::After { parent, index } => (parent, index.saturating_add(1)),
        SidebarDropTarget::InsideFolder { folder_id, index } => (Some(folder_id), index),
    };

    if source_parent == target_parent && source_index < target_index {
        target_index = target_index.saturating_sub(1);
    }

    insert_node(nodes, target_parent, target_index, node)
}

pub fn collect_request_ids(node: &TreeNode, out: &mut Vec<RequestId>) {
    match node {
        TreeNode::Request(request) => out.push(request.id),
        TreeNode::Folder(folder) => {
            for child in &folder.children {
                collect_request_ids(child, out);
            }
        }
    }
}

pub fn set_folder_expanded(nodes: &mut [TreeNode], folder_id: FolderId, expanded: bool) -> bool {
    for node in nodes {
        if let TreeNode::Folder(folder) = node {
            if folder.id == folder_id {
                folder.expanded = expanded;
                return true;
            }

            if set_folder_expanded(&mut folder.children, folder_id, expanded) {
                return true;
            }
        }
    }

    false
}

fn insert_into_folder(
    nodes: &mut Vec<TreeNode>,
    parent_id: FolderId,
    index: usize,
    node: TreeNode,
) -> bool {
    for current in nodes {
        if let TreeNode::Folder(folder) = current {
            if folder.id == parent_id {
                let idx = index.min(folder.children.len());
                folder.children.insert(idx, node);
                return true;
            }

            if insert_into_folder(&mut folder.children, parent_id, index, node.clone()) {
                return true;
            }
        }
    }

    false
}

fn remove_matching(
    nodes: &mut Vec<TreeNode>,
    predicate: &dyn Fn(&TreeNode) -> bool,
) -> Option<TreeNode> {
    let mut index = 0;
    while index < nodes.len() {
        if predicate(&nodes[index]) {
            return Some(nodes.remove(index));
        }

        if let TreeNode::Folder(folder) = &mut nodes[index] {
            if let Some(removed) = remove_matching(&mut folder.children, predicate) {
                return Some(removed);
            }
        }

        index += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{FolderNode, RequestNode};

    fn sample_tree() -> Vec<TreeNode> {
        vec![
            TreeNode::Folder(FolderNode {
                id: 1,
                name: "A".to_string(),
                expanded: true,
                children: vec![
                    TreeNode::Request(RequestNode {
                        id: 10,
                        name: "r1".to_string(),
                        url: "u1".to_string(),
                    }),
                    TreeNode::Request(RequestNode {
                        id: 11,
                        name: "r2".to_string(),
                        url: "u2".to_string(),
                    }),
                    TreeNode::Folder(FolderNode {
                        id: 3,
                        name: "A-Child".to_string(),
                        expanded: true,
                        children: vec![TreeNode::Request(RequestNode {
                            id: 12,
                            name: "r3".to_string(),
                            url: "u3".to_string(),
                        })],
                    }),
                ],
            }),
            TreeNode::Folder(FolderNode {
                id: 2,
                name: "B".to_string(),
                expanded: true,
                children: vec![],
            }),
        ]
    }

    #[test]
    fn move_request_within_same_folder() {
        let mut tree = sample_tree();
        let moved = move_node(
            &mut tree,
            NodeRef::Request(10),
            Some(1),
            0,
            SidebarDropTarget::After {
                parent: Some(1),
                index: 1,
            },
        );

        assert!(moved);

        let folder = match &tree[0] {
            TreeNode::Folder(folder) => folder,
            _ => panic!("expected folder"),
        };

        let ids: Vec<RequestId> = folder
            .children
            .iter()
            .filter_map(|node| {
                if let TreeNode::Request(request) = node {
                    Some(request.id)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(ids, vec![11, 10]);
    }

    #[test]
    fn move_request_across_folders() {
        let mut tree = sample_tree();
        let moved = move_node(
            &mut tree,
            NodeRef::Request(11),
            Some(1),
            1,
            SidebarDropTarget::InsideFolder {
                folder_id: 2,
                index: 0,
            },
        );

        assert!(moved);
        assert!(find_parent_and_index(&tree, NodeRef::Request(11)).is_some());
        assert_eq!(
            find_parent_and_index(&tree, NodeRef::Request(11)),
            Some((Some(2), 0))
        );
    }

    #[test]
    fn move_folder_across_branches() {
        let mut tree = sample_tree();
        let moved = move_node(
            &mut tree,
            NodeRef::Folder(3),
            Some(1),
            2,
            SidebarDropTarget::InsideFolder {
                folder_id: 2,
                index: 0,
            },
        );

        assert!(moved);
        assert_eq!(
            find_parent_and_index(&tree, NodeRef::Folder(3)),
            Some((Some(2), 0))
        );
    }

    #[test]
    fn reject_move_folder_into_descendant() {
        let mut tree = sample_tree();
        let moved = move_node(
            &mut tree,
            NodeRef::Folder(1),
            None,
            0,
            SidebarDropTarget::InsideFolder {
                folder_id: 3,
                index: 0,
            },
        );

        assert!(!moved);
        assert_eq!(
            find_parent_and_index(&tree, NodeRef::Folder(1)),
            Some((None, 0))
        );
    }

    #[test]
    fn delete_folder_removes_subtree() {
        let mut tree = sample_tree();
        let removed = remove_folder(&mut tree, 1);

        assert!(removed.is_some());
        assert!(find_parent_and_index(&tree, NodeRef::Request(10)).is_none());
        assert!(find_parent_and_index(&tree, NodeRef::Request(12)).is_none());
    }

    #[test]
    fn delete_request_removes_only_target() {
        let mut tree = sample_tree();
        let removed = remove_request(&mut tree, 11);

        assert!(removed.is_some());
        assert!(find_parent_and_index(&tree, NodeRef::Request(11)).is_none());
        assert!(find_parent_and_index(&tree, NodeRef::Request(10)).is_some());
    }

    #[test]
    fn reorder_tabs_keeps_active_tab_identity() {
        let mut tabs = vec![1_u64, 2_u64, 3_u64];
        let active = 2_u64;

        let source = 1;
        let mut target = 3;
        let tab = tabs.remove(source);
        if source < target {
            target -= 1;
        }
        tabs.insert(target, tab);

        assert_eq!(tabs, vec![1, 3, 2]);
        assert!(tabs.contains(&active));
    }
}
