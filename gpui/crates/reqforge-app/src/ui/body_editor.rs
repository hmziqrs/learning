//! Body editor component - text area with content-type selector.
//!
//! This is a stub implementation that demonstrates the component structure
//! without requiring GPUI rendering.

/// Content type options for request body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyContentType {
    /// JSON format
    Json,
    /// XML format
    Xml,
    /// Plain text
    Text,
    /// HTML
    Html,
    /// Form data (application/x-www-form-urlencoded)
    FormUrlEncoded,
    /// Multipart form data
    MultipartFormData,
    /// No body
    None,
}

impl BodyContentType {
    /// Get all content types.
    pub fn all() -> &'static [BodyContentType] {
        &[
            BodyContentType::Json,
            BodyContentType::Xml,
            BodyContentType::Text,
            BodyContentType::Html,
            BodyContentType::FormUrlEncoded,
            BodyContentType::MultipartFormData,
            BodyContentType::None,
        ]
    }

    /// Get the display name for this content type.
    pub fn display_name(&self) -> &str {
        match self {
            BodyContentType::Json => "JSON",
            BodyContentType::Xml => "XML",
            BodyContentType::Text => "Text",
            BodyContentType::Html => "HTML",
            BodyContentType::FormUrlEncoded => "Form (URL-Encoded)",
            BodyContentType::MultipartFormData => "Multipart Form",
            BodyContentType::None => "None",
        }
    }

    /// Get the MIME type string for this content type.
    pub fn mime_type(&self) -> &str {
        match self {
            BodyContentType::Json => "application/json",
            BodyContentType::Xml => "application/xml",
            BodyContentType::Text => "text/plain",
            BodyContentType::Html => "text/html",
            BodyContentType::FormUrlEncoded => "application/x-www-form-urlencoded",
            BodyContentType::MultipartFormData => "multipart/form-data",
            BodyContentType::None => "",
        }
    }

    /// Get the file extension for this content type.
    pub fn file_extension(&self) -> &str {
        match self {
            BodyContentType::Json => "json",
            BodyContentType::Xml => "xml",
            BodyContentType::Text => "txt",
            BodyContentType::Html => "html",
            BodyContentType::FormUrlEncoded => "txt",
            BodyContentType::MultipartFormData => "txt",
            BodyContentType::None => "",
        }
    }

    /// Get default content for this content type.
    pub fn default_content(&self) -> String {
        match self {
            BodyContentType::Json => "{\n  \n}".to_string(),
            BodyContentType::Xml => "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root>\n  \n</root>".to_string(),
            BodyContentType::Text => String::new(),
            BodyContentType::Html => "<!DOCTYPE html>\n<html>\n<body>\n  \n</body>\n</html>".to_string(),
            BodyContentType::FormUrlEncoded => String::new(),
            BodyContentType::MultipartFormData => String::new(),
            BodyContentType::None => String::new(),
        }
    }
}

/// Editor state for body content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    /// Single-line text input
    SingleLine,
    /// Multi-line text area
    MultiLine,
    /// Form data editor (key-value pairs)
    Form,
}

/// Body editor component.
///
/// Provides a text area for editing request body content with a content-type selector.
/// Supports JSON, XML, Text, HTML, and form data formats.
pub struct BodyEditor {
    /// Current content type
    pub content_type: BodyContentType,
    /// Body content text
    pub content: String,
    /// Current editor mode
    pub mode: EditorMode,
    /// Whether the editor is read-only
    pub read_only: bool,
    /// Cursor position (line, column)
    pub cursor: (usize, usize),
    /// Selection range (start, end) as (line, column) tuples
    pub selection: Option<((usize, usize), (usize, usize))>,
    /// Whether content has been modified
    pub dirty: bool,
    /// Content type dropdown open state
    pub dropdown_open: bool,
    /// Current line count
    pub line_count: usize,
}

impl BodyEditor {
    /// Create a new body editor.
    pub fn new() -> Self {
        let content_type = BodyContentType::Json;
        let content = content_type.default_content();
        let line_count = content.lines().count();
        Self {
            content_type,
            content,
            mode: EditorMode::MultiLine,
            read_only: false,
            cursor: (0, 0),
            selection: None,
            dirty: false,
            dropdown_open: false,
            line_count,
        }
    }

    /// Create a body editor with specific content type.
    pub fn with_type(content_type: BodyContentType) -> Self {
        let content = content_type.default_content();
        let line_count = content.lines().count();
        Self {
            content_type,
            content,
            mode: EditorMode::MultiLine,
            read_only: false,
            cursor: (0, 0),
            selection: None,
            dirty: false,
            dropdown_open: false,
            line_count,
        }
    }

