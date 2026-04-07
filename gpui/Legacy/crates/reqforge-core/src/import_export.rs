//! Import/export functionality for ReqForge collections and environments.
//!
//! Supports multiple formats:
//! - JSON (native format)
//! - Postman collection export (partial support)
//! - OpenAPI spec import (basic support)

pub mod error;
pub mod formats;

use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use zip::{ZipArchive, ZipWriter};
use zip::write::FileOptions;
use chrono::Utc;
use uuid::Uuid;

use crate::models::{Collection, Environment, RequestDefinition, HttpMethod, KeyValuePair, BodyType, RawContentType, CollectionItem, Folder, Variable};
use error::{ImportError, ExportError, ImportErrorKind, ExportErrorKind};

/// Result type for import operations
pub type ImportResult<T> = Result<T, ImportError>;

/// Result type for export operations
pub type ExportResult<T> = Result<T, ExportError>;

/// Export a collection to a JSON file
pub fn export_collection(collection: &Collection, path: impl AsRef<Path>) -> ExportResult<()> {
    let path = path.as_ref();
    let file = File::create(path)
        .map_err(|e| ExportError::new(ExportErrorKind::Io, &format!("Failed to create file: {}", e)))?;

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, collection)
        .map_err(|e| ExportError::new(ExportErrorKind::Serialization, &format!("Failed to serialize collection: {}", e)))?;

    Ok(())
}

/// Import a collection from a JSON file
pub fn import_collection(path: impl AsRef<Path>) -> ImportResult<Collection> {
    let path = path.as_ref();
    let file = File::open(path)
        .map_err(|e| ImportError::new(ImportErrorKind::Io, &format!("Failed to open file: {}", e)))?;

    let reader = BufReader::new(file);
    let collection: Collection = serde_json::from_reader(reader)
        .map_err(|e| ImportError::new(ImportErrorKind::Deserialization, &format!("Failed to deserialize collection: {}", e)))?;

    validate_collection(&collection)?;
    Ok(collection)
}

/// Import a collection from Postman format
pub fn import_collection_from_postman(path: impl AsRef<Path>) -> ImportResult<Collection> {
    let path = path.as_ref();
    let file = File::open(path)
        .map_err(|e| ImportError::new(ImportErrorKind::Io, &format!("Failed to open Postman file: {}", e)))?;

    let reader = BufReader::new(file);
    let postman: serde_json::Value = serde_json::from_reader(reader)
        .map_err(|e| ImportError::new(ImportErrorKind::Deserialization, &format!("Failed to parse Postman JSON: {}", e)))?;

    formats::postman::parse_postman_collection(&postman)
}

/// Import a collection from OpenAPI specification
pub fn import_collection_from_openapi(path: impl AsRef<Path>) -> ImportResult<Collection> {
    let path = path.as_ref();
    let file = File::open(path)
        .map_err(|e| ImportError::new(ImportErrorKind::Io, &format!("Failed to open OpenAPI file: {}", e)))?;

    let reader = BufReader::new(file);
    let openapi: serde_json::Value = serde_json::from_reader(reader)
        .map_err(|e| ImportError::new(ImportErrorKind::Deserialization, &format!("Failed to parse OpenAPI JSON: {}", e)))?;

    formats::openapi::parse_openapi_spec(&openapi)
}

/// Export an environment to a JSON file
pub fn export_environment(environment: &Environment, path: impl AsRef<Path>) -> ExportResult<()> {
    let path = path.as_ref();
    let file = File::create(path)
        .map_err(|e| ExportError::new(ExportErrorKind::Io, &format!("Failed to create file: {}", e)))?;

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, environment)
        .map_err(|e| ExportError::new(ExportErrorKind::Serialization, &format!("Failed to serialize environment: {}", e)))?;

    Ok(())
}

/// Import an environment from a JSON file
pub fn import_environment(path: impl AsRef<Path>) -> ImportResult<Environment> {
    let path = path.as_ref();
    let file = File::open(path)
        .map_err(|e| ImportError::new(ImportErrorKind::Io, &format!("Failed to open file: {}", e)))?;

    let reader = BufReader::new(file);
    let environment: Environment = serde_json::from_reader(reader)
        .map_err(|e| ImportError::new(ImportErrorKind::Deserialization, &format!("Failed to deserialize environment: {}", e)))?;

    validate_environment(&environment)?;
    Ok(environment)
}

