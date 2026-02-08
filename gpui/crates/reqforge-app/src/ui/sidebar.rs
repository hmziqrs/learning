//! Sidebar panel component - displays collection tree with GPUI.
//!
//! This module implements the sidebar panel using gpui-component's Tree component.
//! It displays the hierarchical collection tree with folders and requests,
//! supporting expand/collapse for folders, click-to-open, and context menus.

use crate::app_state::AppState;
use gpui::{
    App, AppContext, Context, Entity, InteractiveElement, IntoElement, KeyBinding, MouseButton,
    ParentElement, Render, SharedString, Styled, Window, actions, div, prelude::FluentBuilder, px,
};
use gpui_component::{ActiveTheme, Icon, IconName, StyledExt, h_flex, list, tree, v_flex};
use reqforge_core::models::{
    collection::Collection,
    folder::CollectionItem,
    request::{HttpMethod, RequestDefinition},
};
use uuid::Uuid;

/// Context key for sidebar keyboard shortcuts
const SIDEBAR_CONTEXT: &str = "SidebarPanel";

actions!(sidebar, [Open, NewRequest, NewFolder, Rename, Delete]);

/// Initialize sidebar keyboard bindings.
pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("enter", Open, Some(SIDEBAR_CONTEXT)),
        KeyBinding::new("cmd-n", NewRequest, Some(SIDEBAR_CONTEXT)),
        KeyBinding::new("cmd-shift-n", NewFolder, Some(SIDEBAR_CONTEXT)),
        KeyBinding::new("f2", Rename, Some(SIDEBAR_CONTEXT)),
        KeyBinding::new("backspace", Delete, Some(SIDEBAR_CONTEXT)),
    ]);
}

/// Sidebar panel component that displays the collection tree.
///
/// This component wraps a Tree entity and manages the conversion between
/// the core Collection model and the TreeItem format expected by gpui-component.
pub struct SidebarPanel {
    /// The application state entity
    app_state: Entity<AppState>,
    /// Currently selected item ID
    selected_item_id: Option<Uuid>,
    /// Currently selected collection ID (for context menu actions)
    selected_collection_id: Option<Uuid>,
}

impl SidebarPanel {
    /// Create a new SidebarPanel.
    pub fn new(app_state: Entity<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            selected_item_id: None,
            selected_collection_id: None,
        }
    }

    /// Convert a CollectionItem to a TreeItem (simplified).
    fn collection_item_to_tree_item(
        &self,
        collection_item: &CollectionItem,
        collection: &Collection,
    ) -> tree::TreeItem {
        match collection_item {
            CollectionItem::Folder(folder) => {
                let mut folder_item = tree::TreeItem::new(
                    folder.id.to_string(),
                    SharedString::from(folder.name.clone()),
                );

                // Add folder children
                for child in &folder.children {
                    folder_item =
                        folder_item.child(self.collection_item_to_tree_item(child, collection));
                }

                folder_item.expanded(true)
            }
            CollectionItem::Request(request_id) => {
                if let Some(request) = collection.requests.get(request_id) {
                    tree::TreeItem::new(
                        request.id.to_string(),
                        SharedString::from(request.name.clone()),
                    )
                } else {
                    tree::TreeItem::new(
                        request_id.to_string(),
                        SharedString::from("Unknown Request"),
                    )
                }
            }
        }
    }

    /// Open a request in a new tab (or focus existing tab).
    fn open_request_in_tab(
        &mut self,
        collection_id: Uuid,
        request: RequestDefinition,
        cx: &mut Context<Self>,
    ) {
        // Check if tab already exists
        let app_state = self.app_state.clone();
        let existing_tab = app_state
            .read(cx)
            .tabs
            .iter()
            .position(|tab| tab.request_id == request.id);

        if let Some(index) = existing_tab {
            // Focus existing tab
            app_state.update(cx, |state, cx| {
                state.active_tab = Some(index);
                cx.notify();
            });
        } else {
            // Create new tab
            app_state.update(cx, |state, cx| {
                state.open_tab(request.id, collection_id, request);
                cx.notify();
            });
        }
    }

    /// Handle the Open action.
    fn on_action_open(&mut self, _: &Open, _window: &mut Window, _cx: &mut Context<Self>) {
        // For now, just log. The Tree component handles item clicks internally.
        log::info!(
            "Open action triggered, selected_item: {:?}",
            self.selected_item_id
        );
    }

    /// Handle the New Request action.
    fn on_action_new_request(
        &mut self,
        _: &NewRequest,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Get the selected collection or use the first one
        let collection_id = self.selected_collection_id.or_else(|| {
            let core = self.app_state.read(cx).core.clone();
            core.collections.first().map(|c| c.id)
        });

        if let Some(collection_id) = collection_id {
            // Create a new request
            let new_request =
                RequestDefinition::new("New Request", HttpMethod::GET, "{{base_url}}");

            // Add to core (this would need a method on ReqForgeCore)
            // For now, just open it in a tab
            self.open_request_in_tab(collection_id, new_request, cx);
        }
    }

    /// Handle the New Folder action.
    fn on_action_new_folder(
        &mut self,
        _: &NewFolder,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        // TODO: Implement folder creation dialog
        // For now, just log
        log::info!("New Folder action triggered");
    }

    /// Handle the Rename action.
    fn on_action_rename(&mut self, _: &Rename, _window: &mut Window, _cx: &mut Context<Self>) {
        // TODO: Implement rename dialog
        log::info!(
            "Rename action triggered for item: {:?}",
            self.selected_item_id
        );
    }

    /// Handle the Delete action.
    fn on_action_delete(&mut self, _: &Delete, _window: &mut Window, _cx: &mut Context<Self>) {
        // TODO: Implement delete confirmation and logic
        log::info!(
            "Delete action triggered for item: {:?}",
            self.selected_item_id
        );
    }
}

