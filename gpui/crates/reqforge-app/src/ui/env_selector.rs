//! Environment selector component - dropdown for selecting active environment.
//!
//! This is a stub implementation that demonstrates the component structure
//! without requiring GPUI rendering.

use reqforge_core::models::environment::Environment;
use std::collections::HashMap;
use uuid::Uuid;

/// Display option for the environment selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvDisplayOption {
    /// Show environment name only
    NameOnly,
    /// Show environment name with variable count
    WithVarCount,
    /// Show environment name with active indicator
    WithActiveIndicator,
}

/// Environment selector component.
///
/// Provides a dropdown for selecting the active environment from available environments.
/// Shows the currently active environment and allows switching between environments.
pub struct EnvSelector {
    /// Map of all available environments
    pub environments: HashMap<Uuid, Environment>,
    /// Currently active environment ID
    pub active_env_id: Option<Uuid>,
    /// Dropdown open state
    pub dropdown_open: bool,
    /// Focused state for UI feedback
    pub focused: bool,
    /// How to display environment names
    pub display_option: EnvDisplayOption,
    /// Hovered environment ID for keyboard navigation
    pub hovered_env_id: Option<Uuid>,
}

impl EnvSelector {
    /// Create a new environment selector.
    pub fn new() -> Self {
        Self {
            environments: HashMap::new(),
            active_env_id: None,
            dropdown_open: false,
            focused: false,
            display_option: EnvDisplayOption::WithVarCount,
            hovered_env_id: None,
        }
    }

    /// Create an environment selector with initial environments.
    pub fn with_environments(environments: Vec<Environment>) -> Self {
        let mut selector = Self::new();
        for env in environments {
            selector.environments.insert(env.id, env);
        }
        selector
    }

    /// Render the environment selector to console (stub implementation).
    pub fn render(&self) {
        println!();
        println!("┌────────────────────────────────────────────────────────────┐");
        println!("│ Environment Selector                                       │");
        println!("├────────────────────────────────────────────────────────────┤");

        // Current selection
        let active_env = self.active_environment();
        let active_name = active_env.map(|e| e.name.as_str()).unwrap_or("No Environment");
        let active_marker = if self.active_env_id.is_some() { "●" } else { "○" };

        println!("│ Active: {} {}{}                                    │",
            active_marker,
            active_name,
            if self.focused { " ◀" } else { "" }
        );

        println!("├────────────────────────────────────────────────────────────┤");

        if self.dropdown_open {
            println!("│ ▼ Available Environments:                                  │");
            println!("│ ┌────────────────────────────────────────────────────┐  │");

            if self.environments.is_empty() {
                println!("│ │ (No environments configured)                        │  │");
            } else {
                let envs: Vec<_> = self.environments.values().collect();
                for (_i, env) in envs.iter().enumerate() {
                    let is_active = self.active_env_id == Some(env.id);
                    let is_hovered = self.hovered_env_id == Some(env.id);
                    let marker = if is_active { "●" } else if is_hovered { "▶" } else { " " };

                    let display = match self.display_option {
                        EnvDisplayOption::NameOnly => {
                            format!("{}", env.name)
                        }
                        EnvDisplayOption::WithVarCount => {
                            format!("{} ({} vars)", env.name, env.variables.len())
                        }
                        EnvDisplayOption::WithActiveIndicator => {
                            let active_str = if is_active { " [Active]" } else { "" };
                            format!("{}{}", env.name, active_str)
                        }
                    };

                    println!("│ │ {} {:<50} │  │", marker, display);
                }
            }

            println!("│ └────────────────────────────────────────────────────┘  │");
            println!("│ [↑/↓] Navigate  [Enter] Select  [Esc] Close                │");
        } else {
            println!("│ ▼ Press to select environment                            │");
        }

        println!("│                                                          [⚙️] │");
        println!("└────────────────────────────────────────────────────────────┘");
    }

    /// Add or update an environment.
    pub fn set_environment(&mut self, environment: Environment) {
        let id = environment.id;
        self.environments.insert(id, environment);
    }

