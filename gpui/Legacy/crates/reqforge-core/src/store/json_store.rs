use std::path::PathBuf;
use crate::models::collection::Collection;
use crate::models::environment::Environment;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

/// Layout on disk:
/// workspace_dir/
///   environments.json      -> Vec<Environment>
///   collections/
///     {collection_id}.json -> Collection
pub struct JsonStore {
    root: PathBuf,
}

impl JsonStore {
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, StoreError> {
        let root = root.into();
        std::fs::create_dir_all(root.join("collections"))?;
        Ok(Self { root })
    }

    // --- Environments ---

    pub fn load_environments(&self) -> Result<Vec<Environment>, StoreError> {
        let path = self.root.join("environments.json");
        if !path.exists() { return Ok(Vec::new()); }
        let data = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save_environments(&self, envs: &[Environment]) -> Result<(), StoreError> {
        let json = serde_json::to_string_pretty(envs)?;
        std::fs::write(self.root.join("environments.json"), json)?;
        Ok(())
    }

    // --- Collections ---

    pub fn list_collections(&self) -> Result<Vec<Collection>, StoreError> {
        let dir = self.root.join("collections");
        let mut collections = Vec::new();
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            if entry.path().extension().map(|e| e == "json").unwrap_or(false) {
                let data = std::fs::read_to_string(entry.path())?;
                collections.push(serde_json::from_str(&data)?);
            }
        }
        Ok(collections)
    }

    pub fn save_collection(&self, col: &Collection) -> Result<(), StoreError> {
        let path = self.root.join("collections").join(format!("{}.json", col.id));
        let json = serde_json::to_string_pretty(col)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn delete_collection(&self, col: &Collection) -> Result<(), StoreError> {
        let path = self.root.join("collections").join(format!("{}.json", col.id));
        if path.exists() { std::fs::remove_file(path)?; }
        Ok(())
    }
}
