//! ReqForge Application Library
//!
//! This library contains the UI components and bridge layer for the ReqForge
//! HTTP client application.

pub mod app_state;
pub mod bridge;
pub mod ui;

// Re-export commonly used types
pub use app_state::{AppState, TabState, KeyValueRow};
pub use bridge::{
    build_request_from_tab,
    build_request_from_components,
    populate_tab_from_request,
    key_value_row_to_pair,
    key_value_rows_to_pairs,
    body_string_to_body_type,
    body_type_to_string,
};
