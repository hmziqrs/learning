//! Environment editor modal - modal for CRUD on environments and variables.
//!
//! This modal provides a two-panel interface:
//! - Left panel: List of environments with Add/Remove/Rename buttons
//! - Right panel: Variable table with key, value, enabled, and secret columns

use gpui::{
    div, px, App, Context, Entity, EventEmitter, IntoElement, InteractiveElement,
    KeyBinding, ParentElement, Render, SharedString, Styled, Window, actions, MouseButton,
};
use gpui_component::{
    button::Button,
    checkbox::Checkbox,
    h_flex, list, v_flex, ActiveTheme, Disableable, StyledExt, switch::Switch,
};
use reqforge_core::models::environment::{Environment, Variable};
use uuid::Uuid;

/// Context key for environment editor modal keyboard shortcuts
const ENV_EDITOR_CONTEXT: &str = "EnvEditorModal";

actions!(env_editor, [Save, Cancel, AddEnv, RemoveEnv, RenameEnv, AddVar, RemoveVar]);

/// Initialize environment editor keyboard bindings.
pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("cmd-s", Save, Some(ENV_EDITOR_CONTEXT)),
        KeyBinding::new("escape", Cancel, Some(ENV_EDITOR_CONTEXT)),
        KeyBinding::new("cmd-n", AddEnv, Some(ENV_EDITOR_CONTEXT)),
        KeyBinding::new("backspace", RemoveEnv, Some(ENV_EDITOR_CONTEXT)),
        KeyBinding::new("f2", RenameEnv, Some(ENV_EDITOR_CONTEXT)),
        KeyBinding::new("cmd-shift-n", AddVar, Some(ENV_EDITOR_CONTEXT)),
        KeyBinding::new("cmd-backspace", RemoveVar, Some(ENV_EDITOR_CONTEXT)),
    ]);
}

/// A variable row in the variable table.
#[derive(Clone)]
pub struct VariableRow {
    /// The variable data
    pub variable: Variable,
    /// Unique identifier for this row
    pub id: Uuid,
}

impl VariableRow {
    /// Create a new variable row.
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            variable: Variable {
                key: key.into(),
                value: value.into(),
                enabled: true,
                secret: false,
            },
            id: Uuid::new_v4(),
        }
    }

    /// Create from an existing Variable.
    pub fn from_variable(variable: Variable) -> Self {
        Self {
            id: Uuid::new_v4(),
            variable,
        }
    }
}

/// Environment editor modal state.
///
/// Manages the editing of environments and their variables.
pub struct EnvEditorModal {
    /// All environments being managed
    pub environments: Vec<Environment>,
    /// Currently selected environment index
    pub selected_env_index: Option<usize>,
    /// Variable rows for the currently selected environment
    pub variable_rows: Vec<VariableRow>,
    /// Currently selected variable index
    pub selected_var_index: Option<usize>,
    /// Whether the modal is currently open
    pub is_open: bool,
    /// Whether there are unsaved changes
    pub has_unsaved_changes: bool,
}

impl EnvEditorModal {
    /// Create a new environment editor modal.
    pub fn new() -> Self {
        Self {
            environments: Vec::new(),
            selected_env_index: None,
            variable_rows: Vec::new(),
            selected_var_index: None,
            is_open: false,
            has_unsaved_changes: false,
        }
    }

    /// Create a modal with initial environments.
    pub fn with_environments(environments: Vec<Environment>) -> Self {
        let mut modal = Self::new();
        modal.environments = environments;
        if !modal.environments.is_empty() {
            modal.selected_env_index = Some(0);
            modal.load_variables_for_selected();
        }
        modal
    }

    /// Load variables for the currently selected environment.
    fn load_variables_for_selected(&mut self) {
        self.variable_rows = self
            .selected_env_index
            .and_then(|idx| self.environments.get(idx))
            .map(|env| {
                env.variables
                    .iter()
                    .cloned()
                    .map(VariableRow::from_variable)
                    .collect()
            })
            .unwrap_or_default();
        self.selected_var_index = None;
    }

    /// Open the modal.
    pub fn open(&mut self, cx: &mut Context<Self>) {
        self.is_open = true;
        self.has_unsaved_changes = false;
        cx.notify();
    }

    /// Close the modal.
    pub fn close(&mut self, cx: &mut Context<Self>) {
        self.is_open = false;
        cx.notify();
    }

    /// Save changes and return the updated environments.
    pub fn save(&mut self, cx: &mut Context<Self>) -> Vec<Environment> {
        // Save current variable rows to the selected environment
        self.sync_variables_to_environment();
        self.has_unsaved_changes = false;
        cx.notify();
        self.environments.clone()
    }

    /// Sync variable rows back to the environment.
    fn sync_variables_to_environment(&mut self) {
        if let Some(idx) = self.selected_env_index {
            if let Some(env) = self.environments.get_mut(idx) {
                env.variables = self
                    .variable_rows
                    .iter()
                    .map(|row| row.variable.clone())
                    .collect();
            }
        }
    }

