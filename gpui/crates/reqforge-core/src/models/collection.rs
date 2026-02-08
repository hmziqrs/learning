use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::folder::CollectionItem;
use super::request::RequestDefinition;
use std::collections::HashMap;

/// A Collection owns an ordered tree of folders/requests
/// and a lookup table for the actual RequestDefinition objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub tree: Vec<CollectionItem>,
    pub requests: HashMap<Uuid, RequestDefinition>,
}

impl Collection {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            tree: Vec::new(),
            requests: HashMap::new(),
        }
    }

    pub fn add_request(&mut self, req: RequestDefinition, parent_folder: Option<Uuid>) {
        let id = req.id;
        self.requests.insert(id, req);
        let item = CollectionItem::Request(id);
        match parent_folder {
            Some(folder_id) => {
                Self::insert_into_folder(&mut self.tree, folder_id, item);
            }
            None => self.tree.push(item),
        }
    }

    fn insert_into_folder(items: &mut Vec<CollectionItem>, folder_id: Uuid, new_item: CollectionItem) -> bool {
        for item in items.iter_mut() {
            if let CollectionItem::Folder(folder) = item {
                if folder.id == folder_id {
                    folder.children.push(new_item);
                    return true;
                }
                if Self::insert_into_folder(&mut folder.children, folder_id, new_item.clone()) {
                    return true;
                }
            }
        }
        false
    }
}