/// Export entire workspace to a zip archive
pub fn export_all(
    collections: &[Collection],
    environments: &[Environment],
    path: impl AsRef<Path>,
) -> ExportResult<()> {
    let path = path.as_ref();
    let file = File::create(path)
        .map_err(|e| ExportError::new(ExportErrorKind::Io, &format!("Failed to create zip file: {}", e)))?;

    let mut zip = ZipWriter::new(file);
    let options = FileOptions::<()>::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // Create collections directory
    zip.add_directory("collections/", options)
        .map_err(|e| ExportError::new(ExportErrorKind::Io, &format!("Failed to add collections directory: {}", e)))?;

    // Export each collection
    for collection in collections {
        let collection_name = format!("collections/{}.json", collection.id);
        zip.start_file(&collection_name, options)
            .map_err(|e| ExportError::new(ExportErrorKind::Io, &format!("Failed to start file: {}", e)))?;

        let json = serde_json::to_string_pretty(collection)
            .map_err(|e| ExportError::new(ExportErrorKind::Serialization, &format!("Failed to serialize collection: {}", e)))?;

        zip.write_all(json.as_bytes())
            .map_err(|e| ExportError::new(ExportErrorKind::Io, &format!("Failed to write collection data: {}", e)))?;
    }

    // Create environments directory
    zip.add_directory("environments/", options)
        .map_err(|e| ExportError::new(ExportErrorKind::Io, &format!("Failed to add environments directory: {}", e)))?;

    // Export each environment
    for environment in environments {
        let env_name = format!("environments/{}.json", environment.id);
        zip.start_file(&env_name, options)
            .map_err(|e| ExportError::new(ExportErrorKind::Io, &format!("Failed to start file: {}", e)))?;

        let json = serde_json::to_string_pretty(environment)
            .map_err(|e| ExportError::new(ExportErrorKind::Serialization, &format!("Failed to serialize environment: {}", e)))?;

        zip.write_all(json.as_bytes())
            .map_err(|e| ExportError::new(ExportErrorKind::Io, &format!("Failed to write environment data: {}", e)))?;
    }

    // Write manifest
    let manifest = formats::manifest::WorkspaceManifest {
        version: "1.0".to_string(),
        exported_at: Utc::now(),
        collection_count: collections.len(),
        environment_count: environments.len(),
    };

    zip.start_file("manifest.json", options)
        .map_err(|e| ExportError::new(ExportErrorKind::Io, &format!("Failed to start manifest file: {}", e)))?;

    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| ExportError::new(ExportErrorKind::Serialization, &format!("Failed to serialize manifest: {}", e)))?;

    zip.write_all(manifest_json.as_bytes())
        .map_err(|e| ExportError::new(ExportErrorKind::Io, &format!("Failed to write manifest data: {}", e)))?;

    zip.finish()
        .map_err(|e| ExportError::new(ExportErrorKind::Io, &format!("Failed to finish zip: {}", e)))?;

    Ok(())
}

