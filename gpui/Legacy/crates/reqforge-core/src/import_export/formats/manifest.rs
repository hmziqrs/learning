//! Workspace manifest for full exports

use serde::{Deserialize, Serialize};

/// Workspace manifest for full exports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceManifest {
    pub version: String,
    pub exported_at: chrono::DateTime<chrono::Utc>,
    pub collection_count: usize,
    pub environment_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_manifest_serialization() {
        let manifest = WorkspaceManifest {
            version: "1.0".to_string(),
            exported_at: Utc::now(),
            collection_count: 2,
            environment_count: 1,
        };

        let json = serde_json::to_string(&manifest).unwrap();
        assert!(json.contains("version"));
        assert!(json.contains("collection_count"));

        let deserialized: WorkspaceManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(manifest.version, deserialized.version);
        assert_eq!(manifest.collection_count, deserialized.collection_count);
    }
}