    /// Remove an environment.
    pub fn remove_environment(&mut self, id: Uuid) -> bool {
        if let Some(env) = self.environments.remove(&id) {
            println!("🗑️ Removed environment: {}", env.name);

            // Clear active if we removed the active environment
            if self.active_env_id == Some(id) {
                self.active_env_id = None;
                println!("⚠ Active environment cleared");
            }
            true
        } else {
            println!("⚠ Environment not found: {}", id);
            false
        }
    }

    /// Set the active environment by ID.
    pub fn set_active(&mut self, id: Uuid) -> bool {
        if self.environments.contains_key(&id) {
            self.active_env_id = Some(id);
            if let Some(env) = self.environments.get(&id) {
                println!("✓ Activated environment: {}", env.name);
            }
            self.dropdown_open = false;
            true
        } else {
            println!("⚠ Environment not found: {}", id);
            false
        }
    }

    /// Clear the active environment.
    pub fn clear_active(&mut self) {
        self.active_env_id = None;
        println!("⚠ Active environment cleared");
    }

    /// Get the active environment.
    pub fn active_environment(&self) -> Option<&Environment> {
        self.active_env_id
            .and_then(|id| self.environments.get(&id))
    }

    /// Get the active environment's variables as a map.
    pub fn active_variables(&self) -> HashMap<String, String> {
        self.active_environment()
            .map(|env| env.to_map())
            .unwrap_or_default()
    }

    /// Toggle the dropdown open/closed.
    pub fn toggle_dropdown(&mut self) {
        self.dropdown_open = !self.dropdown_open;
        if self.dropdown_open {
            // Set hovered to active environment when opening
            self.hovered_env_id = self.active_env_id;
        }
    }

    /// Open the dropdown.
    pub fn open_dropdown(&mut self) {
        self.dropdown_open = true;
        self.hovered_env_id = self.active_env_id;
    }

    /// Close the dropdown.
    pub fn close_dropdown(&mut self) {
        self.dropdown_open = false;
        self.hovered_env_id = None;
    }

    /// Set the focused state.
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Navigate to the next environment in the list.
    pub fn navigate_next(&mut self) {
        let env_ids: Vec<Uuid> = self.environments.keys().copied().collect();
        if env_ids.is_empty() {
            return;
        }

        let current_idx = self.hovered_env_id
            .and_then(|id| env_ids.iter().position(|&x| x == id))
            .unwrap_or(0);

        let next_idx = (current_idx + 1) % env_ids.len();
        self.hovered_env_id = Some(env_ids[next_idx]);
    }

    /// Navigate to the previous environment in the list.
    pub fn navigate_prev(&mut self) {
        let env_ids: Vec<Uuid> = self.environments.keys().copied().collect();
        if env_ids.is_empty() {
            return;
        }

        let current_idx = self.hovered_env_id
            .and_then(|id| env_ids.iter().position(|&x| x == id))
            .unwrap_or(0);

        let prev_idx = if current_idx == 0 {
            env_ids.len() - 1
        } else {
            current_idx - 1
        };
        self.hovered_env_id = Some(env_ids[prev_idx]);
    }

    /// Select the hovered environment.
    pub fn select_hovered(&mut self) -> bool {
        if let Some(id) = self.hovered_env_id {
            self.set_active(id)
        } else {
            false
        }
    }

    /// Set the display option.
    pub fn set_display_option(&mut self, option: EnvDisplayOption) {
        self.display_option = option;
    }

    /// Get all environment IDs.
    pub fn environment_ids(&self) -> Vec<Uuid> {
        self.environments.keys().copied().collect()
    }

    /// Get all environments.
    pub fn environments(&self) -> Vec<&Environment> {
        self.environments.values().collect()
    }

    /// Check if an environment is active.
    pub fn is_active(&self, id: Uuid) -> bool {
        self.active_env_id == Some(id)
    }

    /// Get the count of environments.
    pub fn count(&self) -> usize {
        self.environments.len()
    }

