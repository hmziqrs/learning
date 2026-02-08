//! Environment editor modal - modal for CRUD on environments and variables.
//!
//! This is a stub implementation that demonstrates the component structure
//! without requiring GPUI rendering.

use reqforge_core::models::environment::{Environment, Variable};
use uuid::Uuid;
use std::collections::HashMap;

/// The current mode of the environment editor modal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvEditorMode {
    /// Viewing environment list
    ViewList,
    /// Creating a new environment
    CreateEnvironment,
    /// Editing an existing environment
    EditEnvironment,
    /// Deleting an environment (with confirmation)
    DeleteEnvironment,
    /// Creating a new variable
    CreateVariable,
    /// Editing an existing variable
    EditVariable,
    /// Deleting a variable (with confirmation)
    DeleteVariable,
}

/// Modal state for user interactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalState {
    /// Modal is closed
    Closed,
    /// Modal is open and visible
    Open,
    /// Modal is in a transition state
    Transitioning,
}

/// Environment editor modal component.
///
/// Provides a modal interface for creating, reading, updating, and deleting
/// environments and their variables.
pub struct EnvEditorModal {
    /// All environments being managed
    pub environments: HashMap<Uuid, Environment>,
    /// Currently selected environment ID
    pub selected_env_id: Option<Uuid>,
    /// Currently editing environment (for Create/Edit modes)
    pub editing_env: Option<Environment>,
    /// Currently selected variable index
    pub selected_var_index: Option<usize>,
    /// Currently editing variable (for Create/Edit modes)
    pub editing_var: Option<Variable>,
    /// Current editor mode
    pub mode: EnvEditorMode,
    /// Modal state
    pub state: ModalState,
    /// Input buffer for text fields
    pub input_buffer: String,
    /// Whether the modal can be closed
    pub closeable: bool,
    /// Whether there are unsaved changes
    pub has_unsaved_changes: bool,
    /// Error message to display
    pub error_message: Option<String>,
    /// Success message to display
    pub success_message: Option<String>,
}

impl EnvEditorModal {
    /// Create a new environment editor modal.
    pub fn new() -> Self {
        Self {
            environments: HashMap::new(),
            selected_env_id: None,
            editing_env: None,
            selected_var_index: None,
            editing_var: None,
            mode: EnvEditorMode::ViewList,
            state: ModalState::Closed,
            input_buffer: String::new(),
            closeable: true,
            has_unsaved_changes: false,
            error_message: None,
            success_message: None,
        }
    }

    /// Create a modal with initial environments.
    pub fn with_environments(environments: Vec<Environment>) -> Self {
        let mut modal = Self::new();
        for env in environments {
            modal.environments.insert(env.id, env);
        }
        modal
    }

    /// Render the modal to console (stub implementation).
    pub fn render(&self) {
        if self.state == ModalState::Closed {
            println!();
            println!("  [Modal Closed]");
            return;
        }

        println!();
        println!("┌────────────────────────────────────────────────────────────┐");
        println!("│                    Environment Editor                       │");
        println!("├────────────────────────────────────────────────────────────┤");

        // Show messages
        if let Some(ref error) = self.error_message {
            println!("│ ⚠ Error: {}", format!("{:<52}", error));
            println!("├────────────────────────────────────────────────────────────┤");
        } else if let Some(ref success) = self.success_message {
            println!("│ ✓ {}", format!("{:<54}", success));
            println!("├────────────────────────────────────────────────────────────┤");
        }

        match self.mode {
            EnvEditorMode::ViewList => {
                self.render_view_list();
            }
            EnvEditorMode::CreateEnvironment => {
                self.render_create_environment();
            }
            EnvEditorMode::EditEnvironment => {
                self.render_edit_environment();
            }
            EnvEditorMode::DeleteEnvironment => {
                self.render_delete_environment();
            }
            EnvEditorMode::CreateVariable => {
                self.render_create_variable();
            }
            EnvEditorMode::EditVariable => {
                self.render_edit_variable();
            }
            EnvEditorMode::DeleteVariable => {
                self.render_delete_variable();
            }
        }

        println!("├────────────────────────────────────────────────────────────┤");

        // Footer with actions
        if self.closeable {
            println!("│ [Esc] Close  [Enter] Confirm                                 │");
        } else {
            println!("│ [Enter] Confirm (must resolve to close)                     │");
        }

        println!("└────────────────────────────────────────────────────────────┘");

        // Unsaved changes indicator
        if self.has_unsaved_changes {
            println!("  ⚠ You have unsaved changes");
        }
    }