/// Import entire workspace from a zip archive
pub fn import_all(path: impl AsRef<Path>) -> ImportResult<WorkspaceImport> {
    let path = path.as_ref();
    let file = File::open(path)
        .map_err(|e| ImportError::new(ImportErrorKind::Io, &format!("Failed to open zip file: {}", e)))?;

    let mut zip = ZipArchive::new(file)
        .map_err(|e| ImportError::new(ImportErrorKind::InvalidFormat, &format!("Failed to read zip archive: {}", e)))?;

    // Read manifest
    let manifest_json = {
        let mut manifest_file = zip.by_name("manifest.json")
            .map_err(|e| ImportError::new(ImportErrorKind::InvalidFormat, &format!("Missing manifest: {}", e)))?;

        let mut content = String::new();
        std::io::Read::read_to_string(&mut manifest_file, &mut content)
            .map_err(|e| ImportError::new(ImportErrorKind::Io, &format!("Failed to read manifest: {}", e)))?;
        content
    };

    let _manifest: formats::manifest::WorkspaceManifest = serde_json::from_str(&manifest_json)
        .map_err(|e| ImportError::new(ImportErrorKind::InvalidFormat, &format!("Invalid manifest: {}", e)))?;

    // Import collections
    let mut collections = Vec::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)
            .map_err(|e| ImportError::new(ImportErrorKind::Io, &format!("Failed to access zip entry: {}", e)))?;

        let name = file.name().to_string();
        if name.starts_with("collections/") && name.ends_with(".json") {
            let mut content = String::new();
            std::io::Read::read_to_string(&mut file, &mut content)
                .map_err(|e| ImportError::new(ImportErrorKind::Io, &format!("Failed to read collection: {}", e)))?;

            let collection: Collection = serde_json::from_str(&content)
                .map_err(|e| ImportError::new(ImportErrorKind::Deserialization, &format!("Failed to deserialize collection: {}", e)))?;

            validate_collection(&collection)?;
            collections.push(collection);
        }
    }

    // Import environments
    let mut environments = Vec::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)
            .map_err(|e| ImportError::new(ImportErrorKind::Io, &format!("Failed to access zip entry: {}", e)))?;

        let name = file.name().to_string();
        if name.starts_with("environments/") && name.ends_with(".json") {
            let mut content = String::new();
            std::io::Read::read_to_string(&mut file, &mut content)
                .map_err(|e| ImportError::new(ImportErrorKind::Io, &format!("Failed to read environment: {}", e)))?;

            let environment: Environment = serde_json::from_str(&content)
                .map_err(|e| ImportError::new(ImportErrorKind::Deserialization, &format!("Failed to deserialize environment: {}", e)))?;

            validate_environment(&environment)?;
            environments.push(environment);
        }
    }

    Ok(WorkspaceImport { collections, environments })
}

/// Result of importing a full workspace
#[derive(Debug, Clone)]
pub struct WorkspaceImport {
    pub collections: Vec<Collection>,
    pub environments: Vec<Environment>,
}

/// Validate a collection
fn validate_collection(collection: &Collection) -> ImportResult<()> {
    if collection.name.trim().is_empty() {
        return Err(ImportError::new(
            ImportErrorKind::Validation,
            "Collection name cannot be empty"
        ));
    }

    // Validate all request IDs exist in the requests map
    let mut validate_item = |item: &CollectionItem| -> ImportResult<()> {
        match item {
            CollectionItem::Request(id) => {
                if !collection.requests.contains_key(id) {
                    return Err(ImportError::new(
                        ImportErrorKind::Validation,
                        &format!("Request ID {} not found in collection", id)
                    ));
                }
            }
            CollectionItem::Folder(folder) => {
                validate_folder(folder, &collection.requests)?;
            }
        }
        Ok(())
    };

    for item in &collection.tree {
        validate_item(item)?;
    }

    Ok(())
}

/// Validate a folder and its children
fn validate_folder(folder: &Folder, requests: &std::collections::HashMap<Uuid, RequestDefinition>) -> ImportResult<()> {
    if folder.name.trim().is_empty() {
        return Err(ImportError::new(
            ImportErrorKind::Validation,
            "Folder name cannot be empty"
        ));
    }

    for item in &folder.children {
        match item {
            CollectionItem::Request(id) => {
                if !requests.contains_key(id) {
                    return Err(ImportError::new(
                        ImportErrorKind::Validation,
                        &format!("Request ID {} not found in collection", id)
                    ));
                }
            }
            CollectionItem::Folder(subfolder) => {
                validate_folder(subfolder, requests)?;
            }
        }
    }

    Ok(())
}

