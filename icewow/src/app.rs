use iced::widget::{column, container, pick_list, row, stack, text, text_input};
use iced::{event, font, mouse, window, Element, Length, Subscription, Task, Theme};
use std::time::Duration;

use crate::model::{
    AppState, BodyType, ClickAction, ContextMenuTarget, DeleteDialog, DragKind, DragState, HttpMethod,
    NodeId, PendingLongPress, PressKind, RequestTab, ResponseData, ResponseTab, SidebarDropTarget,
    TabId,
};
use crate::state::tree::NodeData;
use crate::ui;

pub struct PostmanUiApp {
    pub state: AppState,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleContextMenu(ContextMenuTarget),
    CloseContextMenu,
    CreateFolder {
        parent: Option<NodeId>,
    },
    CreateRequest {
        parent: Option<NodeId>,
    },
    AskDeleteFolder(NodeId),
    AskDeleteRequest(NodeId),
    AskDeleteTab(TabId),
    ConfirmDelete,
    CancelDelete,
    SelectRequest(NodeId),
    NewTab,
    UrlChanged(String),
    ToggleFolder(NodeId),
    BeginLongPressSidebar {
        kind: DragKind,
        source_parent: Option<NodeId>,
        source_index: usize,
        click_action: Option<ClickAction>,
    },
    BeginLongPressTab {
        tab_id: TabId,
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
    SendRequest,
    RequestFinished(TabId, Result<ResponseData, String>),
    MethodChanged(HttpMethod),
    SetBodyType(BodyType),
    UpdateBodyText(String),
    AddFormPair,
    UpdateFormKey(usize, String),
    UpdateFormValue(usize, String),
    RemoveFormPair(usize),
    AddHeader,
    UpdateHeaderKey(usize, String),
    UpdateHeaderValue(usize, String),
    RemoveHeader(usize),
    SetRequestTab(RequestTab),
    SetResponseTab(ResponseTab),
    SaveRequest,
    RequestNameChanged(String),
    AddQueryParam,
    UpdateQueryParamKey(usize, String),
    UpdateQueryParamValue(usize, String),
    RemoveQueryParam(usize),
    IconFontLoaded(Result<(), font::Error>),
}

impl PostmanUiApp {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: AppState::sample(),
            },
            load_icon_fonts(),
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
                let id = self.state.tree.alloc_folder_id();
                let name = format!("New Folder {id}");
                let index = self.state.tree.children_len(parent);
                self.state.tree.insert(
                    parent,
                    index,
                    id,
                    NodeData::Folder { name, expanded: true },
                );
                self.state.open_context_menu = None;
                self.state.context_menu_position = None;
            }
            Message::CreateRequest { parent } => {
                let id = self.state.tree.alloc_request_id();
                let name = format!("New Request {id}");
                let index = self.state.tree.children_len(parent);
                self.state.tree.insert(
                    parent,
                    index,
                    id,
                    NodeData::Request {
                        name,
                        url: "https://api.example.com/new".to_string(),
                        method: HttpMethod::Get,
                    },
                );
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
                            let request_ids = self.state.tree.collect_request_ids(folder_id);
                            self.state.tree.remove(folder_id);
                            self.state.tabs.close_by_requests(&request_ids);
                        }
                        DeleteDialog::Request(request_id) => {
                            self.state.tree.remove(request_id);
                            self.state.tabs.close_by_request(request_id);
                        }
                        DeleteDialog::Tab(tab_id) => {
                            self.state.tabs.close_by_tab(tab_id);
                        }
                    }
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
                self.state.tabs.new_tab();
            }
            Message::UrlChanged(value) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.url_input = value;
                    tab.dirty = true;
                }
            }
            Message::ToggleFolder(folder_id) => {
                if let Some(current) = self.state.tree.is_expanded(folder_id) {
                    self.state.tree.set_expanded(folder_id, !current);
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
                                    self.state.selected_folder = None;
                                }
                                ClickAction::SelectFolder(folder_id) => {
                                    self.state.selected_folder = Some(folder_id);
                                    self.state.tree.set_expanded(folder_id, true);
                                }
                                ClickAction::SelectTab(tab_id) => {
                                    self.state.tabs.set_active(tab_id);
                                }
                            }
                        }
                    }
                }
            },
            Message::WindowResized(size) => {
                self.state.window_size = size;
            }
            Message::SendRequest => {
                let tab = match self.state.tabs.active() {
                    Some(tab) => tab,
                    None => return Task::none(),
                };

                if tab.url_input.is_empty() {
                    return Task::none();
                }

                let url = tab.url_input.clone();
                let method = tab.method;
                let headers = tab.headers.clone();
                let body_type = tab.body_type;
                let body_text = tab.body_text.clone();
                let form_pairs = tab.form_pairs.clone();

                // Capture the sending tab's ID so the response lands on the right tab
                let tab_id = match self.state.tabs.active_id() {
                    Some(id) => id,
                    None => return Task::none(),
                };

                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.loading = true;
                    tab.response = None;
                }

                return Task::perform(
                    send_engine_request(url, method, headers, body_type, body_text, form_pairs),
                    move |result| Message::RequestFinished(tab_id, result),
                );
            }
            Message::RequestFinished(tab_id, result) => {
                if let Some(tab) = self.state.tabs.get_mut(tab_id) {
                    tab.loading = false;
                    match result {
                        Ok(response) => tab.response = Some(response),
                        Err(e) => tab.response = Some(ResponseData {
                            status_code: 0,
                            body: format!("Error: {e}"),
                            elapsed_ms: 0,
                            headers: vec![],
                        }),
                    }
                }
            }
            Message::MethodChanged(method) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.method = method;
                    tab.dirty = true;
                }
            }
            Message::SetBodyType(body_type) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.body_type = body_type;
                    tab.dirty = true;
                }
            }
            Message::UpdateBodyText(text) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.body_text = text;
                    tab.dirty = true;
                }
            }
            Message::AddFormPair => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.form_pairs.push((String::new(), String::new()));
                    tab.dirty = true;
                }
            }
            Message::UpdateFormKey(index, key) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    if let Some(pair) = tab.form_pairs.get_mut(index) {
                        pair.0 = key;
                        tab.dirty = true;
                    }
                }
            }
            Message::UpdateFormValue(index, value) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    if let Some(pair) = tab.form_pairs.get_mut(index) {
                        pair.1 = value;
                        tab.dirty = true;
                    }
                }
            }
            Message::RemoveFormPair(index) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.form_pairs.remove(index);
                    tab.dirty = true;
                }
            }
            Message::AddHeader => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.headers.push((String::new(), String::new()));
                    tab.dirty = true;
                }
            }
            Message::UpdateHeaderKey(index, key) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    if let Some(pair) = tab.headers.get_mut(index) {
                        pair.0 = key;
                        tab.dirty = true;
                    }
                }
            }
            Message::UpdateHeaderValue(index, value) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    if let Some(pair) = tab.headers.get_mut(index) {
                        pair.1 = value;
                        tab.dirty = true;
                    }
                }
            }
            Message::RemoveHeader(index) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.headers.remove(index);
                    tab.dirty = true;
                }
            }
            Message::SetRequestTab(request_tab) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.active_request_tab = request_tab;
                }
            }
            Message::SetResponseTab(response_tab) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.active_response_tab = response_tab;
                }
            }
            Message::SaveRequest => {
                let tab = match self.state.tabs.active() {
                    Some(tab) => tab,
                    None => return Task::none(),
                };

                if let Some(request_id) = tab.request_id {
                    let name = tab.title.clone();
                    let url = tab.url_input.clone();
                    let method = tab.method;

                    self.state.tree.update_request_from_draft(
                        request_id,
                        &name,
                        &url,
                        method,
                    );

                    if let Some(tab) = self.state.tabs.active_mut() {
                        tab.dirty = false;
                    }
                }
            }
            Message::RequestNameChanged(name) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.title = name;
                    tab.dirty = true;
                }
            }
            Message::AddQueryParam => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.query_params.push((String::new(), String::new()));
                    tab.dirty = true;
                }
            }
            Message::UpdateQueryParamKey(index, key) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    if let Some(pair) = tab.query_params.get_mut(index) {
                        pair.0 = key;
                        tab.dirty = true;
                    }
                }
            }
            Message::UpdateQueryParamValue(index, value) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    if let Some(pair) = tab.query_params.get_mut(index) {
                        pair.1 = value;
                        tab.dirty = true;
                    }
                }
            }
            Message::RemoveQueryParam(index) => {
                if let Some(tab) = self.state.tabs.active_mut() {
                    tab.query_params.remove(index);
                    tab.dirty = true;
                }
            }
            Message::IconFontLoaded(_) => {}
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
        let right_content: Element<'_, Message> = if self.state.tabs.active_id().is_some() {
            let url_bar = self.view_url_bar();
            let name_row = ui::main_panel::view_request_name_row(self);
            let section_tabs = ui::main_panel::view_request_section_tabs(self);
            let request_content = ui::main_panel::view_request_content(self);
            let response_section = ui::main_panel::view_response_section(self);

            column![
                ui::tabs::view_tabs(self),
                name_row,
                url_bar,
                section_tabs,
                request_content,
                response_section,
            ]
            .height(Length::Fill)
            .into()
        } else {
            column![
                ui::tabs::view_tabs(self),
                iced::widget::Space::new().height(Length::Fill),
                container(
                    column![
                        iced::widget::text("Open a request or create a new tab to get started").size(self.state.ui_scale.text_title()),
                    ]
                    .spacing(self.state.ui_scale.space_md())
                    .align_x(iced::Alignment::Center),
                )
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .width(Length::Fill)
                .height(Length::Fill),
            ]
            .height(Length::Fill)
            .into()
        };

        let base = row![
            ui::sidebar::view_sidebar(self),
            right_content,
        ]
        .height(Length::Fill)
        .spacing(0);

        let mut root: Element<'_, Message> = container(base)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        if let Some(menu_overlay) = ui::sidebar::view_context_menu_overlay(self) {
            root = stack([root, menu_overlay]).into();
        }

        if let Some(drag_overlay) = ui::drag_preview_overlay(self) {
            root = stack([root, drag_overlay]).into();
        }

        if self.state.delete_dialog.is_some() {
            root = stack([root, ui::delete_modal(self)]).into();
        }

        root
    }

    pub fn theme(&self) -> Theme {
        crate::ui::theme::theme()
    }

    fn view_url_bar(&self) -> Element<'_, Message> {
        let scale = &self.state.ui_scale;
        let current_method = self
            .state
            .tabs
            .active()
            .map(|t| t.method)
            .unwrap_or(HttpMethod::Get);

        let method_picker = pick_list(
            &HttpMethod::ALL[..],
            Some(current_method),
            Message::MethodChanged,
        )
        .padding(scale.pad_button())
        .style(|theme, status| ui::styles::method_pick_list(theme, status));

        let url_value = self
            .state
            .tabs
            .active()
            .map(|t| t.url_input.clone())
            .unwrap_or_default();

        let input = text_input("https://api.example.com", &url_value)
            .on_input(Message::UrlChanged)
            .padding(scale.pad_input())
            .size(scale.text_title())
            .width(Length::Fill);

        let loading = self.state.tabs.active().is_some_and(|t| t.loading);
        let send_label = if loading {
            "Sending..."
        } else {
            "Send"
        };

        let send_btn = iced::widget::button(text(send_label).size(scale.text_label()))
            .on_press_maybe(if loading {
                None
            } else {
                Some(Message::SendRequest)
            })
            .padding([scale.space_md(), 20.0])
            .style(|theme, status| ui::styles::send_button(theme, status));

        let url_row = row![method_picker, input, send_btn,]
            .spacing(scale.space_md())
            .align_y(iced::Alignment::Center);

        container(url_row)
            .padding(scale.pad_panel())
            .style(|theme| ui::styles::panel(theme))
            .into()
    }

    fn open_request_tab(&mut self, request_id: NodeId) {
        let info = self.state.tree.get(request_id).and_then(|entry| match &entry.data {
            NodeData::Request { name, url, method } => {
                Some((name.clone(), url.clone(), *method))
            }
            _ => None,
        });

        if let Some((name, url, method)) = info {
            self.state.tabs.open_for_request(request_id, name, url, method);
        }
    }

    pub fn drag_preview_text(&self) -> Option<String> {
        match &self.state.drag_state {
            Some(DragState::Sidebar {
                kind: DragKind::Folder(folder_id),
                ..
            }) => self
                .state
                .tree
                .folder_name(*folder_id)
                .map(|name| format!("Folder: {name}")),
            Some(DragState::Sidebar {
                kind: DragKind::Request(request_id),
                ..
            }) => self
                .state
                .tree
                .get(*request_id)
                .and_then(|e| match &e.data {
                    NodeData::Request { name, .. } => Some(format!("Request: {name}")),
                    _ => None,
                }),
            Some(DragState::Tabs { tab_id, .. }) => self
                .state
                .tabs
                .get(*tab_id)
                .map(|tab| format!("Tab: {}", tab.title)),
            None => None,
        }
    }

    fn finish_sidebar_drag(&mut self) {
        let Some(DragState::Sidebar {
            kind,
            hover,
            ..
        }) = self.state.drag_state.clone()
        else {
            self.state.drag_state = None;
            return;
        };

        if let Some(target) = hover {
            let source_id = match kind {
                DragKind::Folder(id) => id,
                DragKind::Request(id) => id,
            };
            let _ = self.state.tree.move_node(source_id, target);
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

        if let Some(target_index) = hover_index {
            self.state.tabs.reorder(tab_id, target_index);
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
}

fn load_icon_fonts() -> Task<Message> {
    Task::batch(
        iconflow::fonts()
            .iter()
            .map(|f| font::load(f.bytes).map(Message::IconFontLoaded)),
    )
}

async fn send_engine_request(
    url: String,
    method: HttpMethod,
    headers: Vec<(String, String)>,
    body_type: BodyType,
    body_text: String,
    form_pairs: Vec<(String, String)>,
) -> Result<ResponseData, String> {
    let client = icewow_engine::Client::new();

    let mut request = icewow_engine::Request::new(url, method);
    for (key, value) in headers {
        request = request.header(key, value);
    }

    match body_type {
        BodyType::None => {}
        BodyType::Raw => {
            request = request.raw_body(body_text);
        }
        BodyType::Json => {
            request = request.header(
                "Content-Type".to_string(),
                "application/json".to_string(),
            );
            request = request.raw_body(body_text);
        }
        BodyType::Form => {
            request = request.form(form_pairs);
        }
    }

    client.execute(request).await.map_err(|e| e.to_string())
}

pub fn sidebar_scroll_id() -> iced::widget::Id {
    iced::widget::Id::new("sidebar-scroll")
}
