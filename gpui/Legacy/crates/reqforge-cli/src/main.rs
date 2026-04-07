use clap::{Parser, Subcommand};
use reqforge_core::{
    ReqForgeCore, models::{HttpMethod, BodyType, KeyValuePair, RawContentType},
    export_collection, import_collection, export_environment, import_environment,
    export_all, import_all, import_collection_from_postman, import_collection_from_openapi,
};
use serde_json::Value;
use std::path::PathBuf;

/// CLI tool for ReqForge - HTTP client with import/export capabilities
#[derive(Parser, Debug)]
#[command(name = "reqforge-cli")]
#[command(about = "ReqForge CLI - Execute HTTP requests and manage collections", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute a single HTTP request
    Execute {
        /// Path to the JSON file containing the request definition
        request_file: PathBuf,
        /// Workspace directory (default: current directory)
        #[arg(short, long, default_value = ".")]
        workspace: PathBuf,
    },
    /// Export a collection to a JSON file
    ExportCollection {
        /// Collection ID to export
        #[arg(short, long)]
        id: String,
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
        /// Workspace directory (default: current directory)
        #[arg(short, long, default_value = ".")]
        workspace: PathBuf,
    },
    /// Import a collection from a JSON file
    ImportCollection {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,
        /// Import format (default: json)
        #[arg(short, long, default_value = "json")]
        format: String,
        /// Workspace directory (default: current directory)
        #[arg(short, long, default_value = ".")]
        workspace: PathBuf,
    },
    /// Export an environment to a JSON file
    ExportEnvironment {
        /// Environment ID to export
        #[arg(short, long)]
        id: String,
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
        /// Workspace directory (default: current directory)
        #[arg(short, long, default_value = ".")]
        workspace: PathBuf,
    },
    /// Import an environment from a JSON file
    ImportEnvironment {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,
        /// Workspace directory (default: current directory)
        #[arg(short, long, default_value = ".")]
        workspace: PathBuf,
    },
    /// Export entire workspace to a zip archive
    ExportWorkspace {
        /// Output zip file path
        #[arg(short, long)]
        output: PathBuf,
        /// Workspace directory (default: current directory)
        #[arg(short, long, default_value = ".")]
        workspace: PathBuf,
    },
    /// Import entire workspace from a zip archive
    ImportWorkspace {
        /// Input zip file path
        #[arg(short, long)]
        input: PathBuf,
        /// Workspace directory (default: current directory)
        #[arg(short, long, default_value = ".")]
        workspace: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Execute { request_file, workspace } => {
            execute_request(request_file, workspace).await?;
        }
        Commands::ExportCollection { id, output, workspace } => {
            export_collection_cmd(id, output, workspace)?;
        }
        Commands::ImportCollection { input, format, workspace } => {
            import_collection_cmd(input, format, workspace)?;
        }
        Commands::ExportEnvironment { id, output, workspace } => {
            export_environment_cmd(id, output, workspace)?;
        }
        Commands::ImportEnvironment { input, workspace } => {
            import_environment_cmd(input, workspace)?;
        }
        Commands::ExportWorkspace { output, workspace } => {
            export_workspace_cmd(output, workspace)?;
        }
        Commands::ImportWorkspace { input, workspace } => {
            import_workspace_cmd(input, workspace)?;
        }
    }

    Ok(())
}

/// Execute a single HTTP request
async fn execute_request(request_file: PathBuf, workspace: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Read and parse the JSON file
    let json_content = std::fs::read_to_string(&request_file)
        .map_err(|e| format!("Failed to read file {}: {}", request_file.display(), e))?;

    let json_value: Value = serde_json::from_str(&json_content)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Parse the RequestDefinition from JSON
    let request_definition = parse_request_definition(json_value)?;

    // Create ReqForgeCore instance
    let mut core = ReqForgeCore::open(workspace)?;

    // Execute the request
    println!("Executing request: {} {}\n", request_definition.method, request_definition.url);

    let response = core.execute_request(&request_definition).await?;

    // Print the response
    print_response(&response);

    Ok(())
}

/// Export a collection to a JSON file
fn export_collection_cmd(id: String, output: PathBuf, workspace: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let core = ReqForgeCore::open(workspace)?;

    let collection_id = uuid::Uuid::parse_str(&id)
        .map_err(|e| format!("Invalid collection ID: {}", e))?;

    let collection = core.collections.iter()
        .find(|c| c.id == collection_id)
        .ok_or_else(|| format!("Collection with ID {} not found", id))?;

    export_collection(collection, &output)
        .map_err(|e| format!("Failed to export collection: {}", e))?;

    println!("Collection exported successfully to: {}", output.display());
    Ok(())
}

