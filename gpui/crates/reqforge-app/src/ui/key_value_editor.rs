//! Key-value editor component - for editing headers, params, form data.
//!
//! This is a stub implementation that demonstrates the component structure
//! without requiring GPUI rendering.

use reqforge_core::models::environment::Variable;

/// A single row in the key-value editor.
#[derive(Debug, Clone)]
pub struct KeyValueRow {
    /// Unique identifier for this row
    pub id: uuid::Uuid,
    /// Key field value
    pub key: String,
    /// Value field value
    pub value: String,
    /// Whether this row is enabled/active
    pub enabled: bool,
    /// Whether this value is a secret (should be masked)
    pub secret: bool,
    /// Whether the key field is focused
    pub key_focused: bool,
    /// Whether the value field is focused
    pub value_focused: bool,
}

impl KeyValueRow {
    /// Create a new key-value row.
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            key: String::new(),
            value: String::new(),
            enabled: true,
            secret: false,
            key_focused: false,
            value_focused: false,
        }
    }

    /// Create a key-value row from a key and value.
    pub fn from_kv(key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut row = Self::new();
        row.key = key.into();
        row.value = value.into();
        row
    }

    /// Create a key-value row from a Variable.
    pub fn from_variable(variable: &Variable) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            key: variable.key.clone(),
            value: variable.value.clone(),
            enabled: variable.enabled,
            secret: variable.secret,
            key_focused: false,
            value_focused: false,
        }
    }

    /// Check if this row has valid content.
    pub fn is_valid(&self) -> bool {
        !self.key.is_empty() || !self.value.is_empty()
    }

    /// Get display value (masking secrets if necessary).
    pub fn display_value(&self) -> String {
        if self.secret && !self.value.is_empty() {
            "••••••••".to_string()
        } else {
            self.value.clone()
        }
    }
}

impl Default for KeyValueRow {
    fn default() -> Self {
        Self::new()
    }
}

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
    /// Get the display name for this editor type.
    pub fn display_name(&self) -> &str {
        match self {
            EditorType::Params => "Query Parameters",
            EditorType::Headers => "Headers",
            EditorType::FormData => "Form Data",
            EditorType::Variables => "Variables",
        }
    }

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
}

