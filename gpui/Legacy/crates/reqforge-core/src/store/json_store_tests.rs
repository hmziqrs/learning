use tempfile::TempDir;
use uuid::Uuid;

use crate::models::collection::Collection;
use crate::models::environment::{Environment, Variable};
use crate::models::folder::{CollectionItem, Folder};
use crate::models::request::{BodyType, HttpMethod, KeyValuePair, RawContentType, RequestDefinition};
use crate::store::json_store::JsonStore;

/// Helper to create a test environment with sample data
fn create_test_environment(name: &str) -> Environment {
    Environment {
        id: Uuid::new_v4(),
        name: name.to_string(),
        variables: vec![
            Variable {
                key: "base_url".to_string(),
                value: "https://api.example.com".to_string(),
                secret: false,
                enabled: true,
            },
            Variable {
                key: "api_key".to_string(),
                value: "secret123".to_string(),
                secret: true,
                enabled: true,
            },
            Variable {
                key: "unused".to_string(),
                value: "value".to_string(),
                secret: false,
                enabled: false,
            },
        ],
    }
}

/// Helper to create a test request definition
fn create_test_request(name: &str, method: HttpMethod, url: &str) -> RequestDefinition {
    let mut request = RequestDefinition::new(name, method, url);
    request.headers = vec![
        KeyValuePair {
            key: "Content-Type".to_string(),
            value: "application/json".to_string(),
            enabled: true,
            description: Some("Content type header".to_string()),
        },
        KeyValuePair {
            key: "Authorization".to_string(),
            value: "Bearer {{api_key}}".to_string(),
            enabled: true,
            description: None,
        },
    ];
    request.query_params = vec![
        KeyValuePair {
            key: "page".to_string(),
            value: "1".to_string(),
            enabled: true,
            description: None,
        },
    ];
    request.body = BodyType::Raw {
        content: r#"{"test": "data"}"#.to_string(),
        content_type: RawContentType::Json,
    };
    request
}

/// Helper to create a test collection with sample data
fn create_test_collection(name: &str) -> Collection {
    let id = Uuid::new_v4();
    let mut collection = Collection::new(name);
    collection.id = id;

    // Create a nested folder structure
    let folder1_id = Uuid::new_v4();
    let folder2_id = Uuid::new_v4();

    let request1 = create_test_request("Get Users", HttpMethod::GET, "{{base_url}}/users");
    let request2 = create_test_request("Create User", HttpMethod::POST, "{{base_url}}/users");
    let request3 = create_test_request("Get User", HttpMethod::GET, "{{base_url}}/users/{{id}}");

    let req1_id = request1.id;
    let req2_id = request2.id;
    let req3_id = request3.id;

    collection.requests.insert(req1_id, request1);
    collection.requests.insert(req2_id, request2);
    collection.requests.insert(req3_id, request3);

    // Build tree structure: root has folder1 and request1, folder1 has folder2 and request2, folder2 has request3
    collection.tree = vec![
        CollectionItem::Request(req1_id),
        CollectionItem::Folder(Folder {
            id: folder1_id,
            name: "User Operations".to_string(),
            children: vec![
                CollectionItem::Request(req2_id),
                CollectionItem::Folder(Folder {
                    id: folder2_id,
                    name: "Nested Folder".to_string(),
                    children: vec![CollectionItem::Request(req3_id)],
                }),
            ],
        }),
    ];

    collection
}

/// Helper to compare environments field by field
fn environments_equal(env1: &Environment, env2: &Environment) -> bool {
    env1.id == env2.id
        && env1.name == env2.name
        && env1.variables.len() == env2.variables.len()
        && env1
            .variables
            .iter()
            .zip(env2.variables.iter())
            .all(|(v1, v2)| v1.key == v2.key && v1.value == v2.value && v1.secret == v2.secret && v1.enabled == v2.enabled)
}