    /// Render the environment list view.
    fn render_view_list(&self) {
        println!("│ Environments:                                            [+ New]│");
        println!("│ ┌──────────────────────────────────────────────────────┐ │");

        if self.environments.is_empty() {
            println!("│ │ No environments configured.                              │ │");
        } else {
            for (_i, env) in self.environments.values().enumerate() {
                let is_selected = self.selected_env_id == Some(env.id);
                let marker = if is_selected { "●" } else { " " };
                let active_marker = if is_selected { "◄" } else { "" };

                println!("│ │ {} {} ({} vars){:34} │ │",
                    marker,
                    env.name,
                    env.variables.len(),
                    active_marker
                );
            }
        }

        println!("│ └──────────────────────────────────────────────────────┘ │");
        println!("│                                                          │ │");
        println!("│ [Enter] Edit  [N] New  [D] Delete  [V] View Variables    │ │");
    }

    /// Render the create environment form.
    fn render_create_environment(&self) {
        println!("│ Create New Environment                                    │");
        println!("│ ┌──────────────────────────────────────────────────────┐ │");
        println!("│ │ Name:                                                 │ │");
        println!("│ │ ┌────────────────────────────────────────────────┐   │ │");
        println!("│ │ │ {:<50} │   │ │", self.input_buffer);
        println!("│ │ └────────────────────────────────────────────────┘   │ │");
        println!("│ └──────────────────────────────────────────────────────┘ │");
        println!("│                                                          │ │");
        println!("│ [Enter] Save  [Esc] Cancel                                 │");
    }

    /// Render the edit environment form.
    fn render_edit_environment(&self) {
        let env_name = self.editing_env
            .as_ref()
            .map(|e| e.name.as_str())
            .unwrap_or("");

        println!("│ Edit Environment                                          │");
        println!("│ ┌──────────────────────────────────────────────────────┐ │");
        println!("│ │ Name: {:<48} │ │", env_name);
        println!("│ └──────────────────────────────────────────────────────┘ │");
        println!("│                                                          │ │");
        println!("│ Variables:                                               │ │");
        println!("│ ┌──────────────────────────────────────────────────────┐ │");

        let variables = self.editing_env
            .as_ref()
            .map(|e| e.variables.as_slice())
            .unwrap_or(&[]);

        if variables.is_empty() {
            println!("│ │ No variables.                             [+ Add]   │ │");
        } else {
            for (i, var) in variables.iter().enumerate() {
                let is_selected = self.selected_var_index == Some(i);
                let marker = if is_selected { "►" } else { " " };
                let secret_marker = if var.secret { "🔒" } else { "" };
                let enabled_marker = if var.enabled { "☑" } else { "☐" };
                let display_value = if var.secret && !var.value.is_empty() {
                    "••••••••"
                } else {
                    var.value.as_str()
                };

                println!("│ │ {} {} {} = {}{:36} │ │",
                    marker,
                    enabled_marker,
                    var.key,
                    display_value,
                    secret_marker
                );
            }
            println!("│ │                                                        │ │");
            println!("│ │                                                 [+ Add] │ │");
        }

        println!("│ └──────────────────────────────────────────────────────┘ │");
        println!("│                                                          │ │");
        println!("│ [S] Save  [N] New Var  [E] Edit Var  [D] Delete Var       │");
        println!("│ [Enter] Select Variable  [Esc] Cancel                     │");
    }