    /// Add a new environment.
    pub fn add_environment(&mut self, cx: &mut Context<Self>) {
        let new_env = Environment::new("New Environment");
        self.environments.push(new_env);
        self.selected_env_index = Some(self.environments.len() - 1);
        self.load_variables_for_selected();
        self.has_unsaved_changes = true;
        cx.notify();
    }

    /// Remove the selected environment.
    pub fn remove_environment(&mut self, cx: &mut Context<Self>) {
        if let Some(idx) = self.selected_env_index {
            self.environments.remove(idx);
            if self.environments.is_empty() {
                self.selected_env_index = None;
                self.variable_rows.clear();
            } else if idx >= self.environments.len() {
                self.selected_env_index = Some(self.environments.len() - 1);
            }
            self.load_variables_for_selected();
            self.has_unsaved_changes = true;
            cx.notify();
        }
    }

    /// Rename the selected environment.
    pub fn rename_environment(&mut self, new_name: String, cx: &mut Context<Self>) {
        if let Some(idx) = self.selected_env_index {
            if let Some(env) = self.environments.get_mut(idx) {
                env.name = new_name;
                self.has_unsaved_changes = true;
                cx.notify();
            }
        }
    }

    /// Add a new variable to the current environment.
    pub fn add_variable(&mut self, cx: &mut Context<Self>) {
        let new_row = VariableRow::new("", "");
        self.variable_rows.push(new_row);
        self.selected_var_index = Some(self.variable_rows.len() - 1);
        self.has_unsaved_changes = true;
        cx.notify();
    }

    /// Remove the selected variable.
    pub fn remove_variable(&mut self, cx: &mut Context<Self>) {
        if let Some(idx) = self.selected_var_index {
            self.variable_rows.remove(idx);
            if self.variable_rows.is_empty() {
                self.selected_var_index = None;
            } else if idx >= self.variable_rows.len() {
                self.selected_var_index = Some(self.variable_rows.len() - 1);
            }
            self.has_unsaved_changes = true;
            cx.notify();
        }
    }

    /// Toggle a variable's enabled state.
    pub fn toggle_variable_enabled(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(row) = self.variable_rows.get_mut(index) {
            row.variable.enabled = !row.variable.enabled;
            self.has_unsaved_changes = true;
            cx.notify();
        }
    }

    /// Toggle a variable's secret state.
    pub fn toggle_variable_secret(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(row) = self.variable_rows.get_mut(index) {
            row.variable.secret = !row.variable.secret;
            self.has_unsaved_changes = true;
            cx.notify();
        }
    }

    /// Select an environment by index.
    pub fn select_environment(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.environments.len() {
            self.sync_variables_to_environment();
            self.selected_env_index = Some(index);
            self.load_variables_for_selected();
            cx.notify();
        }
    }

    /// Select a variable by index.
    pub fn select_variable(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.variable_rows.len() {
            self.selected_var_index = Some(index);
            cx.notify();
        }
    }

    /// Get the selected environment.
    pub fn selected_environment(&self) -> Option<&Environment> {
        self.selected_env_index
            .and_then(|idx| self.environments.get(idx))
    }
}

impl EventEmitter<Vec<Environment>> for EnvEditorModal {}