/// Helper to compare request definitions field by field
fn requests_equal(req1: &RequestDefinition, req2: &RequestDefinition) -> bool {
    if req1.id != req2.id
        || req1.name != req2.name
        || req1.method != req2.method
        || req1.url != req2.url
        || req1.headers.len() != req2.headers.len()
        || req1.query_params.len() != req2.query_params.len()
    {
        return false;
    }

    // Compare headers
    for (h1, h2) in req1.headers.iter().zip(req2.headers.iter()) {
        if h1.key != h2.key || h1.value != h2.value || h1.enabled != h2.enabled || h1.description != h2.description {
            return false;
        }
    }

    // Compare query params
    for (p1, p2) in req1.query_params.iter().zip(req2.query_params.iter()) {
        if p1.key != p2.key || p1.value != p2.value || p1.enabled != p2.enabled {
            return false;
        }
    }

    // Compare bodies
    match (&req1.body, &req2.body) {
        (BodyType::None, BodyType::None) => true,
        (BodyType::Raw { content: c1, content_type: t1 }, BodyType::Raw { content: c2, content_type: t2 }) => {
            c1 == c2 && std::mem::discriminant(t1) == std::mem::discriminant(t2)
        }
        (BodyType::FormUrlEncoded(f1), BodyType::FormUrlEncoded(f2)) => {
            f1.len() == f2.len()
                && f1.iter()
                    .zip(f2.iter())
                    .all(|(p1, p2)| p1.key == p2.key && p1.value == p2.value && p1.enabled == p2.enabled)
        }
        _ => false,
    }
}

/// Helper to compare collections field by field
fn collections_equal(col1: &Collection, col2: &Collection) -> bool {
    if col1.id != col2.id || col1.name != col2.name || col1.requests.len() != col2.requests.len() {
        return false;
    }

    // Compare requests - use ID lookup instead of iteration order
    for (id, req1) in &col1.requests {
        if let Some(req2) = col2.requests.get(id) {
            if !requests_equal(req1, req2) {
                return false;
            }
        } else {
            return false;
        }
    }

    // Compare trees
    tree_items_equal(&col1.tree, &col2.tree)
}

