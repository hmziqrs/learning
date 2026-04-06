use iced::widget::{column, container, pick_list, row, stack, text, text_input};
use iced::{event, font, mouse, window, Element, Length, Subscription, Task, Theme};

use crate::features;
use crate::model::{
    AppState, ClickAction, DeleteDialog, DragKind, DragState, HttpMethod,
    PressKind,
};
use crate::ui::scale::Density;
use crate::ui;

pub struct PostmanUiApp {
    pub state: AppState,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Feature messages
    Sidebar(features::SidebarMsg),
    Editor(features::EditorMsg),
    Response(features::ResponseMsg),
    Tabs(features::TabsMsg),
    Http(features::HttpMsg),

    // Global messages
    PointerMoved(iced::Point),
    PointerReleased,
    WindowResized(iced::Size),
    IconFontLoaded(Result<(), font::Error>),
    LongPressElapsed(u64),
    ConfirmDelete,
    CancelDelete,
    SetDensity(Density),
    SetFontScale(f32),
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
            // Feature delegation
            Message::Sidebar(msg) => return features::sidebar::update(&mut self.state, msg),
            Message::Editor(msg) => return features::editor::update(&mut self.state, msg),
            Message::Response(msg) => return features::response::update(&mut self.state, msg),
            Message::Tabs(msg) => return features::tabs::update(&mut self.state, msg),
            Message::Http(msg) => return features::http::update(&mut self.state, msg),

            // Global handlers
            Message::PointerMoved(position) => {
                self.state.pointer_position = position;

                if let Some(task) = self.maybe_sidebar_auto_scroll_task() {
                    return task;
                }

                return Task::none();
            }
            Message::PointerReleased => {
                return match self.state.drag_state {
                    Some(DragState::Sidebar { .. }) => {
                        self.finish_sidebar_drag();
                        Task::none()
                    }
                    Some(DragState::Tabs { .. }) => {
                        self.finish_tab_drag();
                        Task::none()
                    }
                    None => {
                        if let Some(pending) = self.state.pending_long_press.take() {
                            if let Some(click_action) = pending.click_action {
                                match click_action {
                                    ClickAction::SelectRequest(request_id) => {
                                        self.state.open_request_tab(request_id);
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
                        Task::none()
                    }
                };
            }
            Message::WindowResized(size) => {
                self.state.window_size = size;
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
            Message::IconFontLoaded(_) => {}
            Message::SetDensity(density) => {
                self.state.ui_scale.density = density;
            }
            Message::SetFontScale(scale) => {
                self.state.ui_scale.font_scale = scale.clamp(0.5, 3.0);
            }
        }

        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let needs_pointer = self.state.drag_state.is_some()
            || self.state.pending_long_press.is_some();

        let pointer_sub = if needs_pointer {
            event::listen_with(|event, _, _| match event {
                iced::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    Some(Message::PointerMoved(position))
                }
                iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    Some(Message::PointerReleased)
                }
                _ => None,
            })
        } else {
            Subscription::none()
        };

        let resize_events =
            window::resize_events().map(|(_window_id, size)| Message::WindowResized(size));

        Subscription::batch(vec![pointer_sub, resize_events])
    }

    pub fn view(&self) -> Element<'_, Message> {
        let state = &self.state;
        let scale = &state.ui_scale;

        let right_content: Element<'_, Message> = if let Some(active_tab) = state.tabs.active() {
            let tabs = features::tabs::view_tabs(&state.tabs, &state.drag_state, scale).map(Message::Tabs);
            let name_row = features::editor::view_request_name_row(active_tab, scale).map(Message::Editor);
            let url_bar = Self::view_url_bar(state);
            let section_tabs = features::editor::view_request_section_tabs(active_tab, scale).map(Message::Editor);
            let request_content = features::editor::view_request_content(active_tab, scale).map(Message::Editor);
            let response_section = features::response::view_response_section(active_tab, scale).map(Message::Response);

            column![tabs, name_row, url_bar, section_tabs, request_content, response_section,]
                .height(Length::Fill)
                .into()
        } else {
            column![
                features::tabs::view_tabs(&state.tabs, &state.drag_state, scale).map(Message::Tabs),
                iced::widget::Space::new().height(Length::Fill),
                container(
                    column![
                        iced::widget::text("Open a request or create a new tab to get started")
                            .size(scale.text_title()),
                    ]
                    .spacing(scale.space_md())
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
            features::sidebar::view_sidebar(state).map(Message::Sidebar),
            right_content,
        ]
        .height(Length::Fill)
        .spacing(0);

        let mut root: Element<'_, Message> = container(base)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        if let Some(menu_overlay) = state.open_context_menu.as_ref()
            .and_then(|target| features::sidebar::view_context_menu_overlay(
                target,
                &state.context_menu_position,
                state.pointer_position,
                state.window_size,
                scale,
            ))
            .map(|el| el.map(Message::Sidebar))
        {
            root = stack([root, menu_overlay]).into();
        }

        if let Some(drag_overlay) = ui::components::drag_preview_overlay(
            &state.drag_state,
            &state.tree,
            &state.tabs,
            state.pointer_position,
            state.window_size,
            scale,
        ) {
            root = stack([root, drag_overlay]).into();
        }

        if let Some(dialog) = &state.delete_dialog {
            root = stack([root, ui::components::delete_modal(dialog, scale)]).into();
        }

        root
    }

    pub fn theme(&self) -> Theme {
        crate::ui::theme::theme()
    }

    fn view_url_bar(state: &AppState) -> Element<'static, Message> {
        let scale = &state.ui_scale;
        let current_method = state
            .tabs
            .active()
            .map(|t| t.method)
            .unwrap_or(HttpMethod::Get);

        let method_picker = pick_list(
            &HttpMethod::ALL[..],
            Some(current_method),
            |m| Message::Editor(features::EditorMsg::MethodChanged(m)),
        )
        .padding(scale.pad_button())
        .style(|theme, status| ui::styles::method_pick_list(theme, status));

        let url_value = state
            .tabs
            .active()
            .map(|t| t.url_input.clone())
            .unwrap_or_default();

        let input = text_input("https://api.example.com", &url_value)
            .on_input(|v| Message::Editor(features::EditorMsg::UrlChanged(v)))
            .padding(scale.pad_input())
            .size(scale.text_title())
            .width(Length::Fill);

        let loading = state.tabs.active().is_some_and(|t| t.loading);
        let send_label = if loading {
            "Sending..."
        } else {
            "Send"
        };

        let send_btn = iced::widget::button(text(send_label).size(scale.text_label()))
            .on_press_maybe(if loading {
                None
            } else {
                Some(Message::Http(features::HttpMsg::SendRequest))
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

pub fn sidebar_scroll_id() -> iced::widget::Id {
    iced::widget::Id::new("sidebar-scroll")
}
