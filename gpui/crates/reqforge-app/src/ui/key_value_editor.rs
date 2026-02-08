//! Key-value editor component - for editing headers, params, form data.
//!
//! Zero-copy implementation using gpui-component's Entity<InputState>
//! for managing text inputs without unnecessary String cloning.

use gpui::{
    div, px, AppContext, Context, Entity, EventEmitter, IntoElement, ParentElement,
    Render, Styled, Window,
};
use gpui_component::{h_flex, v_flex, Disableable};
use gpui_component::button::Button;
use gpui_component::checkbox::Checkbox;
use gpui_component::input::{Input, InputState};
use uuid::Uuid;

/// The type of data being edited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorType {
    /// Query parameters
    Params,
    /// HTTP headers
    Headers,
    /// Form data
    FormData,
    /// Environment variables
    Variables,
}

impl EditorType {
    /// Get the placeholder text for the key field.
    pub fn key_placeholder(&self) -> &str {
        match self {
            EditorType::Params => "Parameter name",
            EditorType::Headers => "Header name",
            EditorType::FormData => "Field name",
            EditorType::Variables => "Variable name",
        }
    }

    /// Get the placeholder text for the value field.
    pub fn value_placeholder(&self) -> &str {
        match self {
            EditorType::Params => "Parameter value",
            EditorType::Headers => "Header value",
            EditorType::FormData => "Field value",
            EditorType::Variables => "Variable value",
        }
    }

    /// Get the display name for this editor type.
    pub fn display_name(&self) -> &str {
        match self {
            EditorType::Params => "Parameter",
            EditorType::Headers => "Header",
            EditorType::FormData => "Field",
            EditorType::Variables => "Variable",
        }
    }
}

/// A single row in the key-value editor.
///
/// Uses Entity<InputState> for zero-copy text management.
/// The text is stored internally by gpui-component and only
/// converted to String when needed at ownership boundaries.
#[derive(Clone)]
pub struct KeyValueRow {
    /// Unique identifier for this row
    pub id: Uuid,
    /// Whether this row is enabled/active
    pub enabled: bool,
    /// Key input state (zero-copy: managed by gpui)
    pub key_input: Entity<InputState>,
    /// Value input state (zero-copy: managed by gpui)
    pub value_input: Entity<InputState>,
    /// Whether this value is a secret (should be masked) - for variables
    pub secret: bool,
}

impl KeyValueRow {
    /// Create a new key-value row with empty inputs.
    pub fn new(window: &mut Window, cx: &mut Context<KeyValueEditor>) -> Self {
        let key_input = cx.new(|cx| InputState::new(window, cx));
        let value_input = cx.new(|cx| InputState::new(window, cx));

        Self {
            id: Uuid::new_v4(),
            enabled: true,
            key_input,
            value_input,
            secret: false,
        }
    }

    /// Create a key-value row from initial key and value strings.
    ///
    /// Note: This allocates Strings for the initial values, but
    /// subsequent edits are zero-copy within the InputState.
    pub fn from_kv(
        key: impl Into<String>,
        value: impl Into<String>,
        window: &mut Window,
        cx: &mut Context<KeyValueEditor>,
    ) -> Self {
        let key_string = key.into();
        let value_string = value.into();

        let key_input = cx.new(|cx| {
            InputState::new(window, cx).default_value(key_string)
        });
        let value_input = cx.new(|cx| {
            InputState::new(window, cx).default_value(value_string)
        });

        Self {
            id: Uuid::new_v4(),
            enabled: true,
            key_input,
            value_input,
            secret: false,
        }
    }

    /// Get the current key text.
    ///
    /// This performs a single allocation to convert from the internal
    /// representation to String. This is the ownership boundary.
    pub fn key_text(&self, cx: &mut Context<KeyValueEditor>) -> String {
        self.key_input.read(cx).text().to_string()
    }

    /// Get the current value text.
    ///
    /// This performs a single allocation to convert from the internal
    /// representation to String. This is the ownership boundary.
    pub fn value_text(&self, cx: &mut Context<KeyValueEditor>) -> String {
        self.value_input.read(cx).text().to_string()
    }

