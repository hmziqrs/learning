//! Environment selector component - dropdown for selecting active environment.
//!
//! This module implements the environment selector using gpui-component.
//! It displays the currently active environment and allows switching between
//! environments, plus opening the environment editor modal.

use crate::app_state::AppState;
use gpui::{
    actions, div, px, App, Context, Entity, EventEmitter, InteractiveElement,
    IntoElement, KeyBinding, MouseButton, ParentElement, Render, Styled, Window,
};
use gpui_component::{h_flex, v_flex, ActiveTheme, Icon, IconName};
use uuid::Uuid;

/// Context key for environment selector keyboard shortcuts
const ENV_SELECTOR_CONTEXT: &str = "EnvSelector";

/// Actions for environment selector
actions!(env_selector, [OpenDropdown, CloseDropdown, OpenEnvEditor]);

/// Initialize environment selector keyboard bindings.
pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("escape", CloseDropdown, Some(ENV_SELECTOR_CONTEXT)),
    ]);
}

/// Environment selector menu item type for internal tracking.
#[derive(Debug, Clone)]
pub enum EnvMenuItem {
    /// No environment selected (clears active)
    NoEnvironment,
    /// Select an environment by ID
    SelectEnvironment(Uuid),
    /// Open the environment editor modal
    ManageEnvironments,
}

/// Environment selector component.
///
/// Provides a dropdown for selecting the active environment from available environments.
/// Shows the currently active environment and allows switching between environments.
///
/// ## Features
/// - Displays active environment name or "No Environment"
/// - Dropdown with all environments listed
/// - "No Environment" option at top to clear selection
/// - "Manage Environments..." option to open the env editor modal
/// - Icon and chevron indicator
pub struct EnvSelector {
    /// The application state entity
    app_state: Entity<AppState>,
    /// Whether the dropdown is currently open
    dropdown_open: bool,
    /// Whether the component is focused
    focused: bool,
}

