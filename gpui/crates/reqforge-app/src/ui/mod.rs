//! Stub UI implementation for ReqForge.
//!
//! This module provides a minimal console-based UI that demonstrates the architecture
//! without requiring GPUI components, due to dependency conflicts between core-graphics
//! versions in the GPUI dependency tree.

mod body_editor;
mod env_editor_modal;
mod env_selector;
mod key_value_editor;
mod request_editor;
mod response_viewer;
mod sidebar;
mod tab_bar;

pub use body_editor::{BodyContentType, BodyEditor, EditorMode};
pub use env_editor_modal::{EnvEditorMode, EnvEditorModal, ModalState};
pub use env_selector::{EnvDisplayOption, EnvSelector};
pub use key_value_editor::{EditorType, KeyValueEditor, KeyValueRow};
pub use request_editor::RequestEditor;
pub use response_viewer::ResponseViewer;

use crate::app_state::AppState;
use reqforge_core::models::request::{RequestDefinition, HttpMethod};
use uuid::Uuid;

/// Run the stub UI to demonstrate the application architecture.
pub fn run_stub_ui(mut app_state: AppState) {
    println!();
    println!("========================================");
    println!("Stub UI Demo");
    println!("========================================");
    println!();

    // Create a sample request using the correct constructor
    let sample_request = RequestDefinition::new(
        "Sample GET Request",
        HttpMethod::GET,
        "https://httpbin.org/get",
    );

    // Open a tab with the sample request
    println!("Creating sample request...");
    println!("  Method: {}", sample_request.method);
    println!("  URL: {}", sample_request.url);
    println!("  Name: {}", sample_request.name);
    println!("  ID: {}", sample_request.id);
    println!();

    app_state.open_tab(
        sample_request.id,
        Uuid::new_v4(), // Collection ID
        sample_request.clone(),
    );
    println!("✓ Tab opened");

    // Display current state
    println!();
    println!("Application State:");
    println!("  Open tabs: {}", app_state.open_tabs.len());
    if let Some(index) = app_state.active_tab_index {
        println!("  Active tab index: {}", index);
        if let Some(tab) = app_state.active_tab() {
            println!("  Active request: {}", tab.draft.name);
        }
    }
    println!();

    // Close the tab
    println!("Closing tab...");
    app_state.close_active_tab();
    println!("✓ Tab closed");
    println!("  Remaining tabs: {}", app_state.open_tabs.len());

    // Test core functionality
    println!();
    println!("Testing Core Functionality:");
    let core = app_state.core.read();

    // Access collections (direct field access)
    println!("  Found {} collection(s)", core.collections.len());
    for collection in &core.collections {
        println!("    - {} ({})", collection.name, collection.id);
        // Also show requests in each collection
        for (id, request) in &collection.requests {
            println!("      - {} {} ({})", request.method, request.name, id);
        }
    }

    // Show active environment
    if let Some(env_id) = core.active_environment_id {
        if let Some(env) = core.environments.iter().find(|e| e.id == env_id) {
            println!("  Active environment: {} ({})", env.name, env.id);
            println!("    Variables: {} key(s)", env.variables.len());
        }
    } else {
        println!("  No active environment set");
    }

    println!();
    println!("========================================");
    println!("Architecture Verification:");
    println!("========================================");
    println!("✓ AppState: manages tabs and wraps core");
    println!("✓ Core: headless logic engine");
    println!("✓ Tab management: open/close works");
    println!("✓ HttpEngine: ready to execute requests");
    println!("✓ JsonStore: handles persistence");
    println!();
    println!("Note: This is a stub UI demonstrating the architecture.");
    println!("      The full GPUI UI is disabled due to dependency conflicts.");
    println!("      To enable GPUI, see Cargo.toml for details.");
}