    /// Check if this row has valid content.
    pub fn is_valid(&self, cx: &mut Context<KeyValueEditor>) -> bool {
        !self.key_text(cx).is_empty() || !self.value_text(cx).is_empty()
    }
}

/// Key-value editor component.
///
/// Provides a table-like interface for editing key-value pairs with
/// add/remove/toggle functionality. Used for headers, params, form data, etc.
///
/// Zero-copy architecture:
/// - Each row uses Entity<InputState> for key and value inputs
/// - Text is managed internally by gpui-component
/// - String allocation only occurs at ownership boundaries (save/send)
pub struct KeyValueEditor {
    /// Type of data being edited
    pub editor_type: EditorType,
    /// Rows of key-value data
    pub rows: Vec<KeyValueRow>,
    /// Whether the editor is read-only
    pub read_only: bool,
    /// Whether to show the secret toggle column
    pub show_secret_toggle: bool,
}

impl KeyValueEditor {
    /// Create a new key-value editor.
    pub fn new(editor_type: EditorType) -> Self {
        Self {
            editor_type,
            rows: Vec::new(),
            read_only: false,
            show_secret_toggle: editor_type == EditorType::Variables,
        }
    }

    /// Create from a vector of (key, value) pairs.
    pub fn from_pairs(
        editor_type: EditorType,
        pairs: Vec<(String, String)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let rows: Vec<KeyValueRow> = pairs
            .into_iter()
            .map(|(k, v)| KeyValueRow::from_kv(k, v, window, cx))
            .collect();

        Self {
            editor_type,
            rows,
            read_only: false,
            show_secret_toggle: editor_type == EditorType::Variables,
        }
    }

    /// Add a new empty row.
    pub fn add_row(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Uuid {
        let row = KeyValueRow::new(window, cx);
        let id = row.id;
        self.rows.push(row);
        cx.notify();
        id
    }

    /// Remove a row by ID.
    pub fn remove_row(&mut self, id: Uuid, _window: &mut Window, cx: &mut Context<Self>) -> bool {
        if let Some(pos) = self.rows.iter().position(|r| r.id == id) {
            self.rows.remove(pos);
            cx.notify();
            true
        } else {
            false
        }
    }

    /// Toggle the enabled state of a row.
    pub fn toggle_row(&mut self, id: Uuid, _window: &mut Window, cx: &mut Context<Self>) -> bool {
        if let Some(row) = self.rows.iter_mut().find(|r| r.id == id) {
            row.enabled = !row.enabled;
            cx.notify();
            true
        } else {
            false
        }
    }

    /// Toggle the secret state of a row (for variables).
    pub fn toggle_secret(&mut self, id: Uuid, cx: &mut Context<Self>) -> bool {
        if let Some(row) = self.rows.iter_mut().find(|r| r.id == id) {
            row.secret = !row.secret;
            cx.notify();
            true
        } else {
            false
        }
    }

    /// Set read-only mode.
    pub fn set_read_only(&mut self, read_only: bool, cx: &mut Context<Self>) {
        self.read_only = read_only;
        cx.notify();
    }

    /// Get the count of valid (enabled and non-empty) entries.
    pub fn valid_count(&self, cx: &mut Context<Self>) -> usize {
        self.rows.iter().filter(|r| r.enabled && r.is_valid(cx)).count()
    }

    /// Get the total count of entries.
    pub fn total_count(&self) -> usize {
        self.rows.len()
    }

    /// Get all key-value pairs as a vector.
    ///
    /// This is the ownership boundary where we convert from
    /// Entity<InputState> to String pairs.
    pub fn to_pairs(&self, cx: &mut Context<Self>) -> Vec<(String, String)> {
        // Collect valid rows first to avoid borrow checker issues
        let valid_rows: Vec<_> = self.rows
            .iter()
            .filter(|r| r.enabled && r.is_valid(cx))
            .collect();

        // Then map to pairs
        valid_rows
            .into_iter()
            .map(|r| (r.key_text(cx), r.value_text(cx)))
            .collect()
    }
}

impl EventEmitter<()> for KeyValueEditor {}

impl Render for KeyValueEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let read_only = self.read_only;
        let editor_type = self.editor_type;
        let add_button_text = format!("Add {}", editor_type.display_name());