impl Render for EnvEditorModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div().child("");
        }

        let environments: Vec<_> = self
            .environments
            .iter()
            .enumerate()
            .map(|(idx, env)| {
                let name = env.name.clone();
                let var_count = env.variables.len();
                let is_selected = self.selected_env_index == Some(idx);
                let env_idx = idx;
                let bg = if is_selected {
                    cx.theme().muted
                } else {
                    gpui::transparent_black()
                };

                h_flex()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .bg(bg)
                    .cursor_pointer()
                    .child(div().child(name))
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("{} vars", var_count)),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.select_environment(env_idx, cx);
                        }),
                    )
            })
            .collect();

        let variable_rows: Vec<_> = self
            .variable_rows
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                let var = row.variable.clone();
                let row_idx = idx;
                let bg = if self.selected_var_index == Some(idx) {
                    cx.theme().muted
                } else {
                    gpui::transparent_black()
                };

                // Use SharedString for dynamic IDs
                let enabled_id = SharedString::from(format!("var-enabled-{}", row_idx));
                let secret_id = SharedString::from(format!("var-secret-{}", row_idx));
                let delete_id = SharedString::from(format!("var-delete-{}", row_idx));

                h_flex()
                    .gap_2()
                    .w_full()
                    .items_center()
                    .px_2()
                    .py_1()
                    .bg(bg)
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(120.0))
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .text_sm()
                                    .child(var.key.clone()),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(150.0))
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .text_sm()
                                    .child(if var.secret && !var.value.is_empty() {
                                        "••••••••".to_string()
                                    } else {
                                        var.value.clone()
                                    }),
                            ),
                    )
                    .child(
                        Checkbox::new(enabled_id.clone())
                            .checked(var.enabled)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.toggle_variable_enabled(row_idx, cx);
                            })),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .items_center()
                            .child(div().text_sm().child("Secret"))
                            .child(
                                Switch::new(secret_id.clone())
                                    .checked(var.secret)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.toggle_variable_secret(row_idx, cx);
                                    })),
                            ),
                    )
                    .child(
                        div().child(
                            Button::new(delete_id.clone())
                                .label("×")
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.select_variable(row_idx, cx);
                                    this.remove_variable(cx);
                                })),
                        ),
                    )
            })
            .collect();

        let unsaved_div = if self.has_unsaved_changes {
            div()
                .text_sm()
                .text_color(gpui::rgb(0xf59e0b))
                .child("Unsaved changes")
        } else {
            div().text_sm().child("")
        };

        v_flex()
            .key_context(ENV_EDITOR_CONTEXT)
            .on_action(cx.listener(|this, _: &Save, _, cx| {
                this.save(cx);
                this.close(cx);
            }))
            .on_action(cx.listener(|this, _: &Cancel, _, cx| {
                this.close(cx);
            }))
            .on_action(cx.listener(|this, _: &AddEnv, _, cx| {
                this.add_environment(cx);
            }))
            .on_action(cx.listener(|this, _: &RemoveEnv, _, cx| {
                this.remove_environment(cx);
            }))
            .on_action(cx.listener(|_this, _: &RenameEnv, _, _cx| {
                log::info!("Rename environment");
            }))
            .on_action(cx.listener(|this, _: &AddVar, _, cx| {
                this.add_variable(cx);
            }))
            .on_action(cx.listener(|this, _: &RemoveVar, _, cx| {
                this.remove_variable(cx);
            }))
            .w_full()
            .h(px(400.0))
            // Left panel
            .child(
                h_flex()
                    .flex_1()
                    .child(
                        v_flex()
                            .flex_1()
                            .w(px(200.0))
                            .border_r_1()
                            .border_color(cx.theme().border)
                            .child(
                                div()
                                    .h(px(40.0))
                                    .px_3()
                                    .border_b_1()
                                    .border_color(cx.theme().border)
                                    .items_center()
                                    .flex()
                                    .font_semibold()
                                    .child("Environments"),
                            )
                            .child(div().flex_1().child(v_flex().children(environments)))
                            .child(
                                h_flex()
                                    .p_2()
                                    .gap_2()
                                    .border_t_1()
                                    .border_color(cx.theme().border)
                                    .child(
                                        div().child(
                                            Button::new("add-env")
                                                .label("Add")
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.add_environment(cx);
                                                })),
                                        ),
                                    )
                                    .child(
                                        div().child(
                                            Button::new("remove-env")
                                                .label("Remove")
                                                .disabled(self.selected_env_index.is_none())
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.remove_environment(cx);
                                                })),
                                        ),
                                    )
                                    .child(
                                        div().child(
                                            Button::new("rename-env")
                                                .label("Rename")
                                                .disabled(self.selected_env_index.is_none())
                                                .on_click(cx.listener(|_this, _, _, cx| {
                                                    log::info!("Rename environment");
                                                })),
                                        ),
                                    ),
                            ),
                    )
                    // Right panel
                    .child(
                        v_flex()
                            .flex_1()
                            .child(
                                div()
                                    .h(px(40.0))
                                    .px_3()
                                    .border_b_1()
                                    .border_color(cx.theme().border)
                                    .items_center()
                                    .flex()
                                    .justify_between()
                                    .child(
                                        div()
                                            .font_semibold()
                                            .child("Variables"),
                                    )
                                    .child(
                                        div().child(
                                            Button::new("add-var")
                                                .label("Add Variable")
                                                .disabled(self.selected_env_index.is_none())
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.add_variable(cx);
                                                })),
                                        ),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .p_2()
                                            .children(variable_rows),
                                    ),
                            ),
                    ),
            )
            // Footer
            .child(
                h_flex()
                    .gap_2()
                    .p_3()
                    .border_t_1()
                    .justify_end()
                    .child(unsaved_div)
                    .children(vec![
                        Button::new("cancel")
                            .label("Cancel")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.close(cx);
                            })),
                        Button::new("save")
                            .label("Save")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.save(cx);
                                this.close(cx);
                            })),
                    ]),
            )
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
        assert!(!modal.is_open);
        assert!(modal.environments.is_empty());
        assert!(modal.selected_env_index.is_none());
    }

    #[test]
    fn test_modal_with_environments() {
        let envs = vec![
            Environment::new("Development"),
            Environment::new("Production"),
        ];
        let modal = EnvEditorModal::with_environments(envs);
        assert_eq!(modal.environments.len(), 2);
        assert_eq!(modal.selected_env_index, Some(0));
    }

    #[test]
    fn test_variable_row_creation() {
        let row = VariableRow::new("API_KEY", "secret123");
        assert_eq!(row.variable.key, "API_KEY");
        assert_eq!(row.variable.value, "secret123");
        assert!(row.variable.enabled);
        assert!(!row.variable.secret);
    }
}
