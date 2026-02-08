//! Tab bar component - displays open request tabs.
//!
//! This is a stub implementation that demonstrates the component structure
//! without requiring GPUI rendering.

use reqforge_core::models::request::RequestDefinition;
use uuid::Uuid;

/// Represents an open tab in the editor.
#[derive(Debug, Clone)]
pub struct OpenTab {
    /// Unique tab ID
    pub id: Uuid,
    /// The request definition being edited
    pub draft: RequestDefinition,
    /// Collection this request belongs to
    pub collection_id: Uuid,
    /// Whether the tab has unsaved changes
    pub is_dirty: bool,
    /// Whether this tab is currently loading
    pub is_loading: bool,
    /// Display title (may differ from draft name)
    pub title: String,
}

impl OpenTab {
    /// Create a new open tab.
    pub fn new(id: Uuid, draft: RequestDefinition, collection_id: Uuid) -> Self {
        let title = draft.name.clone();
        Self {
            id,
            draft,
            collection_id,
            is_dirty: false,
            is_loading: false,
            title,
        }
    }

    /// Get the display title with dirty indicator.
    pub fn display_title(&self) -> String {
        if self.is_dirty {
            format!("● {}", self.title)
        } else {
            self.title.clone()
        }
    }

    /// Get a short title for display in tabs (truncated if needed).
    pub fn short_title(&self, max_len: usize) -> String {
        let title = self.display_title();
        if title.len() <= max_len {
            title
        } else {
            format!("{}...", &title[..max_len.saturating_sub(3)])
        }
    }
}

/// Tab bar component.
///
/// Displays horizontal tabs for open requests with close buttons
/// and dirty indicators. Handles tab switching and closing.
pub struct TabBar {
    /// All open tabs
    pub tabs: Vec<OpenTab>,
    /// Currently active tab index
    pub active_index: Option<usize>,
    /// Hovered tab index for UI feedback
    pub hovered_index: Option<usize>,
    /// Maximum tab width in characters
    pub max_tab_width: usize,
}

impl TabBar {
    /// Create a new tab bar.
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_index: None,
            hovered_index: None,
            max_tab_width: 25,
        }
    }

    /// Add a new tab.
    pub fn add_tab(&mut self, tab: OpenTab) {
        self.tabs.push(tab);
        // Auto-switch to the new tab
        self.active_index = Some(self.tabs.len() - 1);
    }

    /// Create and add a tab from a request definition.
    pub fn open_request(&mut self, request: RequestDefinition, collection_id: Uuid) -> Uuid {
        let tab_id = Uuid::new_v4();
        let tab = OpenTab::new(tab_id, request, collection_id);
        self.add_tab(tab);
        tab_id
    }

    /// Close a tab by index.
    pub fn close_tab(&mut self, index: usize) -> Option<OpenTab> {
        if index >= self.tabs.len() {
            return None;
        }

        let removed = self.tabs.remove(index);

        // Adjust active index
        match self.active_index {
            Some(active) if active == index => {
                // Closed the active tab
                if self.tabs.is_empty() {
                    self.active_index = None;
                } else if index >= self.tabs.len() {
                    self.active_index = Some(self.tabs.len() - 1);
                } else {
                    self.active_index = Some(index);
                }
            }
            Some(active) if active > index => {
                // Active tab is after closed tab, adjust index
                self.active_index = Some(active - 1);
            }
            _ => {}
        }

        Some(removed)
    }

    /// Close the active tab.
    pub fn close_active(&mut self) -> Option<OpenTab> {
        if let Some(index) = self.active_index {
            self.close_tab(index)
        } else {
            None
        }
    }

    /// Switch to a tab by index.
    pub fn switch_to(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active_index = Some(index);
            true
        } else {
            false
        }
    }

    /// Get the active tab.
    pub fn active_tab(&self) -> Option<&OpenTab> {
        self.active_index
            .and_then(|i| self.tabs.get(i))
    }

    /// Get a mutable reference to the active tab.
    pub fn active_tab_mut(&mut self) -> Option<&mut OpenTab> {
        self.active_index
            .and_then(move |i| self.tabs.get_mut(i))
    }

    /// Mark the active tab as dirty.
    pub fn mark_dirty(&mut self) {
        if let Some(tab) = self.active_tab_mut() {
            tab.is_dirty = true;
        }
    }

    /// Mark the active tab as clean (saved).
    pub fn mark_clean(&mut self) {
        if let Some(tab) = self.active_tab_mut() {
            tab.is_dirty = false;
        }
    }

    /// Update the active tab's draft.
    pub fn update_draft<F>(&mut self, updater: F)
    where
        F: FnOnce(&mut RequestDefinition),
    {
        if let Some(tab) = self.active_tab_mut() {
            updater(&mut tab.draft);
            tab.is_dirty = true;
            tab.title = tab.draft.name.clone();
        }
    }

    /// Set loading state for the active tab.
    pub fn set_loading(&mut self, loading: bool) {
        if let Some(tab) = self.active_tab_mut() {
            tab.is_loading = loading;
        }
    }

    /// Get a tab by index.
    pub fn get_tab(&self, index: usize) -> Option<&OpenTab> {
        self.tabs.get(index)
    }

    /// Find a tab by request ID.
    pub fn find_by_request_id(&self, request_id: Uuid) -> Option<&OpenTab> {
        self.tabs.iter().find(|t| t.draft.id == request_id)
    }

    /// Find a tab index by request ID.
    pub fn find_index_by_request_id(&self, request_id: Uuid) -> Option<usize> {
        self.tabs
            .iter()
            .position(|t| t.draft.id == request_id)
    }

    /// Render the tab bar to console (stub implementation).
    pub fn render(&self) {
        println!();
        println!("┌────────────────────────────────────────────────────────────┐");
        print!("│");

        if self.tabs.is_empty() {
            print!(" (No tabs open) ");
            // Padding to edge
            let padding = 58;
            print!("{:width$}", "", width = padding);
        } else {
            let mut position = 0;
            for (i, tab) in self.tabs.iter().enumerate() {
                let is_active = self.active_index == Some(i);
                let is_hovered = self.hovered_index == Some(i);

                // Tab decorations
                let left = if is_active { "[" } else { " " };
                let right = if is_active { "]" } else { " " };
                let dirty = if tab.is_dirty { "●" } else { " " };
                let loading = if tab.is_loading { "⟳" } else { "" };
                let close = if is_hovered { "×" } else { " " };

                let title = tab.short_title(self.max_tab_width);
                let tab_text = format!("{} {}{}{} {}{}", left, title, dirty, loading, close, right);

                // Check if we need a new line
                if position + tab_text.len() > 60 {
                    println!(" │");
                    print!("│");
                    position = 0;
                }

                print!("{}", tab_text);
                position += tab_text.len();
            }

            // Padding to edge
            if position < 60 {
                print!("{:width$}", "", width = 60 - position);
            }
        }

        println!("│");
        println!("└────────────────────────────────────────────────────────────┘");
    }

    /// Handle tab click.
    pub fn on_tab_click(&mut self, index: usize) {
        if index < self.tabs.len() {
            println!();
            println!("🖱️ Tab clicked: {} ({})", index, self.tabs[index].title);
            self.switch_to(index);
        }
    }

    /// Handle tab close button click.
    pub fn on_tab_close(&mut self, index: usize) {
        if let Some(tab) = self.close_tab(index) {
            println!();
            println!("✕ Tab closed: {}", tab.title);
        }
    }

    /// Handle tab hover.
    pub fn on_tab_hover(&mut self, index: Option<usize>) {
        self.hovered_index = index;
    }

    /// Get the number of open tabs.
    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    /// Check if there are no tabs.
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
}