impl EnvSelector {
    /// Create a new EnvSelector.
    pub fn new(app_state: Entity<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            dropdown_open: false,
            focused: false,
        }
    }

    /// Toggle the dropdown open/closed.
    fn toggle_dropdown(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.dropdown_open = !self.dropdown_open;
        cx.notify();
    }

    /// Open the dropdown.
    fn open_dropdown(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.dropdown_open = true;
        cx.notify();
    }

    /// Close the dropdown.
    fn close_dropdown(&mut self, _: &CloseDropdown, _window: &mut Window, cx: &mut Context<Self>) {
        self.dropdown_open = false;
        cx.notify();
    }

    /// Handle the OpenDropdown action.
    fn on_action_open_dropdown(&mut self, _: &OpenDropdown, window: &mut Window, cx: &mut Context<Self>) {
        self.open_dropdown(window, cx);
    }

    /// Handle selecting "No Environment".
    fn on_select_no_environment(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let app_state = self.app_state.clone();
        app_state.update(cx, |state, cx| {
            state.active_env_id = None;
            cx.notify();
        });
        self.dropdown_open = false;
        cx.notify();
    }

    /// Handle selecting an environment.
    fn on_select_environment(
        &mut self,
        env_id: Uuid,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let app_state = self.app_state.clone();
        app_state.update(cx, |state, cx| {
            // Verify the environment exists
            if state.core.environments.iter().any(|e| e.id == env_id) {
                state.active_env_id = Some(env_id);
                cx.notify();
            }
        });
        self.dropdown_open = false;
        cx.notify();
    }

    /// Handle clicking "Manage Environments...".
    fn on_manage_environments(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        // Emit an event that the parent can handle to open the modal
        cx.emit(());
        self.dropdown_open = false;
        cx.notify();
    }

    /// Handle the OpenEnvEditor action.
    fn on_action_open_env_editor(&mut self, _: &OpenEnvEditor, window: &mut Window, cx: &mut Context<Self>) {
        self.on_manage_environments(window, cx);
    }

    /// Render the active environment name or "No Environment".
    fn render_active_env_name(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = self.app_state.read(cx);
        let active_env_name = app_state
            .active_env_id
            .and_then(|id| app_state.core.environments.iter().find(|e| e.id == id))
            .map(|e| e.name.clone())
            .unwrap_or_else(|| "No Environment".to_string());

        div()
            .text_sm()
            .text_color(cx.theme().foreground)
            .child(active_env_name)
    }

    /// Render a dropdown menu item for "No Environment".
    fn render_no_env_item(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_active = {
            let app_state = self.app_state.read(cx);
            app_state.active_env_id.is_none()
        };

        let mut item = div()
            .h(px(32.0))
            .px_3()
            .rounded_md()
            .flex()
            .items_center()
            .cursor_pointer();

        if is_active {
            item = item.bg(cx.theme().muted);
        }

        item.child(
            h_flex()
                .gap_2()
                .items_center()
                .child(
                    Icon::new(IconName::Close)
                        .size(px(14.0))
                        .text_color(cx.theme().muted_foreground),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().foreground)
                        .child("No Environment"),
                ),
        )
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|this, _, window, cx| this.on_select_no_environment(window, cx)),
        )
    }

    /// Render a dropdown menu item for an environment.
    fn render_env_item(&self, env_id: Uuid, env_name: String, var_count: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let is_active = {
            let app_state = self.app_state.read(cx);
            app_state.active_env_id == Some(env_id)
        };

        let mut item = div()
            .h(px(32.0))
            .px_3()
            .rounded_md()
            .flex()
            .items_center()
            .justify_between()
            .cursor_pointer();

        if is_active {
            item = item.bg(cx.theme().muted);
        }

        let mut content = h_flex()
            .gap_2()
            .items_center()
            .child(
                Icon::new(IconName::Globe)
                    .size(px(14.0))
                    .text_color(cx.theme().muted_foreground),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().foreground)
                    .child(env_name.clone()),
            );

        // Add variable count if any
        if var_count > 0 {
            let var_text = format!("({} var{})", var_count, if var_count == 1 { "" } else { "s" });
            content = content.child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(var_text),
            );
        }

        item = item.child(content);

        // Add checkmark if active
        if is_active {
            item = item.child(
                Icon::new(IconName::Check)
                    .size(px(14.0))
                    .text_color(cx.theme().primary),
            );
        }

        item.on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _, window, cx| {
                this.on_select_environment(env_id, window, cx);
            }),
        )
    }

    /// Render "Manage Environments..." menu item.
    fn render_manage_item(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h(px(32.0))
            .px_3()
            .rounded_md()
            .flex()
            .items_center()
            .cursor_pointer()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        Icon::new(IconName::Settings)
                            .size(px(14.0))
                            .text_color(cx.theme().muted_foreground),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().foreground)
                            .child("Manage Environments..."),
                    ),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.on_manage_environments(window, cx)),
            )
    }

    /// Render a separator line.
    fn render_separator(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h(px(1.0))
            .mx_1()
            .my_1()
            .bg(cx.theme().border)
    }

    /// Render the dropdown menu with all environments.
    fn render_dropdown_menu(&self, cx: &mut Context<Self>) -> impl IntoElement {
        // Collect environments data
        let environments = {
            let app_state = self.app_state.read(cx);
            app_state.core.environments.clone()
        };

        // Build the dropdown menu container with initial content
        let menu_div = div()
            .absolute()
            .top(px(36.0))
            .right(px(0.0))
            .min_w(px(200.0))
            .max_w(px(300.0))
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border)
            .rounded_md()
            .child(
                v_flex()
                    .gap_1()
                    .p_1()
                    .child(self.render_no_env_item(cx))
                    .child(self.render_separator(cx)),
            );

        // Add all environment items
        let mut result = menu_div;
        for env in &environments {
            let env_id = env.id;
            let env_name = env.name.clone();
            let var_count = env.variables.len();
            result = result.child(self.render_env_item(env_id, env_name, var_count, cx));
        }

        // Add final separator and manage option
        result
            .child(self.render_separator(cx))
            .child(self.render_manage_item(cx))
    }

    /// Render the main button showing current environment.
    fn render_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let env_count = {
            let app_state = self.app_state.read(cx);
            app_state.core.environments.len()
        };

        let chevron_icon = if self.dropdown_open {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        let mut button = div()
            .h(px(32.0))
            .min_w(px(180.0))
            .px_3()
            .py_1()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .cursor_pointer();

        if self.dropdown_open {
            button = button
                .border_color(cx.theme().primary);
        }

        button.child(
            h_flex()
                .gap_2()
                .items_center()
                .justify_between()
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            Icon::new(IconName::Globe)
                                .size(px(14.0))
                                .text_color(cx.theme().muted_foreground),
                        )
                        .child(self.render_active_env_name(cx)),
                )
                .child({
                    let mut chevron_and_count = h_flex()
                        .gap_2()
                        .items_center();

                    // Show environment count if any
                    if env_count > 0 {
                        chevron_and_count = chevron_and_count.child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child(format!(
                                    "({} env{})",
                                    env_count,
                                    if env_count == 1 { "" } else { "s" }
                                )),
                        );
                    }

                    // Add chevron
                    chevron_and_count = chevron_and_count.child(
                        Icon::new(chevron_icon)
                            .size(px(12.0))
                            .text_color(cx.theme().muted_foreground),
                    );

                    chevron_and_count
                }),
        )
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|this, _, window, cx| this.toggle_dropdown(window, cx)),
        )
    }
}

