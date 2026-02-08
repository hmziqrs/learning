use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: Uuid,
    pub name: String,
    pub children: Vec<CollectionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectionItem {
    Request(Uuid),              // references RequestDefinition.id
    Folder(Folder),
}
