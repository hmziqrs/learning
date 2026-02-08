//! UI implementation for ReqForge using gpui-component.

mod root;
mod sidebar;

// Stub modules - will be replaced with GPUI components in Phase 4
mod body_editor;
mod env_editor_modal;
mod env_selector;
mod key_value_editor;
mod request_editor;
mod response_viewer;
mod tab_bar;

pub use root::RootView;
pub use sidebar::SidebarPanel;
pub use request_editor::RequestEditor;
pub use response_viewer::ResponseViewer;
pub use tab_bar::{RequestTabBar, init as init_tab_bar};