        // Collect rows to avoid borrow issues in closure
        let rows: Vec<_> = self.rows.clone();

        let mut flex = v_flex()
            .gap_2()
            .w_full()
            .h_full();

        // Add each row
        for row in &rows {
            let row_id = row.id;
            let enabled = row.enabled;
            let secret = row.secret;
            let show_secret = self.show_secret_toggle;

            let mut row_div = h_flex()
                .gap_2()
                .w_full()
                .items_center()
                .child(
                    // Key input
                    div()
                        .flex_1()
                        .min_w(px(150.0))
                        .child(Input::new(&row.key_input))
                )
                .child(
                    // Value input
                    div()
                        .flex_1()
                        .min_w(px(200.0))
                        .child(Input::new(&row.value_input))
                )
                .child(
                    // Enabled checkbox
                    Checkbox::new("enabled")
                        .checked(enabled)
                        .on_click(cx.listener(move |editor, _checked, window, cx| {
                            let id = row_id;
                            editor.toggle_row(id, window, cx);
                        }))
                );

            // Add secret checkbox if needed
            if show_secret {
                row_div = row_div.child(
                    Checkbox::new("secret")
                        .checked(secret)
                        .on_click(cx.listener(move |editor, _checked, _window, cx| {
                            let id = row_id;
                            editor.toggle_secret(id, cx);
                        }))
                );
            }

            row_div = row_div.child(
                // Delete button
                div().child(
                    Button::new("delete")
                        .label("×")
                        .disabled(read_only)
                        .on_click(cx.listener(move |editor, _event, window, cx| {
                            let id = row_id;
                            editor.remove_row(id, window, cx);
                        }))
                )
            );

            flex = flex.child(row_div);
        }

        flex.child(
            // Add row button
            h_flex()
                .w_full()
                .p_2()
                .justify_end()
                .child(
                    div().child(
                        Button::new("add-row")
                            .label(add_button_text)
                            .disabled(read_only)
                            .on_click(cx.listener(move |editor, _event, window, cx| {
                                editor.add_row(window, cx);
                            }))
                    )
                )
        )
    }
}

/// Stub implementation for tests - non-GPUI fallback
#[cfg(test)]
mod tests {
    use super::*;

    /// Test row creation logic
    #[test]
    fn test_row_creation_logic() {
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        assert_ne!(id1, id2, "UUIDs should be unique");
    }

    /// Test editor type display names
    #[test]
    fn test_editor_type_display_names() {
        assert_eq!(EditorType::Params.display_name(), "Parameter");
        assert_eq!(EditorType::Headers.display_name(), "Header");
        assert_eq!(EditorType::FormData.display_name(), "Field");
        assert_eq!(EditorType::Variables.display_name(), "Variable");
    }

    /// Test editor type placeholders
    #[test]
    fn test_editor_type_placeholders() {
        assert_eq!(EditorType::Params.key_placeholder(), "Parameter name");
        assert_eq!(EditorType::Headers.key_placeholder(), "Header name");
        assert_eq!(EditorType::FormData.key_placeholder(), "Field name");
        assert_eq!(EditorType::Variables.key_placeholder(), "Variable name");

        assert_eq!(EditorType::Params.value_placeholder(), "Parameter value");
        assert_eq!(EditorType::Headers.value_placeholder(), "Header value");
        assert_eq!(EditorType::FormData.value_placeholder(), "Field value");
        assert_eq!(EditorType::Variables.value_placeholder(), "Variable value");
    }

    /// Test KeyValueEditor creation
    #[test]
    fn test_editor_creation() {
        let editor = KeyValueEditor::new(EditorType::Headers);
        assert_eq!(editor.editor_type, EditorType::Headers);
        assert!(editor.rows.is_empty());
        assert!(!editor.read_only);
        assert!(!editor.show_secret_toggle);
    }

    /// Test show_secret_toggle for Variables
    #[test]
    fn test_variables_show_secret_toggle() {
        let editor = KeyValueEditor::new(EditorType::Variables);
        assert!(editor.show_secret_toggle);
    }

    /// Test total_count
    #[test]
    fn test_total_count() {
        let editor = KeyValueEditor::new(EditorType::Params);
        assert_eq!(editor.total_count(), 0);
    }
}