/// Validate an environment
fn validate_environment(environment: &Environment) -> ImportResult<()> {
    if environment.name.trim().is_empty() {
        return Err(ImportError::new(
            ImportErrorKind::Validation,
            "Environment name cannot be empty"
        ));
    }

    // Check for duplicate variable keys
    let mut seen_keys = std::collections::HashSet::new();
    for variable in &environment.variables {
        if !variable.key.trim().is_empty() {
            if !seen_keys.insert(&variable.key) {
                return Err(ImportError::new(
                    ImportErrorKind::Validation,
                    &format!("Duplicate variable key: {}", variable.key)
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_collection() {
        let mut collection = Collection::new("Test Collection");
        let request = RequestDefinition::new("Test Request", HttpMethod::GET, "https://example.com");
        collection.add_request(request, None);

        let temp_dir = tempfile::tempdir().unwrap();
        let export_path = temp_dir.path().join("collection.json");

        assert!(export_collection(&collection, &export_path).is_ok());
        assert!(export_path.exists());
    }

    #[test]
    fn test_roundtrip_collection() {
        let mut collection = Collection::new("Test Collection");
        let request = RequestDefinition::new("Test Request", HttpMethod::GET, "https://example.com");
        collection.add_request(request, None);

        let temp_dir = tempfile::tempdir().unwrap();
        let export_path = temp_dir.path().join("collection.json");

        export_collection(&collection, &export_path).unwrap();
        let imported = import_collection(&export_path).unwrap();

        assert_eq!(collection.id, imported.id);
        assert_eq!(collection.name, imported.name);
        assert_eq!(collection.requests.len(), imported.requests.len());
    }

    #[test]
    fn test_export_environment() {
        let mut environment = Environment::new("Test Environment");
        environment.variables.push(Variable {
            key: "API_KEY".to_string(),
            value: "secret".to_string(),
            secret: true,
            enabled: true,
        });

        let temp_dir = tempfile::tempdir().unwrap();
        let export_path = temp_dir.path().join("environment.json");

        assert!(export_environment(&environment, &export_path).is_ok());
        assert!(export_path.exists());
    }

    #[test]
    fn test_roundtrip_environment() {
        let mut environment = Environment::new("Test Environment");
        environment.variables.push(Variable {
            key: "API_KEY".to_string(),
            value: "secret".to_string(),
            secret: true,
            enabled: true,
        });

        let temp_dir = tempfile::tempdir().unwrap();
        let export_path = temp_dir.path().join("environment.json");

        export_environment(&environment, &export_path).unwrap();
        let imported = import_environment(&export_path).unwrap();

        assert_eq!(environment.id, imported.id);
        assert_eq!(environment.name, imported.name);
        assert_eq!(environment.variables.len(), imported.variables.len());
    }

    #[test]
    fn test_workspace_export_import() {
        let mut collection = Collection::new("Test Collection");
        let request = RequestDefinition::new("Test Request", HttpMethod::GET, "https://example.com");
        collection.add_request(request, None);

        let mut environment = Environment::new("Test Environment");
        environment.variables.push(Variable {
            key: "API_KEY".to_string(),
            value: "secret".to_string(),
            secret: true,
            enabled: true,
        });

        let temp_dir = tempfile::tempdir().unwrap();
        let export_path = temp_dir.path().join("workspace.zip");

        export_all(&[collection.clone()], &[environment.clone()], &export_path).unwrap();
        let imported = import_all(&export_path).unwrap();

        assert_eq!(1, imported.collections.len());
        assert_eq!(1, imported.environments.len());
        assert_eq!(collection.id, imported.collections[0].id);
        assert_eq!(environment.id, imported.environments[0].id);
    }

    #[test]
    fn test_validate_empty_collection_name() {
        let collection = Collection::new("");
        let temp_dir = tempfile::tempdir().unwrap();
        let export_path = temp_dir.path().join("collection.json");

        export_collection(&collection, &export_path).unwrap();
        let result = import_collection(&export_path);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err().kind(), ImportErrorKind::Validation));
    }

    #[test]
    fn test_validate_empty_environment_name() {
        let environment = Environment::new("");
        let temp_dir = tempfile::tempdir().unwrap();
        let export_path = temp_dir.path().join("environment.json");

        export_environment(&environment, &export_path).unwrap();
        let result = import_environment(&export_path);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err().kind(), ImportErrorKind::Validation));
    }
}