    /// Create a body editor with initial content.
    pub fn with_content(content_type: BodyContentType, content: String) -> Self {
        let line_count = content.lines().count();
        let mode = match content_type {
            BodyContentType::FormUrlEncoded | BodyContentType::MultipartFormData => {
                EditorMode::Form
            }
            _ => EditorMode::MultiLine,
        };
        Self {
            content_type,
            content,
            mode,
            read_only: false,
            cursor: (0, 0),
            selection: None,
            dirty: false,
            dropdown_open: false,
            line_count,
        }
    }

    /// Render the body editor to console (stub implementation).
    pub fn render(&self) {
        println!();
        println!("┌────────────────────────────────────────────────────────────┐");
        println!("│ Request Body Editor                                        │");
        println!("├────────────────────────────────────────────────────────────┤");

        // Content type selector
        print!("│ Content-Type: [");
        for (i, ct) in BodyContentType::all().iter().enumerate() {
            let marker = if *ct == self.content_type {
                "●"
            } else {
                " "
            };
            print!("{}{}", marker, ct.display_name());
            if i < BodyContentType::all().len() - 1 {
                print!("|");
            }
        }
        println!("]  │");

        println!("├────────────────────────────────────────────────────────────┤");
        println!("│ ┌──────────────────────────────────────────────────────┐ │");

        // Mode indicator
        let mode_str = match self.mode {
            EditorMode::SingleLine => "[Single]",
            EditorMode::MultiLine => "[Multi]",
            EditorMode::Form => "[Form]",
        };
        println!("│ │ {} {} | Line {}:{} | {} lines{}                    │ │",
            mode_str,
            if self.read_only { "[RO]" } else { "" },
            self.cursor.0,
            self.cursor.1,
            self.line_count,
            if self.dirty { " [Modified]" } else { "" }
        );
        println!("│ │──────────────────────────────────────────────────────│ │");

        // Content preview
        let lines: Vec<&str> = self.content.lines().collect();
        let visible_lines = lines.len().min(8);

        for i in 0..visible_lines {
            let line_num = i + 1;
            let cursor_marker = if self.cursor.0 == i { "►" } else { " " };
            let line = lines.get(i).unwrap_or(&"");
            let display_line = if line.len() > 54 {
                format!("{}...", &line[..51])
            } else {
                line.to_string()
            };
            println!("│ │{} {:3}: {:<52} │ │", cursor_marker, line_num, display_line);
        }

        if lines.len() > 8 {
            println!("│ │       ({} more lines...)                                    │ │", lines.len() - 8);
        }

        println!("│ │                                                           │ │");
        println!("│ └──────────────────────────────────────────────────────┘ │");

        // Footer actions
        if !self.read_only {
            println!("│ [Format] [Prettify] [Minify] [Clear]                       │");
        }

        println!("└────────────────────────────────────────────────────────────┘");
    }

    /// Set the content type.
    pub fn set_content_type(&mut self, content_type: BodyContentType) {
        self.content_type = content_type;
        self.dropdown_open = false;

        // Update mode based on content type
        self.mode = match content_type {
            BodyContentType::FormUrlEncoded | BodyContentType::MultipartFormData => {
                EditorMode::Form
            }
            _ => EditorMode::MultiLine,
        };

        println!("📝 Content type set to: {} ({})", content_type.display_name(), content_type.mime_type());
    }

    /// Toggle the content type dropdown.
    pub fn toggle_dropdown(&mut self) {
        self.dropdown_open = !self.dropdown_open;
    }