impl EventEmitter<()> for EnvSelector {}

impl Render for EnvSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let container = div()
            .id("env-selector")
            .key_context(ENV_SELECTOR_CONTEXT)
            .relative()
            .on_action(cx.listener(Self::on_action_open_dropdown))
            .on_action(cx.listener(Self::close_dropdown))
            .on_action(cx.listener(Self::on_action_open_env_editor))
            .child(self.render_button(cx));

        if self.dropdown_open {
            container.child(self.render_dropdown_menu(cx))
        } else {
            container
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqforge_core::{ReqForgeCore, models::environment::Environment};

    #[gpui::test]
    fn test_env_selector_creation(cx: &mut gpui::TestAppContext) {
        let core = ReqForgeCore::new();
        let app_state = cx.new(|_| AppState::new(core));
        let selector = cx.new(|cx| EnvSelector::new(app_state.clone(), cx));

        // Verify selector was created
        let selector_read = selector.read(cx);
        assert!(!selector_read.dropdown_open);
        assert!(!selector_read.focused);
    }

    #[gpui::test]
    fn test_env_selector_toggle_dropdown(cx: &mut gpui::TestAppContext) {
        let core = ReqForgeCore::new();
        let app_state = cx.new(|_| AppState::new(core));
        let selector = cx.new(|cx| EnvSelector::new(app_state.clone(), cx));

        // Toggle dropdown open
        selector.update(cx, |selector, window, cx| {
            selector.toggle_dropdown(window, cx);
        });

        assert!(selector.read(cx).dropdown_open);

        // Toggle dropdown closed
        selector.update(cx, |selector, window, cx| {
            selector.toggle_dropdown(window, cx);
        });

        assert!(!selector.read(cx).dropdown_open);
    }

    #[gpui::test]
    fn test_env_selector_with_environments(cx: &mut gpui::TestAppContext) {
        let mut core = ReqForgeCore::new();
        let env1 = Environment::new("Development");
        let env1_id = env1.id;
        core.environments.push(env1);
        core.environments.push(Environment::new("Production"));

        let app_state = cx.new(|_| AppState::new(core));
        let selector = cx.new(|cx| EnvSelector::new(app_state.clone(), cx));

        // Verify environments are available
        let app_state_read = app_state.read(cx);
        assert_eq!(app_state_read.core.environments.len(), 2);

        // Select an environment
        drop(app_state_read);
        app_state.update(cx, |state, cx| {
            state.active_env_id = Some(env1_id);
            cx.notify();
        });

        // Verify selection
        let app_state_read = app_state.read(cx);
        assert_eq!(app_state_read.active_env_id, Some(env1_id));
    }

    #[gpui::test]
    fn test_env_selector_clear_environment(cx: &mut gpui::TestAppContext) {
        let mut core = ReqForgeCore::new();
        let env1 = Environment::new("Staging");
        let env1_id = env1.id;
        core.environments.push(env1);

        let app_state = cx.new(|_| AppState::new(core));
        let selector = cx.new(|cx| EnvSelector::new(app_state.clone(), cx));

        // Set active environment
        app_state.update(cx, |state, cx| {
            state.active_env_id = Some(env1_id);
            cx.notify();
        });

        // Clear active environment
        selector.update(cx, |selector, window, cx| {
            selector.on_select_no_environment(window, cx);
        });

        // Verify cleared
        let app_state_read = app_state.read(cx);
        assert!(app_state_read.active_env_id.is_none());
    }

    #[test]
    fn test_env_menu_item_variants() {
        // Just verify the variants exist and can be created
        let _no_env = EnvMenuItem::NoEnvironment;
        let id = Uuid::new_v4();
        let _select = EnvMenuItem::SelectEnvironment(id);
        let _manage = EnvMenuItem::ManageEnvironments;
    }
}
