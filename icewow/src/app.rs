use iced::widget::{column, container, row, stack, text, text_input};
use iced::{event, mouse, Element, Length, Subscription, Task, Theme};

use crate::model::{
    AppState, ContextMenuTarget, DeleteDialog, DragKind, DragState, FolderId, NodeRef, RequestId,
    SidebarDropTarget, Tab,
};
use crate::tree_ops;
use crate::ui;

pub struct PostmanUiApp {
    pub state: AppState,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleContextMenu(ContextMenuTarget),
    CreateFolder { parent: Option<FolderId> },
    CreateRequest { parent: Option<FolderId> },
    AskDeleteFolder(FolderId),
    AskDeleteRequest(RequestId),
    AskDeleteTab(u64),
    ConfirmDelete,
    CancelDelete,
    SelectRequest(RequestId),
    SelectTab(u64),
    NewTab,
    UrlChanged(String),
    ToggleFolder(FolderId),
    StartDragSidebar(DragKind, Option<FolderId>, usize),
    HoverSidebarTarget(SidebarDropTarget),
    ClearSidebarHover,
    StartDragTab(u64, usize),
    HoverTabIndex(usize),
    ClearTabHover,
    PointerMoved(iced::Point),
    PointerReleased,
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
                } else {
                    self.state.open_context_menu = Some(target);
                }
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
            }
            Message::AskDeleteFolder(folder_id) => {
                self.state.delete_dialog = Some(DeleteDialog::Folder(folder_id));
                self.state.open_context_menu = None;
            }
            Message::AskDeleteRequest(request_id) => {
                self.state.delete_dialog = Some(DeleteDialog::Request(request_id));
                self.state.open_context_menu = None;
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
            }
            Message::SelectTab(tab_id) => {
                self.state.active_tab = Some(tab_id);
                self.state.sync_url_input_from_active_tab();
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
            Message::StartDragSidebar(kind, source_parent, source_index) => {
                self.state.drag_state = Some(DragState::Sidebar {
                    kind,
                    source_parent,
                    source_index,
                    hover: None,
                });
                self.state.open_context_menu = None;
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
            Message::StartDragTab(tab_id, source_index) => {
                self.state.drag_state = Some(DragState::Tabs {
                    tab_id,
                    source_index,
                    hover_index: Some(source_index),
                });
                self.state.open_context_menu = None;
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
            }
            Message::PointerReleased => match self.state.drag_state {
                Some(DragState::Sidebar { .. }) => self.finish_sidebar_drag(),
                Some(DragState::Tabs { .. }) => self.finish_tab_drag(),
                None => {}
            },
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.state.drag_state.is_none() {
            return Subscription::none();
        }

        event::listen_with(|event, _, _| match event {
            iced::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                Some(Message::PointerMoved(position))
            }
            iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                Some(Message::PointerReleased)
            }
            _ => None,
        })
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

        if self.state.delete_dialog.is_some() {
            stack([
                container(base)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into(),
                ui::delete_modal(self),
            ])
            .into()
        } else {
            container(base)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
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