    /// Render the delete environment confirmation.
    fn render_delete_environment(&self) {
        let env_name = self.editing_env
            .as_ref()
            .map(|e| e.name.as_str())
            .unwrap_or("this environment");

        println!("│ Delete Environment?                                       │");
        println!("│ ┌──────────────────────────────────────────────────────┐ │");
        println!("│ │                                                        │ │");
        println!("│ │   Are you sure you want to delete \"{}\"?               │ │", env_name);
        println!("│ │                                                        │ │");
        println!("│ │   This action cannot be undone.                       │ │");
        println!("│ │                                                        │ │");
        println!("│ └──────────────────────────────────────────────────────┘ │");
        println!("│                                                          │ │");
        println!("│ [Enter] Confirm Delete  [Esc] Cancel                      │");
    }

    /// Render the create variable form.
    fn render_create_variable(&self) {
        println!("│ Add Variable                                             │");
        println!("│ ┌──────────────────────────────────────────────────────┐ │");
        println!("│ │ Key:   {:<49} │ │", self.input_buffer);
        println!("│ │ Value: {:<48} │ │", "");
        println!("│ └──────────────────────────────────────────────────────┘ │");
        println!("│                                                          │ │");
        println!("│ [☑] Enabled  [☐] Secret                                   │");
        println!("│                                                          │ │");
        println!("│ [Enter] Save  [Esc] Cancel                                 │");
    }

    /// Render the edit variable form.
    fn render_edit_variable(&self) {
        let var = self.editing_var.as_ref();

        let key = var.map(|v| v.key.as_str()).unwrap_or("");
        let value = var.map(|v| {
            if v.secret && !v.value.is_empty() {
                "••••••••"
            } else {
                v.value.as_str()
            }
        }).unwrap_or("");
        let enabled = var.map(|v| v.enabled).unwrap_or(false);
        let secret = var.map(|v| v.secret).unwrap_or(false);

        println!("│ Edit Variable                                            │");
        println!("│ ┌──────────────────────────────────────────────────────┐ │");
        println!("│ │ Key:   {:<49} │ │", key);
        println!("│ │ Value: {:<48} │ │", value);
        println!("│ └──────────────────────────────────────────────────────┘ │");
        println!("│                                                          │ │");
        println!("│ [{}] Enabled  [{}] Secret                                 │",
            if enabled { "☑" } else { "☐" },
            if secret { "☑" } else { "☐" }
        );
        println!("│                                                          │ │");
        println!("│ [S] Save  [Esc] Cancel                                    │");
    }

    /// Render the delete variable confirmation.
    fn render_delete_variable(&self) {
        let var_key = self.editing_var
            .as_ref()
            .map(|v| v.key.as_str())
            .unwrap_or("this variable");

        println!("│ Delete Variable?                                         │");
        println!("│ ┌──────────────────────────────────────────────────────┐ │");
        println!("│ │                                                        │ │");
        println!("│ │   Are you sure you want to delete \"{}\"?                  │ │", var_key);
        println!("│ │                                                        │ │");
        println!("│ └──────────────────────────────────────────────────────┘ │");
        println!("│                                                          │ │");
        println!("│ [Enter] Confirm Delete  [Esc] Cancel                      │");
    }

    /// Open the modal in a specific mode.
    pub fn open(&mut self, mode: EnvEditorMode) {
        self.state = ModalState::Open;
        self.mode = mode;
        self.error_message = None;
        self.success_message = None;

        match mode {
            EnvEditorMode::CreateEnvironment => {
                self.editing_env = Some(Environment::new(""));
                self.input_buffer = String::new();
            }
            EnvEditorMode::CreateVariable => {
                self.editing_var = Some(Variable {
                    key: String::new(),
                    value: String::new(),
                    enabled: true,
                    secret: false,
                });
                self.input_buffer = String::new();
            }
            _ => {}
        }

        println!("📋 Opened modal in {:?}", mode);
    }