/// Key-value editor component.
///
/// Provides a table-like interface for editing key-value pairs with
/// add/remove/toggle functionality. Used for headers, params, form data, etc.
pub struct KeyValueEditor {
    /// Type of data being edited
    pub editor_type: EditorType,
    /// Rows of key-value data
    pub rows: Vec<KeyValueRow>,
    /// Whether the editor is read-only
    pub read_only: bool,
    /// Currently focused row index
    pub focused_row: Option<usize>,
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
            focused_row: None,
            show_secret_toggle: editor_type == EditorType::Variables,
        }
    }

    /// Create a new key-value editor with initial rows.
    pub fn with_rows(editor_type: EditorType, rows: Vec<KeyValueRow>) -> Self {
        let mut editor = Self::new(editor_type);
        editor.rows = rows;
        editor
    }

    /// Create from a vector of (key, value) pairs.
    pub fn from_pairs(editor_type: EditorType, pairs: Vec<(String, String)>) -> Self {
        let rows: Vec<KeyValueRow> = pairs
            .into_iter()
            .map(|(k, v)| KeyValueRow::from_kv(k, v))
            .collect();
        Self::with_rows(editor_type, rows)
    }

    /// Create from environment variables.
    pub fn from_variables(variables: &[Variable]) -> Self {
        let rows: Vec<KeyValueRow> = variables
            .iter()
            .map(KeyValueRow::from_variable)
            .collect();
        Self {
            editor_type: EditorType::Variables,
            rows,
            read_only: false,
            focused_row: None,
            show_secret_toggle: true,
        }
    }

    /// Render the key-value editor to console (stub implementation).
    pub fn render(&self) {
        println!();
        println!("┌────────────────────────────────────────────────────────────┐");
        println!("│ {}                                           │", self.editor_type.display_name());
        println!("├────────────────────────────────────────────────────────────┤");
        println!("│ ┌──────────────────────────────────────────────────────┐ │");

        // Header row
        let secret_header = if self.show_secret_toggle { " 🔒" } else { "" };
        println!("│ │ Key            │ Value             │ Enabled{}      │ │", secret_header);
        println!("│ │────────────────│───────────────────│──────────────────│ │");

        if self.rows.is_empty() {
            println!("│ │ (No entries)                                           │ │");
        } else {
            for (i, row) in self.rows.iter().enumerate() {
                let focus_marker = if self.focused_row == Some(i) { "►" } else { " " };
                let enabled_marker = if row.enabled { "☑" } else { "☐" };
                let secret_marker = if self.show_secret_toggle && row.secret { "☑" } else if self.show_secret_toggle { "☐" } else { "" };

                let key_display = if row.key.len() > 14 {
                    format!("{}...", &row.key[..11])
                } else {
                    row.key.clone()
                };

                let value_display = row.display_value();
                let value_display = if value_display.len() > 17 {
                    format!("{}...", &value_display[..14])
                } else {
                    value_display
                };

                if self.show_secret_toggle {
                    println!("│ │{} {:<14} │ {:<17} │ {} {}          │ │",
                        focus_marker, key_display, value_display, enabled_marker, secret_marker);
                } else {
                    println!("│ │{} {:<14} │ {:<17} │ {}           │ │",
                        focus_marker, key_display, value_display, enabled_marker);
                }
            }
        }

        println!("│ └──────────────────────────────────────────────────────┘ │");
        println!("│                                                      [+] Add│");
        println!("└────────────────────────────────────────────────────────────┘");

        if self.read_only {
            println!("  (Read-only mode)");
        }
    }

    /// Add a new empty row.
    pub fn add_row(&mut self) -> uuid::Uuid {
        let row = KeyValueRow::new();
        let id = row.id;
        self.rows.push(row);
        self.focused_row = Some(self.rows.len() - 1);
        println!("➕ Added new row with ID: {}", id);
        id
    }

    /// Remove a row by ID.
    pub fn remove_row(&mut self, id: uuid::Uuid) -> bool {
        if let Some(pos) = self.rows.iter().position(|r| r.id == id) {
            let row = self.rows.remove(pos);
            println!("🗑️ Removed row: {}={}", row.key, row.display_value());
            // Update focused row if necessary
            if self.focused_row == Some(pos) {
                self.focused_row = None;
            } else if self.focused_row.map_or(false, |f| f > pos) {
                self.focused_row = self.focused_row.map(|f| f - 1);
            }
            true
        } else {
            println!("⚠ Row not found: {}", id);
            false
        }
    }

    /// Remove a row by index.
    pub fn remove_row_at(&mut self, index: usize) -> bool {
        if index < self.rows.len() {
            let row = self.rows.remove(index);
            println!("🗑️ Removed row at index {}: {}={}", index, row.key, row.display_value());
            // Update focused row if necessary
            if self.focused_row == Some(index) {
                self.focused_row = None;
            } else if self.focused_row.map_or(false, |f| f > index) {
                self.focused_row = self.focused_row.map(|f| f - 1);
            }
            true
        } else {
            println!("⚠ Index out of bounds: {}", index);
            false
        }
    }

    /// Toggle the enabled state of a row.
    pub fn toggle_row(&mut self, id: uuid::Uuid) -> bool {
        if let Some(row) = self.rows.iter_mut().find(|r| r.id == id) {
            row.enabled = !row.enabled;
            println!("🔄 Row {} {}abled", row.key, if row.enabled { "en" } else { "dis" });
            true
        } else {
            println!("⚠ Row not found: {}", id);
            false
        }
    }

    /// Toggle the secret state of a row (for variables).
    pub fn toggle_secret(&mut self, id: uuid::Uuid) -> bool {
        if let Some(row) = self.rows.iter_mut().find(|r| r.id == id) {
            row.secret = !row.secret;
            let secret_state = if row.secret { "on" } else { "off" };
            println!("🔒 Row {} secret: {}", row.key, secret_state);
            true
        } else {
            println!("⚠ Row not found: {}", id);
            false
        }
    }

    /// Update the key of a row.
    pub fn update_key(&mut self, id: uuid::Uuid, key: String) -> bool {
        if let Some(row) = self.rows.iter_mut().find(|r| r.id == id) {
            row.key = key;
            true
        } else {
            false
        }
    }

    /// Update the value of a row.
    pub fn update_value(&mut self, id: uuid::Uuid, value: String) -> bool {
        if let Some(row) = self.rows.iter_mut().find(|r| r.id == id) {
            row.value = value;
            true
        } else {
            false
        }
    }

    /// Focus a specific row.
    pub fn focus_row(&mut self, index: usize) {
        if index < self.rows.len() {
            self.focused_row = Some(index);
        }
    }

    /// Clear focus.
    pub fn clear_focus(&mut self) {
        self.focused_row = None;
    }

    /// Remove all invalid rows (empty keys and values).
    pub fn cleanup(&mut self) -> usize {
        let original_len = self.rows.len();
        self.rows.retain(|r| r.is_valid());
        let removed = original_len - self.rows.len();
        if removed > 0 {
            println!("🧹 Cleaned up {} invalid row(s)", removed);
        }
        removed
    }

    /// Get all key-value pairs as a vector.
    pub fn to_pairs(&self) -> Vec<(String, String)> {
        self.rows
            .iter()
            .filter(|r| r.enabled && r.is_valid())
            .map(|r| (r.key.clone(), r.value.clone()))
            .collect()
    }

    /// Get all key-value pairs as a HashMap.
    pub fn to_map(&self) -> std::collections::HashMap<String, String> {
        self.to_pairs().into_iter().collect()
    }

    /// Convert to environment variables.
    pub fn to_variables(&self) -> Vec<Variable> {
        self.rows
            .iter()
            .map(|r| Variable {
                key: r.key.clone(),
                value: r.value.clone(),
                enabled: r.enabled,
                secret: r.secret,
            })
            .collect()
    }

    /// Set read-only mode.
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    /// Get the count of valid (enabled and non-empty) entries.
    pub fn valid_count(&self) -> usize {
        self.rows.iter().filter(|r| r.enabled && r.is_valid()).count()
    }

    /// Get the total count of entries.
    pub fn total_count(&self) -> usize {
        self.rows.len()
    }
}

