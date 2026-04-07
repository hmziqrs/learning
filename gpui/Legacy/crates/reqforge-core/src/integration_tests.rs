use tempfile::TempDir;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

use crate::ReqForgeCore;
use crate::models::collection::Collection;
use crate::models::environment::{Environment, Variable};
use crate::models::request::{HttpMethod, KeyValuePair, RequestDefinition};

/// Helper to create a test request with environment variables
fn create_test_request_with_vars(name: &str, method: HttpMethod, url: &str) -> RequestDefinition {
    let mut request = RequestDefinition::new(name, method, url);
    request.headers = vec![
        KeyValuePair {
            key: "Accept".to_string(),
            value: "application/json".to_string(),
            enabled: true,
            description: None,
        },
        KeyValuePair {
            key: "X-API-Key".to_string(),
            value: "{{api_key}}".to_string(),
            enabled: true,
            description: None,
        },
    ];
    request
}

/// Helper to create a test environment with variables
fn create_test_env(name: &str, base_url: &str, api_key: &str) -> Environment {
    let mut env = Environment::new(name);
    env.variables = vec![
        Variable {
            key: "base_url".to_string(),
            value: base_url.to_string(),
            secret: false,
            enabled: true,
        },
        Variable {
            key: "api_key".to_string(),
            value: api_key.to_string(),
            secret: true,
            enabled: true,
        },
        Variable {
            key: "disabled_var".to_string(),
            value: "should_not_appear".to_string(),
            secret: false,
            enabled: false,
        },
    ];
    env
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
            .all(|(v1, v2)| {
                v1.key == v2.key
                    && v1.value == v2.value
                    && v1.secret == v2.secret
                    && v1.enabled == v2.enabled
            })
}

/// Helper to compare request definitions field by field
fn requests_equal(req1: &RequestDefinition, req2: &RequestDefinition) -> bool {
    req1.id == req2.id
        && req1.name == req2.name
        && req1.method == req2.method
        && req1.url == req2.url
        && req1.headers.len() == req2.headers.len()
        && req1
            .headers
            .iter()
            .zip(req2.headers.iter())
            .all(|(h1, h2)| {
                h1.key == h2.key
                    && h1.value == h2.value
                    && h1.enabled == h2.enabled
                    && h1.description == h2.description
            })
}