    /// Close the modal.
    pub fn close(&mut self) {
        if self.closeable {
            self.state = ModalState::Closed;
            self.clear_state();
            println!("📋 Modal closed");
        } else {
            println!("⚠ Cannot close modal with unsaved changes");
        }
    }

    /// Clear modal state.
    fn clear_state(&mut self) {
        self.editing_env = None;
        self.editing_var = None;
        self.selected_var_index = None;
        self.input_buffer.clear();
        self.error_message = None;
        self.success_message = None;
        self.has_unsaved_changes = false;
    }

    /// Create a new environment.
    pub fn create_environment(&mut self, name: String) -> Result<Uuid, String> {
        if name.trim().is_empty() {
            return Err("Environment name cannot be empty".to_string());
        }

        // Check for duplicate names
        if self.environments.values().any(|e| e.name == name) {
            return Err(format!("Environment '{}' already exists", name));
        }

        let env = Environment::new(&name);
        let id = env.id;
        self.environments.insert(id, env);
        self.has_unsaved_changes = true;
        self.success_message = Some(format!("Created environment '{}'", name));

        println!("✓ Created environment: {}", name);
        Ok(id)
    }

    /// Update an existing environment.
    pub fn update_environment(&mut self, id: Uuid, name: String) -> Result<(), String> {
        if let Some(env) = self.environments.get_mut(&id) {
            if name.trim().is_empty() {
                return Err("Environment name cannot be empty".to_string());
            }
            env.name = name;
            self.has_unsaved_changes = true;
            self.success_message = Some("Environment updated".to_string());
            println!("✓ Updated environment: {}", env.name);
            Ok(())
        } else {
            Err("Environment not found".to_string())
        }
    }

    /// Delete an environment.
    pub fn delete_environment(&mut self, id: Uuid) -> Result<(), String> {
        if let Some(env) = self.environments.remove(&id) {
            if self.selected_env_id == Some(id) {
                self.selected_env_id = None;
            }
            self.has_unsaved_changes = true;
            self.success_message = Some(format!("Deleted environment '{}'", env.name));
            println!("✓ Deleted environment: {}", env.name);
            Ok(())
        } else {
            Err("Environment not found".to_string())
        }
    }

    /// Add a variable to an environment.
    pub fn add_variable(&mut self, env_id: Uuid, variable: Variable) -> Result<(), String> {
        if let Some(env) = self.environments.get_mut(&env_id) {
            let key = variable.key.clone();
            if key.trim().is_empty() {
                return Err("Variable key cannot be empty".to_string());
            }

            // Check for duplicate keys
            if env.variables.iter().any(|v| v.key == key) {
                return Err(format!("Variable '{}' already exists", key));
            }

            env.variables.push(variable);
            self.has_unsaved_changes = true;
            println!("✓ Added variable: {} to {}", key, env.name);
            Ok(())
        } else {
            Err("Environment not found".to_string())
        }
    }

    /// Update a variable in an environment.
    pub fn update_variable(&mut self, env_id: Uuid, index: usize, variable: Variable) -> Result<(), String> {
        if let Some(env) = self.environments.get_mut(&env_id) {
            if index < env.variables.len() {
                if variable.key.trim().is_empty() {
                    return Err("Variable key cannot be empty".to_string());
                }
                env.variables[index] = variable;
                self.has_unsaved_changes = true;
                println!("✓ Updated variable at index {}", index);
                Ok(())
            } else {
                Err("Variable index out of bounds".to_string())
            }
        } else {
            Err("Environment not found".to_string())
        }
    }

    /// Remove a variable from an environment.
    pub fn remove_variable(&mut self, env_id: Uuid, index: usize) -> Result<(), String> {
        if let Some(env) = self.environments.get_mut(&env_id) {
            if index < env.variables.len() {
                let var = env.variables.remove(index);
                self.has_unsaved_changes = true;
                println!("✓ Removed variable: {} from {}", var.key, env.name);
                Ok(())
            } else {
                Err("Variable index out of bounds".to_string())
            }
        } else {
            Err("Environment not found".to_string())
        }
    }