    /// Set the body content.
    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.line_count = self.content.lines().count();
        self.dirty = true;
    }

    /// Get the body content.
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// Get the content as bytes.
    pub fn get_content_bytes(&self) -> &[u8] {
        self.content.as_bytes()
    }

    /// Get the content length.
    pub fn content_length(&self) -> usize {
        self.content.len()
    }

    /// Update the cursor position.
    pub fn set_cursor(&mut self, line: usize, column: usize) {
        self.cursor = (line, column);
    }

    /// Set the selection.
    pub fn set_selection(&mut self, start: (usize, usize), end: (usize, usize)) {
        self.selection = Some((start, end));
    }

    /// Clear the selection.
    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    /// Insert text at the cursor position.
    pub fn insert_text(&mut self, text: &str) {
        let pos = self.cursor_position();
        self.content.insert_str(pos, text);
        self.dirty = true;
        self.line_count = self.content.lines().count();
    }

    /// Delete text at the cursor position.
    pub fn delete_text(&mut self, count: usize) {
        let pos = self.cursor_position();
        let end = (pos + count).min(self.content.len());
        self.content.replace_range(pos..end, "");
        self.dirty = true;
        self.line_count = self.content.lines().count();
    }

    /// Get the byte position of the cursor.
    pub fn cursor_position(&self) -> usize {
        let lines: Vec<&str> = self.content.lines().collect();
        let mut pos = 0;
        for (i, line) in lines.iter().enumerate() {
            if i == self.cursor.0 {
                return pos + self.cursor.1.min(line.len());
            }
            pos += line.len() + 1; // +1 for newline
        }
        self.content.len()
    }

    /// Format the content (pretty-print JSON, XML, etc.).
    pub fn format_content(&mut self) -> Result<(), String> {
        match self.content_type {
            BodyContentType::Json => {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&self.content) {
                    self.content = serde_json::to_string_pretty(&value)
                        .map_err(|e| e.to_string())?;
                    self.dirty = true;
                    self.line_count = self.content.lines().count();
                    println!("✓ JSON formatted");
                    Ok(())
                } else {
                    Err("Invalid JSON".to_string())
                }
            }
            BodyContentType::Xml => {
                // XML formatting would require an XML library
                println!("ℹ XML formatting not implemented in stub");
                Err("XML formatting not implemented".to_string())
            }
            _ => {
                println!("ℹ Formatting not available for {}", self.content_type.display_name());
                Err("Formatting not available".to_string())
            }
        }
    }

    /// Minify the content (remove unnecessary whitespace).
    pub fn minify_content(&mut self) -> Result<(), String> {
        match self.content_type {
            BodyContentType::Json => {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&self.content) {
                    self.content = serde_json::to_string(&value)
                        .map_err(|e| e.to_string())?;
                    self.dirty = true;
                    self.line_count = self.content.lines().count();
                    println!("✓ JSON minified");
                    Ok(())
                } else {
                    Err("Invalid JSON".to_string())
                }
            }
            _ => {
                println!("ℹ Minify not available for {}", self.content_type.display_name());
                Err("Minify not available".to_string())
            }
        }
    }

    /// Clear the content.
    pub fn clear(&mut self) {
        self.content.clear();
        self.dirty = true;
        self.line_count = 0;
        println!("🗑️ Content cleared");
    }

    /// Load content from a string.
    pub fn load_from_string(&mut self, content: String) {
        self.content = content;
        self.line_count = self.content.lines().count();
        self.dirty = false;
    }

    /// Load content from bytes.
    pub fn load_from_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        match String::from_utf8(bytes.to_vec()) {
            Ok(content) => {
                self.content = content;
                self.line_count = self.content.lines().count();
                self.dirty = false;
                Ok(())
            }
            Err(e) => Err(format!("Invalid UTF-8: {}", e)),
        }
    }

    /// Check if the content is valid for the current content type.
    pub fn validate(&self) -> Result<(), String> {
        match self.content_type {
            BodyContentType::Json => {
                if self.content.trim().is_empty() {
                    return Ok(());
                }
                serde_json::from_str::<serde_json::Value>(&self.content)
                    .map(|_| ())
                    .map_err(|e| format!("Invalid JSON: {}", e))
            }
            BodyContentType::Xml => {
                if self.content.trim().is_empty() {
                    return Ok(());
                }
                // Basic XML validation
                if !self.content.trim().starts_with('<') {
                    return Err("XML must start with '<'".to_string());
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Set read-only mode.
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    /// Mark content as saved (not dirty).
    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    /// Check if content has been modified.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Check if content is empty.
    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
}

impl Default for BodyEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_creation() {
        let editor = BodyEditor::new();
        assert_eq!(editor.content_type, BodyContentType::Json);
        assert!(!editor.read_only);
        assert!(!editor.dirty);
    }

    #[test]
    fn test_set_content_type() {
        let mut editor = BodyEditor::new();
        editor.set_content_type(BodyContentType::Xml);
        assert_eq!(editor.content_type, BodyContentType::Xml);
    }

    #[test]
    fn test_mime_types() {
        assert_eq!(BodyContentType::Json.mime_type(), "application/json");
        assert_eq!(BodyContentType::Xml.mime_type(), "application/xml");
        assert_eq!(BodyContentType::Text.mime_type(), "text/plain");
        assert_eq!(BodyContentType::Html.mime_type(), "text/html");
    }

    #[test]
    fn test_content_length() {
        let mut editor = BodyEditor::new();
        editor.set_content("Hello, World!".to_string());
        assert_eq!(editor.content_length(), 13);
    }

    #[test]
    fn test_clear() {
        let mut editor = BodyEditor::new();
        editor.set_content("Some content".to_string());
        assert!(!editor.is_empty());
        editor.clear();
        assert!(editor.is_empty());
    }

    #[test]
    fn test_validate_json() {
        let mut editor = BodyEditor::new();
        editor.set_content("{\"key\": \"value\"}".to_string());
        assert!(editor.validate().is_ok());

        editor.set_content("{invalid json}".to_string());
        assert!(editor.validate().is_err());
    }

    #[test]
    fn test_is_empty() {
        let mut editor = BodyEditor::with_type(BodyContentType::None);
        assert!(editor.is_empty());

        editor.set_content("   ".to_string()); // whitespace only
        assert!(editor.is_empty());

        editor.set_content("content".to_string());
        assert!(!editor.is_empty());
    }
}