/// Helper to compare collection item trees recursively
fn tree_items_equal(tree1: &[CollectionItem], tree2: &[CollectionItem]) -> bool {
    if tree1.len() != tree2.len() {
        return false;
    }

    for (item1, item2) in tree1.iter().zip(tree2.iter()) {
        match (item1, item2) {
            (CollectionItem::Request(id1), CollectionItem::Request(id2)) => {
                if id1 != id2 {
                    return false;
                }
            }
            (CollectionItem::Folder(f1), CollectionItem::Folder(f2)) => {
                if f1.id != f2.id || f1.name != f2.name {
                    return false;
                }
                if !tree_items_equal(&f1.children, &f2.children) {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}

#[cfg(test)]
mod json_store_tests {
    use super::*;

    #[test]
    fn test_directory_creation_on_open() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().join("workspace");
        let collections_dir = root.join("collections");

        // Verify directories don't exist yet
        assert!(!root.exists());
        assert!(!collections_dir.exists());

        // Open the store - should create directories
        let _store = JsonStore::open(&root).unwrap();

        // Verify directories were created
        assert!(root.exists());
        assert!(collections_dir.exists());
    }

    #[test]
    fn test_empty_state_handling() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Loading environments when none exist should return empty vec
        let envs = store.load_environments().unwrap();
        assert!(envs.is_empty());

        // Listing collections when none exist should return empty vec
        let collections = store.list_collections().unwrap();
        assert!(collections.is_empty());
    }

    #[test]
    fn test_save_and_load_environments() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Create and save environments
        let env1 = create_test_environment("Development");
        let env2 = create_test_environment("Production");
        let original_envs = vec![env1.clone(), env2.clone()];

        store.save_environments(&original_envs).unwrap();

        // Load environments
        let loaded_envs = store.load_environments().unwrap();

        assert_eq!(loaded_envs.len(), 2);

        // Verify each environment
        for (original, loaded) in original_envs.iter().zip(loaded_envs.iter()) {
            assert!(environments_equal(original, loaded));
        }
    }

    #[test]
    fn test_save_and_load_collections() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Create and save collections
        let col1 = create_test_collection("API Collection");
        let col2 = create_test_collection("Tests Collection");
        let col1_id = col1.id;
        let col2_id = col2.id;

        store.save_collection(&col1).unwrap();
        store.save_collection(&col2).unwrap();

        // Load collections
        let loaded_collections = store.list_collections().unwrap();

        assert_eq!(loaded_collections.len(), 2);

        // Find collections by ID
        let loaded_col1 = loaded_collections.iter().find(|c| c.id == col1_id).unwrap();
        let loaded_col2 = loaded_collections.iter().find(|c| c.id == col2_id).unwrap();

        assert!(collections_equal(&col1, loaded_col1));
        assert!(collections_equal(&col2, loaded_col2));
    }

    #[test]
    fn test_round_trip_environment_serialization() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Create a complex environment
        let original = create_test_environment("Production");

        // Save
        store.save_environments(&[original.clone()]).unwrap();

        // Load
        let loaded = &store.load_environments().unwrap()[0];

        // Verify equality
        assert!(environments_equal(&original, loaded));
        assert_eq!(original.id, loaded.id);
        assert_eq!(original.name, loaded.name);
        assert_eq!(original.variables.len(), loaded.variables.len());

        for (orig_var, loaded_var) in original.variables.iter().zip(loaded.variables.iter()) {
            assert_eq!(orig_var.key, loaded_var.key);
            assert_eq!(orig_var.value, loaded_var.value);
            assert_eq!(orig_var.secret, loaded_var.secret);
            assert_eq!(orig_var.enabled, loaded_var.enabled);
        }
    }

    #[test]
    fn test_round_trip_collection_serialization() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Create a complex collection
        let original = create_test_collection("Complex API");

        // Save
        store.save_collection(&original).unwrap();

        // Load
        let loaded = &store.list_collections().unwrap()[0];

        // Verify equality
        assert!(collections_equal(&original, loaded));
        assert_eq!(original.id, loaded.id);
        assert_eq!(original.name, loaded.name);
        assert_eq!(original.requests.len(), loaded.requests.len());
    }

    #[test]
    fn test_round_trip_request_definition_serialization() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Create a collection with various request types
        let mut collection = create_test_collection("Request Tests");

        // Add different request types
        let get_req = create_test_request("Simple GET", HttpMethod::GET, "https://api.test.com/simple");
        let post_req = create_test_request("POST Form", HttpMethod::POST, "https://api.test.com/form");
        let mut post_with_form = post_req;
        post_with_form.body = BodyType::FormUrlEncoded(vec![
            KeyValuePair {
                key: "field1".to_string(),
                value: "value1".to_string(),
                enabled: true,
                description: Some("First field".to_string()),
            },
            KeyValuePair {
                key: "field2".to_string(),
                value: "value2".to_string(),
                enabled: false,
                description: None,
            },
        ]);

        let patch_req = create_test_request("PATCH", HttpMethod::PATCH, "https://api.test.com/resource");
        let delete_req = create_test_request("DELETE", HttpMethod::DELETE, "https://api.test.com/resource");

        collection.requests.insert(get_req.id, get_req);
        collection.requests.insert(post_with_form.id, post_with_form);
        collection.requests.insert(patch_req.id, patch_req);
        collection.requests.insert(delete_req.id, delete_req);

        // Save
        store.save_collection(&collection).unwrap();

        // Load
        let loaded = &store.list_collections().unwrap()[0];

        // Verify all requests - use ID lookup instead of iteration order
        assert_eq!(collection.requests.len(), loaded.requests.len());

        for (orig_id, orig_req) in &collection.requests {
            if let Some(loaded_req) = loaded.requests.get(orig_id) {
                assert!(requests_equal(orig_req, loaded_req), "Request with ID {} did not round-trip correctly", orig_id);
            } else {
                panic!("Request with ID {} not found in loaded collection", orig_id);
            }
        }
    }

    #[test]
    fn test_delete_collection() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Create and save collections
        let col1 = create_test_collection("Collection 1");
        let col2 = create_test_collection("Collection 2");
        let col3 = create_test_collection("Collection 3");
        let col1_id = col1.id;

        store.save_collection(&col1).unwrap();
        store.save_collection(&col2).unwrap();
        store.save_collection(&col3).unwrap();

        // Verify all three exist
        let collections = store.list_collections().unwrap();
        assert_eq!(collections.len(), 3);

        // Delete one collection
        store.delete_collection(&col1).unwrap();

        // Verify only two remain
        let collections = store.list_collections().unwrap();
        assert_eq!(collections.len(), 2);

        // Verify the deleted collection is gone
        assert!(!collections.iter().any(|c| c.id == col1_id));

        // Verify the other collections still exist
        assert!(collections.iter().any(|c| c.id == col2.id));
        assert!(collections.iter().any(|c| c.id == col3.id));
    }

    #[test]
    fn test_delete_nonexistent_collection() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Create a collection that doesn't exist on disk
        let fake_collection = create_test_collection("Fake");

        // Deleting should not error
        store.delete_collection(&fake_collection).unwrap();
    }

    #[test]
    fn test_overwrite_environment() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Save initial environments
        let env1 = create_test_environment("Development");
        let env2 = create_test_environment("Production");
        store.save_environments(&[env1.clone(), env2.clone()]).unwrap();

        // Overwrite with different set
        let env3 = create_test_environment("Staging");
        let env4 = create_test_environment("QA");
        store.save_environments(&[env3.clone(), env4.clone()]).unwrap();

        // Load and verify
        let loaded = store.load_environments().unwrap();
        assert_eq!(loaded.len(), 2);

        // Should be the new environments
        assert!(environments_equal(&env3, &loaded[0]));
        assert!(environments_equal(&env4, &loaded[1]));
    }

    #[test]
    fn test_overwrite_collection() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Create and save initial collection
        let mut collection = create_test_collection("Original");
        collection.requests.clear();
        collection.tree.clear();

        store.save_collection(&collection).unwrap();

        // Modify and save again
        let req = create_test_request("New Request", HttpMethod::GET, "https://api.test.com");
        collection.requests.insert(req.id, req.clone());
        collection.tree.push(CollectionItem::Request(req.id));

        store.save_collection(&collection).unwrap();

        // Load and verify
        let loaded = &store.list_collections().unwrap()[0];
        assert!(collections_equal(&collection, loaded));
        assert_eq!(loaded.requests.len(), 1);
    }

    #[test]
    fn test_empty_environments() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Save empty list
        store.save_environments(&[]).unwrap();

        // Load should return empty
        let loaded = store.load_environments().unwrap();
        assert!(loaded.is_empty());
    }

    #[test]
    fn test_empty_collection() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Create empty collection
        let collection = Collection::new("Empty Collection");

        // Save
        store.save_collection(&collection).unwrap();

        // Load
        let loaded = &store.list_collections().unwrap()[0];

        // Verify
        assert_eq!(loaded.name, collection.name);
        assert!(loaded.tree.is_empty());
        assert!(loaded.requests.is_empty());
    }

    #[test]
    fn test_environment_with_special_characters() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonStore::open(temp_dir.path()).unwrap();

        // Create environment with special characters in values
        let env = Environment {
            id: Uuid::new_v4(),
            name: "Test Environment".to_string(),
            variables: vec![
                Variable {
                    key: "json_data".to_string(),
                    value: r#"{"key": "value", "nested": {"a": 1}}"#.to_string(),
                    secret: false,
                    enabled: true,
                },
                Variable {
                    key: "url_with_query".to_string(),
                    value: "https://example.com?foo=bar&baz=qux".to_string(),
                    secret: false,
                    enabled: true,
                },
                Variable {
                    key: "multiline".to_string(),
                    value: "line1\nline2\nline3".to_string(),
                    secret: false,
                    enabled: true,
                },
                Variable {
                    key: "unicode".to_string(),
                    value: "Hello 世界 🌍".to_string(),
                    secret: false,
                    enabled: true,
                },
            ],
        };

        // Save
        store.save_environments(&[env.clone()]).unwrap();

        // Load
        let loaded = &store.load_environments().unwrap()[0];

        // Verify
        assert!(environments_equal(&env, loaded));

        for (orig_var, loaded_var) in env.variables.iter().zip(loaded.variables.iter()) {
            assert_eq!(orig_var.value, loaded_var.value);
        }
    }
}