    /// Select an environment.
    pub fn select_environment(&mut self, id: Uuid) -> bool {
        if self.environments.contains_key(&id) {
            self.selected_env_id = Some(id);
            true
        } else {
            false
        }
    }

    /// Select a variable by index.
    pub fn select_variable(&mut self, index: usize) -> bool {
        if let Some(env_id) = self.selected_env_id {
            if let Some(env) = self.environments.get(&env_id) {
                if index < env.variables.len() {
                    self.selected_var_index = Some(index);
                    return true;
                }
            }
        }
        false
    }

    /// Get the selected environment.
    pub fn selected_environment(&self) -> Option<&Environment> {
        self.selected_env_id
            .and_then(|id| self.environments.get(&id))
    }

    /// Set an error message.
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }

    /// Clear the error message.
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    /// Set the closeable state.
    pub fn set_closeable(&mut self, closeable: bool) {
        self.closeable = closeable;
    }

    /// Get all environment IDs.
    pub fn environment_ids(&self) -> Vec<Uuid> {
        self.environments.keys().copied().collect()
    }

    /// Get the count of environments.
    pub fn count(&self) -> usize {
        self.environments.len()
    }
}

impl Default for EnvEditorModal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_creation() {
        let modal = EnvEditorModal::new();
        assert_eq!(modal.state, ModalState::Closed);
        assert!(modal.environments.is_empty());
    }

    #[test]
    fn test_open_close() {
        let mut modal = EnvEditorModal::new();
        modal.open(EnvEditorMode::ViewList);
        assert_eq!(modal.state, ModalState::Open);
        modal.close();
        assert_eq!(modal.state, ModalState::Closed);
    }

    #[test]
    fn test_create_environment() {
        let mut modal = EnvEditorModal::new();
        let id = modal.create_environment("Production".to_string()).unwrap();
        assert_eq!(modal.count(), 1);
        assert!(modal.environments.contains_key(&id));
    }

    #[test]
    fn test_create_environment_empty_name() {
        let mut modal = EnvEditorModal::new();
        let result = modal.create_environment("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_environment() {
        let mut modal = EnvEditorModal::new();
        let id = modal.create_environment("Staging".to_string()).unwrap();
        assert!(modal.delete_environment(id).is_ok());
        assert_eq!(modal.count(), 0);
    }

    #[test]
    fn test_select_environment() {
        let mut modal = EnvEditorModal::new();
        let id = modal.create_environment("Dev".to_string()).unwrap();
        assert!(modal.select_environment(id));
        assert_eq!(modal.selected_env_id, Some(id));
    }

    #[test]
    fn test_add_variable() {
        let mut modal = EnvEditorModal::new();
        let env_id = modal.create_environment("Test".to_string()).unwrap();

        let var = Variable {
            key: "API_KEY".to_string(),
            value: "secret123".to_string(),
            enabled: true,
            secret: true,
        };

        assert!(modal.add_variable(env_id, var).is_ok());
        let env = modal.environments.get(&env_id).unwrap();
        assert_eq!(env.variables.len(), 1);
    }

    #[test]
    fn test_remove_variable() {
        let mut modal = EnvEditorModal::new();
        let env_id = modal.create_environment("Prod".to_string()).unwrap();

        let var = Variable {
            key: "URL".to_string(),
            value: "https://api.com".to_string(),
            enabled: true,
            secret: false,
        };

        modal.add_variable(env_id, var).unwrap();
        assert!(modal.remove_variable(env_id, 0).is_ok());

        let env = modal.environments.get(&env_id).unwrap();
        assert_eq!(env.variables.len(), 0);
    }

    #[test]
    fn test_unsaved_changes_flag() {
        let mut modal = EnvEditorModal::new();
        assert!(!modal.has_unsaved_changes);

        modal.create_environment("NewEnv".to_string()).unwrap();
        assert!(modal.has_unsaved_changes);
    }
}