/// Helper to compare collections field by field
fn collections_equal(col1: &Collection, col2: &Collection) -> bool {
    col1.id == col2.id
        && col1.name == col2.name
        && col1.requests.len() == col2.requests.len()
        && col1.tree.len() == col2.tree.len()
        && col1.requests.iter().all(|(id, req1)| {
            col2.requests
                .get(id)
                .map(|req2| requests_equal(req1, req2))
                .unwrap_or(false)
        })
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// End-to-end integration test that verifies the complete workflow:
    /// 1. Creates a temporary workspace
    /// 2. Creates a ReqForgeCore instance
    /// 3. Creates a collection with requests
    /// 4. Creates an environment with variables
    /// 5. Executes a request using the environment
    /// 6. Verifies the response
    /// 7. Saves and reloads to verify persistence
    #[tokio::test]
    async fn test_end_to_end_workflow() {
        // Step 1: Create a temporary workspace
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let workspace_path = temp_dir.path().join("test_workspace");

        // Step 2: Create a ReqForgeCore instance
        let mut core = ReqForgeCore::open(&workspace_path).expect("Failed to open ReqForgeCore");

        // Step 3: Create a collection with requests
        let mut collection = Collection::new("API Test Collection");

        // Create a GET request
        let get_request =
            create_test_request_with_vars("Get User", HttpMethod::GET, "{{base_url}}/api/user");
        let get_request_id = get_request.id;
        collection.add_request(get_request, None);

        // Create a POST request
        let post_request =
            create_test_request_with_vars("Create User", HttpMethod::POST, "{{base_url}}/api/user");
        let post_request_id = post_request.id;
        collection.add_request(post_request, None);

        core.collections.push(collection);

        // Step 4: Create an environment with variables
        let env_id = {
            let env = Environment {
                id: uuid::Uuid::new_v4(),
                name: "Test Environment".to_string(),
                variables: vec![
                    Variable {
                        key: "base_url".to_string(),
                        value: "http://localhost:8080".to_string(),
                        secret: false,
                        enabled: true,
                    },
                    Variable {
                        key: "api_key".to_string(),
                        value: "test-api-key-123".to_string(),
                        secret: true,
                        enabled: true,
                    },
                ],
            };
            let id = env.id;
            core.environments.push(env);
            id
        };

        core.active_environment_id = Some(env_id);

        // Verify environment is loaded
        let active_vars = core.active_vars();
        assert_eq!(active_vars.len(), 2);
        assert_eq!(
            active_vars.get("base_url").unwrap(),
            "http://localhost:8080"
        );
        assert_eq!(active_vars.get("api_key").unwrap(), "test-api-key-123");

        // Step 5 & 6: Execute a request using environment and verify response
        // Set up mock server for testing
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/user"))
            .and(header("X-API-Key", "test-api-key-123"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/json")
                    .set_body_json(serde_json::json!({
                        "id": 123,
                        "name": "Test User",
                        "email": "test@example.com"
                    })),
            )
            .mount(&mock_server)
            .await;

        // Update environment with mock server URL
        core.environments[0].variables[0].value = mock_server.uri();

        // Find and execute the GET request
        let collection = core.collections.first().unwrap();
        let request = collection.requests.get(&get_request_id).unwrap();
        let request_clone = request.clone();

        let response = core
            .execute_request(&request_clone)
            .await
            .expect("Failed to execute request");

        // Verify response
        assert_eq!(response.status, 200);
        assert_eq!(response.status_text, "OK");
        assert!(response.is_success());
        assert!(response.body_text().is_some());
        let body_text = response.body_text().unwrap();
        assert!(body_text.contains("Test User"));
        assert!(body_text.contains("test@example.com"));

        // Verify headers contain content-type
        assert_eq!(
            response.headers.get("content-type").unwrap(),
            "application/json"
        );

        // Step 7: Save and reload to verify persistence
        core.save_all().expect("Failed to save core state");

        // Create a new ReqForgeCore instance to verify persistence
        let mut reloaded_core =
            ReqForgeCore::open(&workspace_path).expect("Failed to reopen ReqForgeCore");

        // Verify environment persistence
        assert_eq!(reloaded_core.environments.len(), 1);
        let reloaded_env = &reloaded_core.environments[0];
        assert_eq!(reloaded_env.name, "Test Environment");
        assert!(environments_equal(&core.environments[0], reloaded_env));

        // Verify collection persistence
        assert_eq!(reloaded_core.collections.len(), 1);
        let reloaded_collection = &reloaded_core.collections[0];
        assert_eq!(reloaded_collection.name, "API Test Collection");
        assert!(collections_equal(&core.collections[0], reloaded_collection));

        // Verify both requests were persisted
        assert_eq!(reloaded_collection.requests.len(), 2);
        assert!(reloaded_collection.requests.contains_key(&get_request_id));
        assert!(reloaded_collection.requests.contains_key(&post_request_id));

        // Verify tree structure
        assert_eq!(reloaded_collection.tree.len(), 2);

        // Verify we can execute a request after reload
        // Set up a new mock for the reloaded core using a different path
        Mock::given(method("GET"))
            .and(path("/api/user/reloaded"))
            .and(header("X-API-Key", "test-api-key-123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 456,
                "name": "Reloaded User",
                "email": "reloaded@example.com"
            })))
            .mount(&mock_server)
            .await;

        // Update the request URL to use the new path
        let collection = reloaded_core.collections.first_mut().unwrap();
        let request = collection.requests.get_mut(&get_request_id).unwrap();
        request.url = format!("{}/api/user/reloaded", mock_server.uri());

        // Set active environment
        reloaded_core.active_environment_id = Some(reloaded_core.environments[0].id);

        let reloaded_request = reloaded_core
            .collections
            .first()
            .unwrap()
            .requests
            .get(&get_request_id)
            .unwrap()
            .clone();

        let reloaded_response = reloaded_core
            .execute_request(&reloaded_request)
            .await
            .expect("Failed to execute request after reload");

        assert_eq!(reloaded_response.status, 200);
        let reloaded_body = reloaded_response.body_text().unwrap();
        assert!(reloaded_body.contains("Reloaded User"));
    }

    /// Test that verifies multiple environments and switching between them
    #[tokio::test]
    async fn test_multiple_environments() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let workspace_path = temp_dir.path().join("multi_env_workspace");

        let mut core = ReqForgeCore::open(&workspace_path).expect("Failed to open ReqForgeCore");

        // Create multiple environments
        let dev_env = create_test_env("Development", "http://localhost:3000", "dev-key-123");
        let staging_env =
            create_test_env("Staging", "https://staging.example.com", "staging-key-456");
        let prod_env = create_test_env("Production", "https://api.example.com", "prod-key-789");

        let dev_env_id = dev_env.id;
        let staging_env_id = staging_env.id;
        let prod_env_id = prod_env.id;

        core.environments = vec![dev_env, staging_env, prod_env];

        // Test switching between environments
        core.active_environment_id = Some(dev_env_id);
        let vars = core.active_vars();
        assert_eq!(vars.get("base_url").unwrap(), "http://localhost:3000");
        assert_eq!(vars.get("api_key").unwrap(), "dev-key-123");

        core.active_environment_id = Some(staging_env_id);
        let vars = core.active_vars();
        assert_eq!(vars.get("base_url").unwrap(), "https://staging.example.com");
        assert_eq!(vars.get("api_key").unwrap(), "staging-key-456");

        core.active_environment_id = Some(prod_env_id);
        let vars = core.active_vars();
        assert_eq!(vars.get("base_url").unwrap(), "https://api.example.com");
        assert_eq!(vars.get("api_key").unwrap(), "prod-key-789");

        // Test no active environment returns empty vars
        core.active_environment_id = None;
        let vars = core.active_vars();
        assert!(vars.is_empty());

        // Verify persistence after saving
        core.save_all()
            .expect("Failed to save multi-environment state");

        let reloaded =
            ReqForgeCore::open(&workspace_path).expect("Failed to reload multi-environment core");

        assert_eq!(reloaded.environments.len(), 3);
        assert_eq!(reloaded.environments[0].name, "Development");
        assert_eq!(reloaded.environments[1].name, "Staging");
        assert_eq!(reloaded.environments[2].name, "Production");
    }

    /// Test environment variable interpolation in URLs
    #[tokio::test]
    async fn test_environment_variable_interpolation() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let workspace_path = temp_dir.path().join("interpolation_workspace");

        let mut core = ReqForgeCore::open(&workspace_path).expect("Failed to open ReqForgeCore");

        // Create environment with various variable types
        let mut env = Environment::new("Test Env");
        env.variables = vec![
            Variable {
                key: "protocol".to_string(),
                value: "https".to_string(),
                secret: false,
                enabled: true,
            },
            Variable {
                key: "domain".to_string(),
                value: "example.com".to_string(),
                secret: false,
                enabled: true,
            },
            Variable {
                key: "port".to_string(),
                value: "8443".to_string(),
                secret: false,
                enabled: true,
            },
            Variable {
                key: "path".to_string(),
                value: "api/v2/users".to_string(),
                secret: false,
                enabled: true,
            },
        ];
        let env_id = env.id;

        core.environments.push(env);
        core.active_environment_id = Some(env_id);

        // Create request with complex URL using variables
        let request = RequestDefinition::new(
            "Complex URL Test",
            HttpMethod::GET,
            "{{protocol}}://{{domain}}:{{port}}/{{path}}",
        );

        // Verify interpolation works
        let vars = core.active_vars();
        let interpolated = crate::env::interpolator::Interpolator::resolve(&request, &vars);

        assert_eq!(interpolated.url, "https://example.com:8443/api/v2/users");
    }

    /// Test disabled variables are not interpolated
    #[tokio::test]
    async fn test_disabled_variables_not_interpolated() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let workspace_path = temp_dir.path().join("disabled_vars_workspace");

        let mut core = ReqForgeCore::open(&workspace_path).expect("Failed to open ReqForgeCore");

        // Create environment with mixed enabled/disabled variables
        let mut env = Environment::new("Mixed Env");
        env.variables = vec![
            Variable {
                key: "enabled_var".to_string(),
                value: "active".to_string(),
                secret: false,
                enabled: true,
            },
            Variable {
                key: "disabled_var".to_string(),
                value: "inactive".to_string(),
                secret: false,
                enabled: false,
            },
        ];
        let env_id = env.id;

        core.environments.push(env);
        core.active_environment_id = Some(env_id);

        // Create request using both variables
        let request = RequestDefinition::new(
            "Mixed Vars Test",
            HttpMethod::GET,
            "{{enabled_var}}/{{disabled_var}}",
        );

        let vars = core.active_vars();
        let interpolated = crate::env::interpolator::Interpolator::resolve(&request, &vars);

        // Only enabled variable should be interpolated
        assert_eq!(interpolated.url, "active/{{disabled_var}}");

        // Verify only enabled variable is in the map
        assert_eq!(vars.len(), 1);
        assert!(vars.contains_key("enabled_var"));
        assert!(!vars.contains_key("disabled_var"));
    }

    /// Test that multiple collections can be managed
    #[tokio::test]
    async fn test_multiple_collections() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let workspace_path = temp_dir.path().join("multi_collection_workspace");

        let mut core = ReqForgeCore::open(&workspace_path).expect("Failed to open ReqForgeCore");

        // Create multiple collections
        let mut api_collection = Collection::new("API Collection");
        let api_req =
            create_test_request_with_vars("API Request", HttpMethod::GET, "{{base_url}}/api");
        api_collection.add_request(api_req, None);

        let mut test_collection = Collection::new("Test Collection");
        let test_req =
            create_test_request_with_vars("Test Request", HttpMethod::POST, "{{base_url}}/test");
        test_collection.add_request(test_req, None);

        let mut docs_collection = Collection::new("Documentation");
        let docs_req =
            create_test_request_with_vars("Docs Request", HttpMethod::GET, "{{base_url}}/docs");
        docs_collection.add_request(docs_req, None);

        core.collections = vec![api_collection, test_collection, docs_collection];

        // Verify all collections are present
        assert_eq!(core.collections.len(), 3);
        assert_eq!(core.collections[0].name, "API Collection");
        assert_eq!(core.collections[1].name, "Test Collection");
        assert_eq!(core.collections[2].name, "Documentation");

        // Verify each collection has its requests
        assert_eq!(core.collections[0].requests.len(), 1);
        assert_eq!(core.collections[1].requests.len(), 1);
        assert_eq!(core.collections[2].requests.len(), 1);

        // Save and reload
        core.save_all()
            .expect("Failed to save multi-collection state");

        let reloaded =
            ReqForgeCore::open(&workspace_path).expect("Failed to reload multi-collection core");

        assert_eq!(reloaded.collections.len(), 3);
        // Collections may be loaded in a different order, so check by name
        let collection_names: Vec<&str> = reloaded
            .collections
            .iter()
            .map(|c| c.name.as_str())
            .collect();
        assert!(collection_names.contains(&"API Collection"));
        assert!(collection_names.contains(&"Test Collection"));
        assert!(collection_names.contains(&"Documentation"));
    }

    /// Test empty workspace initialization
    #[tokio::test]
    async fn test_empty_workspace_initialization() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let workspace_path = temp_dir.path().join("empty_workspace");

        let core = ReqForgeCore::open(&workspace_path).expect("Failed to open ReqForgeCore");

        // Verify empty state
        assert!(core.environments.is_empty());
        assert!(core.collections.is_empty());
        assert!(core.active_environment_id.is_none());

        // Verify active_vars returns empty map
        let vars = core.active_vars();
        assert!(vars.is_empty());

        // Saving empty state should work
        core.save_all().expect("Failed to save empty state");

        // Reloading should maintain empty state
        let reloaded = ReqForgeCore::open(&workspace_path).expect("Failed to reload empty core");

        assert!(reloaded.environments.is_empty());
        assert!(reloaded.collections.is_empty());
    }

    /// Test secret variables are properly stored
    #[tokio::test]
    async fn test_secret_variable_storage() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let workspace_path = temp_dir.path().join("secret_workspace");

        let mut core = ReqForgeCore::open(&workspace_path).expect("Failed to open ReqForgeCore");

        // Create environment with secret variables
        let mut env = Environment::new("Secure Environment");
        env.variables = vec![
            Variable {
                key: "public_key".to_string(),
                value: "public-value".to_string(),
                secret: false,
                enabled: true,
            },
            Variable {
                key: "secret_key".to_string(),
                value: "super-secret-value".to_string(),
                secret: true,
                enabled: true,
            },
            Variable {
                key: "api_token".to_string(),
                value: "bearer-token-12345".to_string(),
                secret: true,
                enabled: true,
            },
        ];

        core.environments.push(env);
        core.active_environment_id = Some(core.environments[0].id);

        // Verify all variables (including secrets) are in active_vars
        let vars = core.active_vars();
        assert_eq!(vars.len(), 3);
        assert_eq!(vars.get("public_key").unwrap(), "public-value");
        assert_eq!(vars.get("secret_key").unwrap(), "super-secret-value");
        assert_eq!(vars.get("api_token").unwrap(), "bearer-token-12345");

        // Save and reload
        core.save_all().expect("Failed to save secrets");

        let reloaded = ReqForgeCore::open(&workspace_path).expect("Failed to reload secrets");

        assert_eq!(reloaded.environments.len(), 1);
        let reloaded_env = &reloaded.environments[0];
        assert_eq!(reloaded_env.variables.len(), 3);

        // Verify secret flags are preserved
        assert!(!reloaded_env.variables[0].secret);
        assert!(reloaded_env.variables[1].secret);
        assert!(reloaded_env.variables[2].secret);
    }

    /// Test that workspace directory structure is created correctly
    #[tokio::test]
    async fn test_workspace_directory_structure() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let workspace_path = temp_dir.path().join("structured_workspace");

        // Verify workspace doesn't exist yet
        assert!(!workspace_path.exists());

        let _core = ReqForgeCore::open(&workspace_path).expect("Failed to open ReqForgeCore");

        // Verify workspace directory was created
        assert!(workspace_path.exists());

        // Verify collections directory exists
        let collections_dir = workspace_path.join("collections");
        assert!(collections_dir.exists());
        assert!(collections_dir.is_dir());
    }

    /// Test request execution with query parameters and environment variables
    #[tokio::test]
    async fn test_request_execution_with_query_params() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let workspace_path = temp_dir.path().join("query_params_workspace");

        let mut core = ReqForgeCore::open(&workspace_path).expect("Failed to open ReqForgeCore");

        // Create environment
        let mut env = Environment::new("Test Env");
        env.variables = vec![
            Variable {
                key: "base_url".to_string(),
                value: "http://localhost:9999".to_string(),
                secret: false,
                enabled: true,
            },
            Variable {
                key: "version".to_string(),
                value: "v1".to_string(),
                secret: false,
                enabled: true,
            },
        ];
        let env_id = env.id;

        core.environments.push(env);
        core.active_environment_id = Some(env_id);

        // Create request with query parameters
        let mut request = RequestDefinition::new(
            "Query Params Test",
            HttpMethod::GET,
            "{{base_url}}/api/data",
        );
        request.query_params = vec![
            KeyValuePair {
                key: "version".to_string(),
                value: "{{version}}".to_string(),
                enabled: true,
                description: None,
            },
            KeyValuePair {
                key: "format".to_string(),
                value: "json".to_string(),
                enabled: true,
                description: None,
            },
            KeyValuePair {
                key: "disabled_param".to_string(),
                value: "should_not_appear".to_string(),
                enabled: false,
                description: None,
            },
        ];

        // Set up mock server
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/data"))
            .and(wiremock::matchers::query_param("version", "v1"))
            .and(wiremock::matchers::query_param("format", "json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "success",
                "data": {"version": "v1", "format": "json"}
            })))
            .mount(&mock_server)
            .await;

        // Update environment with mock server URL
        core.environments[0].variables[0].value = mock_server.uri();

        // Execute request
        let response = core
            .execute_request(&request)
            .await
            .expect("Failed to execute request with query params");

        // Verify response
        assert_eq!(response.status, 200);
        let body = response.body_text().unwrap();
        assert!(body.contains("success"));
    }
}