/// Import a collection from a JSON file
fn import_collection_cmd(input: PathBuf, format: String, workspace: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut core = ReqForgeCore::open(&workspace)?;

    let collection = match format.to_lowercase().as_str() {
        "json" => import_collection(&input)
            .map_err(|e| format!("Failed to import JSON collection: {}", e))?,
        "postman" => import_collection_from_postman(&input)
            .map_err(|e| format!("Failed to import Postman collection: {}", e))?,
        "openapi" | "swagger" => import_collection_from_openapi(&input)
            .map_err(|e| format!("Failed to import OpenAPI spec: {}", e))?,
        _ => return Err(format!("Unsupported format: {}. Supported: json, postman, openapi", format).into()),
    };

    println!("Imported collection: '{}' with {} requests", collection.name, collection.requests.len());

    // Save the imported collection to the workspace
    core.store.save_collection(&collection)
        .map_err(|e| format!("Failed to save collection to workspace: {}", e))?;

    println!("Collection saved to workspace");
    Ok(())
}

/// Export an environment to a JSON file
fn export_environment_cmd(id: String, output: PathBuf, workspace: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let core = ReqForgeCore::open(workspace)?;

    let environment_id = uuid::Uuid::parse_str(&id)
        .map_err(|e| format!("Invalid environment ID: {}", e))?;

    let environment = core.environments.iter()
        .find(|e| e.id == environment_id)
        .ok_or_else(|| format!("Environment with ID {} not found", id))?;

    export_environment(environment, &output)
        .map_err(|e| format!("Failed to export environment: {}", e))?;

    println!("Environment exported successfully to: {}", output.display());
    Ok(())
}

/// Import an environment from a JSON file
fn import_environment_cmd(input: PathBuf, workspace: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let environment = import_environment(&input)
        .map_err(|e| format!("Failed to import environment: {}", e))?;

    println!("Imported environment: '{}' with {} variables", environment.name, environment.variables.len());

    let mut core = ReqForgeCore::open(&workspace)?;

    // Check if environment already exists
    if core.environments.iter().any(|e| e.id == environment.id) {
        return Err(format!("Environment with ID {} already exists", environment.id).into());
    }

    core.environments.push(environment.clone());
    core.store.save_environments(&core.environments)
        .map_err(|e| format!("Failed to save environment to workspace: {}", e))?;

    println!("Environment saved to workspace");
    Ok(())
}

/// Export entire workspace to a zip archive
fn export_workspace_cmd(output: PathBuf, workspace: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let core = ReqForgeCore::open(&workspace)?;

    println!("Exporting {} collections and {} environments...", core.collections.len(), core.environments.len());

    export_all(&core.collections, &core.environments, &output)
        .map_err(|e| format!("Failed to export workspace: {}", e))?;

    println!("Workspace exported successfully to: {}", output.display());
    Ok(())
}

/// Import entire workspace from a zip archive
fn import_workspace_cmd(input: PathBuf, workspace: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let workspace_import = import_all(&input)
        .map_err(|e| format!("Failed to import workspace: {}", e))?;

    println!("Imported {} collections and {} environments", workspace_import.collections.len(), workspace_import.environments.len());

    let core = ReqForgeCore::open(&workspace)?;

    // Save all imported collections and environments
    for collection in &workspace_import.collections {
        core.store.save_collection(collection)
            .map_err(|e| format!("Failed to save collection {}: {}", collection.name, e))?;
        println!("Saved collection: {}", collection.name);
    }

    for environment in &workspace_import.environments {
        // Check for duplicates
        if core.environments.iter().any(|e| e.id == environment.id) {
            println!("Skipping duplicate environment: {}", environment.name);
            continue;
        }
    }

    // Merge and save environments
    let mut all_environments = core.environments.clone();
    for environment in workspace_import.environments {
        if !all_environments.iter().any(|e| e.id == environment.id) {
            all_environments.push(environment);
        }
    }

    core.store.save_environments(&all_environments)
        .map_err(|e| format!("Failed to save environments to workspace: {}", e))?;

    println!("Workspace imported successfully");
    Ok(())
}