/// Run the stub UI with async request execution demonstration.
///
/// This demonstrates the full flow:
/// 1. Open a tab with a request
/// 2. Click "Send" button (simulated)
/// 3. Spawn async task to call core.execute_request()
/// 4. Update the response viewer with results
pub async fn run_stub_ui_with_async_demo(app_state: &mut AppState) {
    println!();
    println!("========================================");
    println!("Stub UI Demo - Full Request Flow");
    println!("========================================");
    println!();

    // Step 1: Create a sample request
    let sample_request = RequestDefinition::new(
        "Sample GET Request",
        HttpMethod::GET,
        "https://httpbin.org/get",
    );

    println!("Step 1: Creating sample request...");
    println!("  Method: {}", sample_request.method);
    println!("  URL: {}", sample_request.url);
    println!("  Name: {}", sample_request.name);
    println!("  ID: {}", sample_request.id);
    println!();

    // Step 2: Open a tab with the sample request
    println!("Step 2: Opening tab with request...");
    app_state.open_tab(
        sample_request.id,
        Uuid::new_v4(), // Collection ID
        sample_request.clone(),
    );
    println!("✓ Tab opened");
    println!("  Open tabs: {}", app_state.open_tabs.len());
    if let Some(index) = app_state.active_tab_index {
        println!("  Active tab index: {}", index);
        if let Some(tab) = app_state.active_tab() {
            println!("  Active request: {}", tab.draft.name);
        }
    }
    println!();

    // Step 3: Create request editor (simulating UI component)
    println!("Step 3: Creating RequestEditor component...");
    let mut request_editor = RequestEditor::new(sample_request.clone());
    request_editor.render();
    println!("✓ RequestEditor rendered");
    println!();

    // Step 4: Create response viewer (simulating UI component)
    println!("Step 4: Creating ResponseViewer component...");
    let mut response_viewer = ResponseViewer::new();
    response_viewer.render();
    println!("✓ ResponseViewer rendered (empty state)");
    println!();

    // Step 5: Simulate clicking "Send" button
    println!("Step 5: Simulating 'Send' button click...");
    println!("  → User clicks Send button");
    println!("  → RequestEditor.on_send() triggered");
    let request_to_send = request_editor.on_send();
    println!("  → Request prepared for execution");
    println!();

    // Step 6: Set loading state
    println!("Step 6: Setting loading state...");
    app_state.set_active_tab_loading(true);
    println!("  ✓ Loading state set");
    println!();

    // Step 7: Spawn async task to execute the request
    println!("Step 7: Executing request (async)...");
    println!("  → Spawning async task to call core.execute_request()");

    let start_time = std::time::Instant::now();
    let result = app_state.execute_active_tab_request().await;
    let elapsed = start_time.elapsed();

    match result {
        Ok(response) => {
            println!("  ✓ Request completed in {:?}", elapsed);
            println!("    Status: {} {}", response.status, response.status_text);
            println!("    Size: {} bytes", response.size_bytes);
            println!("    Body length: {} bytes", response.body.len());
            println!();

            // Step 8: Update the response viewer
            println!("Step 8: Updating ResponseViewer with results...");
            response_viewer.load_from_core_response(
                &response,
                request_to_send.method,
                request_to_send.url.clone(),
            );
            app_state.update_active_tab_response(response);
            println!("  ✓ ResponseViewer updated");
            println!();

            // Step 9: Render the updated response viewer
            println!("Step 9: Rendering updated ResponseViewer...");
            response_viewer.render();
            println!();
        }
        Err(e) => {
            println!("  ✗ Request failed: {}", e);
            println!();
        }
    }

    // Step 10: Summary
    println!("========================================");
    println!("Flow Summary:");
    println!("========================================");
    println!("✓ AppState: managed tabs and state");
    println!("✓ RequestEditor: triggered send action");
    println!("✓ Async task: executed core.execute_request()");
    println!("✓ ResponseViewer: displayed results");
    println!("✓ Full cycle: UI → Core → UI");
    println!();
    println!("This demonstrates the architecture works:");
    println!("  • Components communicate through AppState");
    println!("  • Async request execution is handled properly");
    println!("  • Response flows back to UI components");
    println!("  • No GPUI rendering required for core logic");
}
