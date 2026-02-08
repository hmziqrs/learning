//! ReqForge - A Postman-like HTTP Client built with Rust
//!
//! This is the main entry point for the ReqForge application.
//!
//! Note: This is currently a stub implementation that demonstrates the architecture
//! without using GPUI components, due to dependency conflicts between core-graphics
//! versions in the GPUI dependency tree.

mod app_state;
mod ui;

use std::path::PathBuf;

#[tokio::main]
async fn main() {
    println!("========================================");
    println!("ReqForge - HTTP Client");
    println!("========================================");
    println!();

    // Determine workspace directory
    let workspace_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("reqforge");

    println!("Workspace directory: {:?}", workspace_dir);

    // Load headless core
    let core = match reqforge_core::ReqForgeCore::open(&workspace_dir) {
        Ok(core) => {
            println!("✓ Successfully opened ReqForge workspace");
            core
        }
        Err(e) => {
            eprintln!("✗ Failed to open ReqForge workspace: {}", e);
            return;
        }
    };

    // Create application state
    let mut app_state = app_state::AppState::new(core);
    println!("✓ Application state initialized");

    // Run the stub UI with async request execution demo
    ui::run_stub_ui_with_async_demo(&mut app_state).await;

    println!();
    println!("========================================");
    println!("ReqForge shutting down...");
    println!("========================================");
}