    /// Check if there are any environments.
    pub fn is_empty(&self) -> bool {
        self.environments.is_empty()
    }

    /// Get a variable value from the active environment.
    pub fn get_variable(&self, key: &str) -> Option<String> {
        self.active_environment()
            .and_then(|env| {
                env.variables
                    .iter()
                    .find(|v| v.enabled && v.key == key)
                    .map(|v| v.value.clone())
            })
    }

    /// Get all variable keys from the active environment.
    pub fn variable_keys(&self) -> Vec<String> {
        self.active_environment()
            .map(|env| {
                env.variables
                    .iter()
                    .filter(|v| v.enabled)
                    .map(|v| v.key.clone())
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl Default for EnvSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqforge_core::models::environment::Variable;

    #[test]
    fn test_selector_creation() {
        let selector = EnvSelector::new();
        assert!(selector.is_empty());
        assert!(selector.active_env_id.is_none());
    }

    #[test]
    fn test_set_environment() {
        let mut selector = EnvSelector::new();
        let env = Environment::new("Development");
        let id = env.id;
        selector.set_environment(env);
        assert_eq!(selector.count(), 1);
        assert!(selector.environments.contains_key(&id));
    }

    #[test]
    fn test_set_active() {
        let mut selector = EnvSelector::new();
        let env = Environment::new("Production");
        let id = env.id;
        selector.set_environment(env);
        assert!(selector.set_active(id));
        assert_eq!(selector.active_env_id, Some(id));
    }

    #[test]
    fn test_clear_active() {
        let mut selector = EnvSelector::new();
        let env = Environment::new("Staging");
        let id = env.id;
        selector.set_environment(env);
        selector.set_active(id);
        selector.clear_active();
        assert!(selector.active_env_id.is_none());
    }

    #[test]
    fn test_remove_environment() {
        let mut selector = EnvSelector::new();
        let env = Environment::new("Test");
        let id = env.id;
        selector.set_environment(env);
        selector.set_active(id);
        assert!(selector.remove_environment(id));
        assert!(selector.active_env_id.is_none());
        assert_eq!(selector.count(), 0);
    }

    #[test]
    fn test_toggle_dropdown() {
        let mut selector = EnvSelector::new();
        assert!(!selector.dropdown_open);
        selector.toggle_dropdown();
        assert!(selector.dropdown_open);
        selector.toggle_dropdown();
        assert!(!selector.dropdown_open);
    }

    #[test]
    fn test_get_variable() {
        let mut selector = EnvSelector::new();
        let mut env = Environment::new("Dev");
        env.variables.push(Variable {
            key: "API_URL".to_string(),
            value: "https://api.dev.com".to_string(),
            enabled: true,
            secret: false,
        });
        let id = env.id;
        selector.set_environment(env);
        selector.set_active(id);

        assert_eq!(
            selector.get_variable("API_URL"),
            Some("https://api.dev.com".to_string())
        );
        assert_eq!(selector.get_variable("NONEXISTENT"), None);
    }

    #[test]
    fn test_is_active() {
        let mut selector = EnvSelector::new();
        let env = Environment::new("Prod");
        let id = env.id;
        selector.set_environment(env);

        assert!(!selector.is_active(id));
        selector.set_active(id);
        assert!(selector.is_active(id));
    }

    #[test]
    fn test_active_variables() {
        let mut selector = EnvSelector::new();
        let mut env = Environment::new("Test");
        env.variables.push(Variable {
            key: "KEY1".to_string(),
            value: "value1".to_string(),
            enabled: true,
            secret: false,
        });
        env.variables.push(Variable {
            key: "KEY2".to_string(),
            value: "value2".to_string(),
            enabled: false, // disabled
            secret: false,
        });
        let id = env.id;
        selector.set_environment(env);
        selector.set_active(id);

        let vars = selector.active_variables();
        assert_eq!(vars.len(), 1);
        assert_eq!(vars.get("KEY1"), Some(&"value1".to_string()));
        assert_eq!(vars.get("KEY2"), None);
    }
}
