//! ReqForge - A Postman-like HTTP Client built with Rust and GPUI
//!
//! This is the main entry point for the ReqForge application.

mod app_state;
mod bridge;
mod ui;

use gpui::{App, Window, WindowOptions, Bounds, Point, Size, TitlebarOptions, px, AppContext};
use std::path::PathBuf;

fn main() {
    // Determine workspace directory before starting the app
    // TODO: Phase 4 - Make this configurable via UI
    let workspace_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".reqforge");

    // Load headless core before starting the app
    let core = match reqforge_core::ReqForgeCore::open(&workspace_dir) {
        Ok(core) => {
            eprintln!("✓ Successfully opened ReqForge workspace");
            core
        }
        Err(e) => {
            eprintln!("✗ Failed to open ReqForge workspace: {}", e);
            // Create the workspace and try again
            std::fs::create_dir_all(&workspace_dir).ok();
            match reqforge_core::ReqForgeCore::open(&workspace_dir) {
                Ok(core) => {
                    eprintln!("✓ Created new ReqForge workspace");
                    core
                }
                Err(e) => {
                    panic!("Failed to create ReqForge workspace: {}", e);
                }
            }
        }
    };

    // Initialize the GPUI application
    gpui::Application::new().run(move |cx: &mut App| {
        // Initialize gpui-component (register actions, themes, etc.)
        gpui_component::init(cx);

        // Create application state - core is moved into AppState
        let app_state = cx.new(|_cx| app_state::AppState::new(core));

        // Create the RootView entity
        let root_view = cx.new(|cx| ui::RootView::new(app_state, cx));

        // Open the main window
        cx.open_window(
            WindowOptions {
                window_bounds: Some(gpui::WindowBounds::Windowed(Bounds {
                    origin: Point { x: px(100.0), y: px(100.0) },
                    size: Size { width: px(1400.0), height: px(900.0) },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("ReqForge - HTTP Client".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_window, _cx| root_view,
        )
        .unwrap();
    });
}