fn parse_request_definition(json: Value) -> Result<reqforge_core::RequestDefinition, String> {
    use chrono::Utc;
    use uuid::Uuid;

    let id = json.get("id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .unwrap_or_else(Uuid::new_v4);

    let name = json.get("name")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'name' field")?
        .to_string();

    let method_str = json.get("method")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'method' field")?;

    let method = match method_str {
        "GET" => HttpMethod::GET,
        "POST" => HttpMethod::POST,
        "PUT" => HttpMethod::PUT,
        "PATCH" => HttpMethod::PATCH,
        "DELETE" => HttpMethod::DELETE,
        "HEAD" => HttpMethod::HEAD,
        "OPTIONS" => HttpMethod::OPTIONS,
        _ => return Err(format!("Unknown HTTP method: {}", method_str)),
    };

    let url = json.get("url")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'url' field")?
        .to_string();

    // Parse headers
    let headers: Result<Vec<KeyValuePair>, String> = json.get("headers")
        .and_then(|v| v.as_array())
        .unwrap_or(&vec![])
        .iter()
        .map(|v| {
            Ok(KeyValuePair {
                key: v.get("key").and_then(|k| k.as_str()).unwrap_or("").to_string(),
                value: v.get("value").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                enabled: v.get("enabled").and_then(|e| e.as_bool()).unwrap_or(true),
                description: v.get("description").and_then(|d| d.as_str()).map(String::from),
            })
        })
        .collect();
    let headers = headers?;

    // Parse query_params
    let query_params: Result<Vec<KeyValuePair>, String> = json.get("query_params")
        .and_then(|v| v.as_array())
        .unwrap_or(&vec![])
        .iter()
        .map(|v| {
            Ok(KeyValuePair {
                key: v.get("key").and_then(|k| k.as_str()).unwrap_or("").to_string(),
                value: v.get("value").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                enabled: v.get("enabled").and_then(|e| e.as_bool()).unwrap_or(true),
                description: v.get("description").and_then(|d| d.as_str()).map(String::from),
            })
        })
        .collect();
    let query_params = query_params?;

    // Parse body
    let body = parse_body_type(json.get("body"))?;

    let now = Utc::now();

    Ok(reqforge_core::RequestDefinition {
        id,
        name,
        method,
        url,
        headers,
        query_params,
        body,
        created_at: now,
        updated_at: now,
    })
}

fn parse_body_type(body_value: Option<&Value>) -> Result<BodyType, String> {
    match body_value {
        None => Ok(BodyType::None),
        Some(Value::String(s)) if s == "None" => Ok(BodyType::None),
        Some(Value::String(s)) => Ok(BodyType::Raw {
            content: s.clone(),
            content_type: RawContentType::Text,
        }),
        Some(Value::Object(map)) => {
            if let Some(content_type) = map.get("type").and_then(|v| v.as_str()) {
                match content_type {
                    "None" => Ok(BodyType::None),
                    "Raw" => {
                        let content = map.get("content")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let content_type_str = map.get("content_type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Text");
                        let content_type = match content_type_str {
                            "Json" => RawContentType::Json,
                            "Xml" => RawContentType::Xml,
                            "Text" => RawContentType::Text,
                            "Html" => RawContentType::Html,
                            _ => RawContentType::Text,
                        };
                        Ok(BodyType::Raw { content, content_type })
                    }
                    "FormUrlEncoded" => {
                        let fields: Result<Vec<KeyValuePair>, String> = map.get("fields")
                            .and_then(|v| v.as_array())
                            .unwrap_or(&vec![])
                            .iter()
                            .map(|v| {
                                Ok(KeyValuePair {
                                    key: v.get("key").and_then(|k| k.as_str()).unwrap_or("").to_string(),
                                    value: v.get("value").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                    enabled: v.get("enabled").and_then(|e| e.as_bool()).unwrap_or(true),
                                    description: v.get("description").and_then(|d| d.as_str()).map(String::from),
                                })
                            })
                            .collect();
                        Ok(BodyType::FormUrlEncoded(fields?))
                    }
                    _ => Err(format!("Unknown body type: {}", content_type)),
                }
            } else {
                Err("Body object missing 'type' field".to_string())
            }
        }
        Some(_) => Ok(BodyType::None),
    }
}

fn print_response(response: &reqforge_core::HttpResponse) {
    println!("=== Response ===");
    println!("\nStatus: {} {}", response.status, response.status_text);
    println!("Size: {} bytes", response.size_bytes);
    println!("Elapsed: {:?}", response.elapsed);

    println!("\n--- Headers ---");
    for (key, value) in &response.headers {
        println!("{}: {}", key, value);
    }

    println!("\n--- Body ---");
    if let Some(pretty) = response.pretty_body() {
        println!("{}", pretty);
    } else if let Some(text) = response.body_text() {
        println!("{}", text);
    } else {
        println!("<binary data, {} bytes>", response.body.len());
    }
}
