use iced::widget::{column, container, row, stack, text, text_input};
use iced::{event, mouse, window, Element, Length, Subscription, Task, Theme};
use std::time::Duration;

use crate::model::{
    AppState, ClickAction, ContextMenuTarget, DeleteDialog, DragKind, DragState, FolderId, NodeRef,
    PendingLongPress, PressKind, RequestId, SidebarDropTarget, Tab,
};
use crate::tree_ops;
use crate::ui;

pub struct PostmanUiApp {
    pub state: AppState,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleContextMenu(ContextMenuTarget),
    CloseContextMenu,
    CreateFolder {
        parent: Option<FolderId>,
    },
    CreateRequest {
        parent: Option<FolderId>,
    },
    AskDeleteFolder(FolderId),
    AskDeleteRequest(RequestId),
    AskDeleteTab(u64),
    ConfirmDelete,
    CancelDelete,
    SelectRequest(RequestId),
    NewTab,
    UrlChanged(String),
    ToggleFolder(FolderId),
    BeginLongPressSidebar {
        kind: DragKind,
        source_parent: Option<FolderId>,
        source_index: usize,
        click_action: Option<ClickAction>,
    },
    BeginLongPressTab {
        tab_id: u64,
        source_index: usize,
    },
    LongPressElapsed(u64),
    HoverSidebarTarget(SidebarDropTarget),
    ClearSidebarHover,
    HoverTabIndex(usize),
    ClearTabHover,
    PointerMoved(iced::Point),
    PointerReleased,
    WindowResized(iced::Size),
}