impl Default for KeyValueEditor {
    fn default() -> Self {
        Self::new(EditorType::Params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_creation() {
        let editor = KeyValueEditor::new(EditorType::Headers);
        assert_eq!(editor.editor_type, EditorType::Headers);
        assert!(editor.rows.is_empty());
        assert!(!editor.read_only);
    }

    #[test]
    fn test_add_row() {
        let mut editor = KeyValueEditor::new(EditorType::Params);
        let id = editor.add_row();
        assert_eq!(editor.rows.len(), 1);
        assert_eq!(editor.rows[0].id, id);
    }

    #[test]
    fn test_remove_row() {
        let mut editor = KeyValueEditor::new(EditorType::FormData);
        let id = editor.add_row();
        assert_eq!(editor.rows.len(), 1);
        assert!(editor.remove_row(id));
        assert_eq!(editor.rows.len(), 0);
    }

    #[test]
    fn test_toggle_row() {
        let mut editor = KeyValueEditor::new(EditorType::Params);
        let id = editor.add_row();
        assert!(editor.rows[0].enabled);
        editor.toggle_row(id);
        assert!(!editor.rows[0].enabled);
        editor.toggle_row(id);
        assert!(editor.rows[0].enabled);
    }

    #[test]
    fn test_from_pairs() {
        let pairs = vec![
            ("Accept".to_string(), "application/json".to_string()),
            ("Content-Type".to_string(), "text/plain".to_string()),
        ];
        let editor = KeyValueEditor::from_pairs(EditorType::Headers, pairs);
        assert_eq!(editor.rows.len(), 2);
        assert_eq!(editor.rows[0].key, "Accept");
        assert_eq!(editor.rows[1].key, "Content-Type");
    }

    #[test]
    fn test_to_pairs() {
        let mut editor = KeyValueEditor::new(EditorType::Params);
        editor.add_row();
        editor.rows[0].key = "foo".to_string();
        editor.rows[0].value = "bar".to_string();

        let pairs = editor.to_pairs();
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0], (String::from("foo"), String::from("bar")));
    }

    #[test]
    fn test_valid_count() {
        let mut editor = KeyValueEditor::new(EditorType::Headers);
        editor.add_row();
        editor.rows[0].key = "Valid".to_string();
        editor.rows[0].value = "Value".to_string();

        editor.add_row();
        // Second row is empty (invalid)

        assert_eq!(editor.valid_count(), 1);
        assert_eq!(editor.total_count(), 2);
    }

    #[test]
    fn test_cleanup() {
        let mut editor = KeyValueEditor::new(EditorType::Params);
        editor.add_row();
        editor.rows[0].key = "good".to_string();
        editor.rows[0].value = "row".to_string();

        editor.add_row();
        // Second row is empty

        let removed = editor.cleanup();
        assert_eq!(removed, 1);
        assert_eq!(editor.rows.len(), 1);
        assert_eq!(editor.rows[0].key, "good");
    }
}