impl Default for TabBar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqforge_core::models::request::HttpMethod;

    fn create_test_request(name: &str) -> RequestDefinition {
        RequestDefinition::new(
            name,
            HttpMethod::GET,
            "https://example.com",
        )
    }

    #[test]
    fn test_tab_bar_creation() {
        let bar = TabBar::new();
        assert!(bar.is_empty());
        assert_eq!(bar.active_index, None);
    }

    #[test]
    fn test_add_tab() {
        let mut bar = TabBar::new();
        let request = create_test_request("Test");

        bar.open_request(request, Uuid::new_v4());

        assert_eq!(bar.len(), 1);
        assert_eq!(bar.active_index, Some(0));
    }

    #[test]
    fn test_close_tab() {
        let mut bar = TabBar::new();
        let request = create_test_request("Test");
        let collection_id = Uuid::new_v4();

        let tab_id = bar.open_request(request, collection_id);
        assert_eq!(bar.len(), 1);

        bar.close_tab(0);
        assert_eq!(bar.len(), 0);
        assert_eq!(bar.active_index, None);
    }

    #[test]
    fn test_mark_dirty() {
        let mut bar = TabBar::new();
        let request = create_test_request("Test");

        bar.open_request(request, Uuid::new_v4());
        bar.mark_dirty();

        assert!(bar.active_tab().unwrap().is_dirty);
    }

    #[test]
    fn test_switch_to() {
        let mut bar = TabBar::new();
        let r1 = create_test_request("Tab 1");
        let r2 = create_test_request("Tab 2");

        bar.open_request(r1, Uuid::new_v4());
        bar.open_request(r2, Uuid::new_v4());

        assert!(bar.switch_to(0));
        assert_eq!(bar.active_index, Some(0));
        assert_eq!(bar.active_tab().unwrap().title, "Tab 1");

        assert!(bar.switch_to(1));
        assert_eq!(bar.active_index, Some(1));
        assert_eq!(bar.active_tab().unwrap().title, "Tab 2");

        assert!(!bar.switch_to(5)); // Invalid index
    }
}