impl Render for SidebarPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Capture collections data for the render closure
        let collections: Vec<_> = self
            .app_state
            .read(cx)
            .core
            .collections
            .iter()
            .flat_map(|col| {
                std::iter::once(
                    tree::TreeItem::new(col.id.to_string(), SharedString::from(col.name.clone()))
                        .expanded(true),
                )
                .chain(
                    col.tree
                        .iter()
                        .map(|item| self.collection_item_to_tree_item(item, col)),
                )
            })
            .collect();

        // Build the main sidebar container
        div()
            .id("sidebar-panel")
            .key_context(SIDEBAR_CONTEXT)
            .size_full()
            .flex()
            .flex_col()
            .on_action(cx.listener(Self::on_action_open))
            .on_action(cx.listener(Self::on_action_new_request))
            .on_action(cx.listener(Self::on_action_new_folder))
            .on_action(cx.listener(Self::on_action_rename))
            .on_action(cx.listener(Self::on_action_delete))
            .child(
                v_flex()
                    .id("sidebar-header")
                    .h(px(40.0))
                    .px_3()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .items_center()
                    .justify_between()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Folder).size(px(16.0)))
                            .child(
                                div()
                                    .text_sm()
                                    .font_semibold()
                                    .text_color(cx.theme().foreground)
                                    .child("Collections"),
                            ),
                    )
                    .child(
                        h_flex().gap_1().child(
                            div()
                                .size(px(28.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_md()
                                .hover(|div| div.bg(cx.theme().muted))
                                .cursor_pointer()
                                .child(Icon::new(IconName::Plus).size(px(14.0)))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _window, cx| {
                                        this.on_action_new_request(&NewRequest, _window, cx);
                                    }),
                                ),
                        ),
                    ),
            )
            .child(div().id("sidebar-tree-container").flex_1().child(
                // Create tree state inline with simple items
                {
                    let tree_state = cx.new(|cx| tree::TreeState::new(cx).items(collections));

                    tree::Tree::new(&tree_state, {
                        move |ix: usize,
                              entry: &tree::TreeEntry,
                              selected: bool,
                              _window: &mut Window,
                              cx: &mut App| {
                            let _item_id = entry.item().id.clone();
                            let label = entry.item().label.clone();
                            let depth = entry.depth();
                            let is_folder = entry.is_folder();

                            // Simple content div
                            let content = h_flex()
                                .gap_2()
                                .items_center()
                                .h(px(28.0))
                                .pl(px(16.0 * depth as f32 + 8.0))
                                .pr(px(8.0))
                                .rounded_lg()
                                .when(selected, |div| div.bg(cx.theme().muted));

                            // Add icon
                            let content = if is_folder {
                                let icon = if entry.is_expanded() {
                                    IconName::FolderOpen
                                } else {
                                    IconName::FolderClosed
                                };
                                content.child(
                                    Icon::new(icon)
                                        .size(px(14.0))
                                        .text_color(cx.theme().muted_foreground),
                                )
                            } else {
                                content.child(
                                    Icon::new(IconName::File)
                                        .size(px(14.0))
                                        .text_color(cx.theme().muted_foreground),
                                )
                            };

                            // Build list item
                            list::ListItem::new(ix)
                                .w_full()
                                .selected(selected)
                                .child(content.child(label))
                        }
                    })
                },
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqforge_core::ReqForgeCore;

    #[gpui::test]
    fn test_sidebar_creation(cx: &mut gpui::TestAppContext) {
        let core = ReqForgeCore::new();
        let app_state = cx.new(|_| AppState::new(core));
        let sidebar = cx.new(|cx| SidebarPanel::new(app_state.clone(), cx));

        // Verify sidebar was created
        let sidebar_read = sidebar.read(cx);
        assert_eq!(sidebar_read.selected_item_id, None);
    }
}