impl PostmanUiApp {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: AppState::sample(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleContextMenu(target) => {
                if self.state.open_context_menu == Some(target) {
                    self.state.open_context_menu = None;
                    self.state.context_menu_position = None;
                } else {
                    self.state.open_context_menu = Some(target);
                    self.state.context_menu_position = Some(self.state.pointer_position);
                }
            }
            Message::CloseContextMenu => {
                self.state.open_context_menu = None;
                self.state.context_menu_position = None;
            }
            Message::CreateFolder { parent } => {
                let id = self.state.alloc_folder_id();
                let name = format!("New Folder {id}");

                let folder = crate::model::TreeNode::Folder(crate::model::FolderNode {
                    id,
                    name,
                    expanded: true,
                    children: vec![],
                });

                let index = self.children_len(parent);
                let _ = tree_ops::insert_node(&mut self.state.tree_root, parent, index, folder);
                self.state.open_context_menu = None;
                self.state.context_menu_position = None;
            }
            Message::CreateRequest { parent } => {
                let id = self.state.alloc_request_id();
                let name = format!("New Request {id}");

                let request = crate::model::TreeNode::Request(crate::model::RequestNode {
                    id,
                    name,
                    url: "https://api.example.com/new".to_string(),
                });

                let index = self.children_len(parent);
                let _ = tree_ops::insert_node(&mut self.state.tree_root, parent, index, request);
                self.state.open_context_menu = None;
                self.state.context_menu_position = None;
            }
            Message::AskDeleteFolder(folder_id) => {
                self.state.delete_dialog = Some(DeleteDialog::Folder(folder_id));
                self.state.open_context_menu = None;
                self.state.context_menu_position = None;
            }
            Message::AskDeleteRequest(request_id) => {
                self.state.delete_dialog = Some(DeleteDialog::Request(request_id));
                self.state.open_context_menu = None;
                self.state.context_menu_position = None;
            }
            Message::AskDeleteTab(tab_id) => {
                self.state.delete_dialog = Some(DeleteDialog::Tab(tab_id));
            }
            Message::ConfirmDelete => {
                if let Some(dialog) = self.state.delete_dialog.take() {
                    match dialog {
                        DeleteDialog::Folder(folder_id) => {
                            if let Some(removed) =
                                tree_ops::remove_folder(&mut self.state.tree_root, folder_id)
                            {
                                let mut request_ids = vec![];
                                tree_ops::collect_request_ids(&removed, &mut request_ids);
                                self.state.tabs.retain(|tab| {
                                    !tab.request_id.is_some_and(|id| request_ids.contains(&id))
                                });
                            }
                        }
                        DeleteDialog::Request(request_id) => {
                            let _ = tree_ops::remove_request(&mut self.state.tree_root, request_id);
                            self.state
                                .tabs
                                .retain(|tab| tab.request_id != Some(request_id));
                        }
                        DeleteDialog::Tab(tab_id) => {
                            self.state.tabs.retain(|tab| tab.id != tab_id);
                        }
                    }

                    self.state.fallback_active_tab();
                }
            }
            Message::CancelDelete => {
                self.state.delete_dialog = None;
            }
            Message::SelectRequest(request_id) => {
                self.open_request_tab(request_id);
                self.state.open_context_menu = None;
                self.state.context_menu_position = None;
            }
            Message::NewTab => {
                let id = self.state.alloc_tab_id();
                let tab = Tab {
                    id,
                    request_id: None,
                    title: format!("New Tab {id}"),
                    url_input: String::new(),
                };

                self.state.tabs.push(tab);
                self.state.active_tab = Some(id);
                self.state.sync_url_input_from_active_tab();
            }
            Message::UrlChanged(value) => {
                self.state.url_input = value.clone();
                if let Some(tab) = self.state.active_tab_mut() {
                    tab.url_input = value;
                }
            }
            Message::ToggleFolder(folder_id) => {
                if let Some(current) = self.folder_expanded(folder_id) {
                    let _ = tree_ops::set_folder_expanded(
                        &mut self.state.tree_root,
                        folder_id,
                        !current,
                    );
                }
            }
            Message::BeginLongPressSidebar {
                kind,
                source_parent,
                source_index,
                click_action,
            } => {
                let token = self.state.alloc_press_token();
                self.state.pending_long_press = Some(PendingLongPress {
                    token,
                    kind: PressKind::Sidebar {
                        kind,
                        source_parent,
                        source_index,
                    },
                    click_action,
                });
                self.state.open_context_menu = None;
                self.state.context_menu_position = None;

                return Task::perform(
                    async move {
                        tokio::time::sleep(Duration::from_millis(220)).await;
                        token
                    },
                    Message::LongPressElapsed,
                );
            }
            Message::BeginLongPressTab {
                tab_id,
                source_index,
            } => {
                let token = self.state.alloc_press_token();
                self.state.pending_long_press = Some(PendingLongPress {
                    token,
                    kind: PressKind::Tab {
                        tab_id,
                        source_index,
                    },
                    click_action: Some(ClickAction::SelectTab(tab_id)),
                });
                self.state.open_context_menu = None;
                self.state.context_menu_position = None;

                return Task::perform(
                    async move {
                        tokio::time::sleep(Duration::from_millis(220)).await;
                        token
                    },
                    Message::LongPressElapsed,
                );
            }
            Message::LongPressElapsed(token) => {
                if self
                    .state
                    .pending_long_press
                    .is_some_and(|pending| pending.token == token)
                    && self.state.drag_state.is_none()
                {
                    let pending = self.state.pending_long_press.take();

                    if let Some(pending) = pending {
                        match pending.kind {
                            PressKind::Sidebar {
                                kind,
                                source_parent,
                                source_index,
                            } => {
                                self.state.drag_state = Some(DragState::Sidebar {
                                    kind,
                                    source_parent,
                                    source_index,
                                    hover: None,
                                });
                            }
                            PressKind::Tab {
                                tab_id,
                                source_index,
                            } => {
                                self.state.drag_state = Some(DragState::Tabs {
                                    tab_id,
                                    source_index,
                                    hover_index: Some(source_index),
                                });
                            }
                        }
                    }
                }
            }
            Message::HoverSidebarTarget(target) => {
                if let Some(DragState::Sidebar { hover, .. }) = &mut self.state.drag_state {
                    *hover = Some(target);
                }
            }
            Message::ClearSidebarHover => {
                if let Some(DragState::Sidebar { hover, .. }) = &mut self.state.drag_state {
                    *hover = None;
                }
            }
            Message::HoverTabIndex(index) => {
                if let Some(DragState::Tabs { hover_index, .. }) = &mut self.state.drag_state {
                    *hover_index = Some(index);
                }
            }
            Message::ClearTabHover => {
                if let Some(DragState::Tabs { hover_index, .. }) = &mut self.state.drag_state {
                    *hover_index = None;
                }
            }
            Message::PointerMoved(position) => {
                self.state.pointer_position = position;

                if let Some(task) = self.maybe_sidebar_auto_scroll_task() {
                    return task;
                }
            }
            Message::PointerReleased => match self.state.drag_state {
                Some(DragState::Sidebar { .. }) => self.finish_sidebar_drag(),
                Some(DragState::Tabs { .. }) => self.finish_tab_drag(),
                None => {
                    if let Some(pending) = self.state.pending_long_press.take() {
                        if let Some(click_action) = pending.click_action {
                            match click_action {
                                ClickAction::SelectRequest(request_id) => {
                                    self.open_request_tab(request_id);
                                }
                                ClickAction::SelectTab(tab_id) => {
                                    self.state.active_tab = Some(tab_id);
                                    self.state.sync_url_input_from_active_tab();
                                }
                            }
                        }
                    }
                }
            },
            Message::WindowResized(size) => {
                self.state.window_size = size;
            }
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let pointer_events = event::listen_with(|event, _, _| match event {
            iced::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                Some(Message::PointerMoved(position))
            }
            iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                Some(Message::PointerReleased)
            }
            _ => None,
        });

        let resize_events =
            window::resize_events().map(|(_window_id, size)| Message::WindowResized(size));

        Subscription::batch(vec![pointer_events, resize_events])
    }

    pub fn view(&self) -> Element<'_, Message> {
        let url_bar = self.view_url_bar();

        let base = column![
            ui::tabs::view_tabs(self),
            url_bar,
            row![
                ui::sidebar::view_sidebar(self),
                ui::main_panel::view_main_panel(self)
            ]
            .height(Length::Fill)
            .spacing(8),
        ]
        .height(Length::Fill)
        .spacing(8)
        .padding(8);

        let mut root: Element<'_, Message> = container(base)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        if let Some(menu_overlay) = ui::sidebar::view_context_menu_overlay(self) {
            root = stack([root, menu_overlay]).into();
        }

        if self.state.delete_dialog.is_some() {
            root = stack([root, ui::delete_modal(self)]).into();
        }

        root
    }

    pub fn theme(&self) -> Theme {
        Theme::CatppuccinMocha
    }

    fn view_url_bar(&self) -> Element<'_, Message> {
        let input = text_input("https://api.example.com", &self.state.url_input)
            .on_input(Message::UrlChanged)
            .padding(10)
            .size(16)
            .width(Length::Fill);

        let url_row = row![
            container(text("GET").size(14))
                .padding([8, 12])
                .style(|theme| ui::styles::method_badge(theme)),
            input,
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        container(url_row)
            .padding(10)
            .style(|theme| ui::styles::panel(theme))
            .into()
    }

    fn open_request_tab(&mut self, request_id: RequestId) {
        if let Some(existing) = self
            .state
            .tabs
            .iter()
            .find(|tab| tab.request_id == Some(request_id))
            .map(|tab| tab.id)
        {
            self.state.active_tab = Some(existing);
            self.state.sync_url_input_from_active_tab();
            return;
        }

        let Some(request) = self.state.find_request(request_id) else {
            return;
        };

        let title = request.name.clone();
        let url = request.url.clone();

        let tab = Tab {
            id: self.state.alloc_tab_id(),
            request_id: Some(request_id),
            title,
            url_input: url,
        };

        self.state.active_tab = Some(tab.id);
        self.state.tabs.push(tab);
        self.state.sync_url_input_from_active_tab();
    }

    fn finish_sidebar_drag(&mut self) {
        let Some(DragState::Sidebar {
            kind,
            source_parent,
            source_index,
            hover,
        }) = self.state.drag_state.clone()
        else {
            self.state.drag_state = None;
            return;
        };

        if let Some(target) = hover {
            let source_ref = match kind {
                DragKind::Folder(folder_id) => NodeRef::Folder(folder_id),
                DragKind::Request(request_id) => NodeRef::Request(request_id),
            };

            let _ = tree_ops::move_node(
                &mut self.state.tree_root,
                source_ref,
                source_parent,
                source_index,
                target,
            );
        }

        self.state.drag_state = None;
    }

    fn finish_tab_drag(&mut self) {
        let Some(DragState::Tabs {
            tab_id,
            hover_index,
            ..
        }) = self.state.drag_state.clone()
        else {
            self.state.drag_state = None;
            return;
        };

        if let Some(mut target_index) = hover_index {
            if let Some(source_index) = self.state.tabs.iter().position(|tab| tab.id == tab_id) {
                target_index = target_index.min(self.state.tabs.len());

                let tab = self.state.tabs.remove(source_index);

                if source_index < target_index {
                    target_index = target_index.saturating_sub(1);
                }

                self.state.tabs.insert(target_index, tab);
            }
        }

        self.state.drag_state = None;
    }

    fn maybe_sidebar_auto_scroll_task(&self) -> Option<Task<Message>> {
        if !matches!(self.state.drag_state, Some(DragState::Sidebar { .. })) {
            return None;
        }

        let x = self.state.pointer_position.x;
        let y = self.state.pointer_position.y;

        if x > 360.0 {
            return None;
        }

        let top_zone = 150.0;
        let bottom_zone = (self.state.window_size.height - 40.0).max(top_zone + 120.0);

        let delta_y: f32 = if y < top_zone {
            -14.0
        } else if y > bottom_zone {
            14.0
        } else {
            0.0
        };

        if delta_y.abs() < f32::EPSILON {
            return None;
        }

        Some(iced::widget::operation::scroll_by(
            sidebar_scroll_id(),
            iced::widget::operation::AbsoluteOffset { x: 0.0, y: delta_y },
        ))
    }

    fn children_len(&self, parent: Option<FolderId>) -> usize {
        fn recurse(nodes: &[crate::model::TreeNode], id: FolderId) -> Option<usize> {
            for node in nodes {
                if let crate::model::TreeNode::Folder(folder) = node {
                    if folder.id == id {
                        return Some(folder.children.len());
                    }

                    if let Some(found) = recurse(&folder.children, id) {
                        return Some(found);
                    }
                }
            }
            None
        }

        match parent {
            None => self.state.tree_root.len(),
            Some(folder_id) => recurse(&self.state.tree_root, folder_id).unwrap_or(0),
        }
    }

    fn folder_expanded(&self, folder_id: FolderId) -> Option<bool> {
        fn recurse(nodes: &[crate::model::TreeNode], id: FolderId) -> Option<bool> {
            for node in nodes {
                if let crate::model::TreeNode::Folder(folder) = node {
                    if folder.id == id {
                        return Some(folder.expanded);
                    }

                    if let Some(found) = recurse(&folder.children, id) {
                        return Some(found);
                    }
                }
            }

            None
        }

        recurse(&self.state.tree_root, folder_id)
    }
}

pub fn sidebar_scroll_id() -> iced::widget::Id {
    iced::widget::Id::new("sidebar-scroll")
}
