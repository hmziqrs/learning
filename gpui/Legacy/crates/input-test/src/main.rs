use gpui::{App, AppContext, Bounds, Context, Entity, IntoElement, InteractiveElement,
    MouseButton, ParentElement, Point, Render, Size, Styled, Window, WindowOptions,
    TitlebarOptions, px, div};
use gpui_component::{h_flex, v_flex, ActiveTheme, Root, button::{Button, ButtonVariants}, input::{Input, InputState}};

// Simulates AppState - owns InputState entities in a "tab"
struct AppState {
    tabs: Vec<TabState>,
    active_tab: Option<usize>,
}

struct TabState {
    url_input: Entity<InputState>,
    body_input: Entity<InputState>,
}

impl AppState {
    fn new() -> Self {
        Self { tabs: Vec::new(), active_tab: None }
    }

    fn create_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let url_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("https://api.example.com")
        });
        let body_input = cx.new(|cx| {
            InputState::new(window, cx).multi_line(true).placeholder("Body...")
        });

        self.tabs.push(TabState { url_input, body_input });
        self.active_tab = Some(self.tabs.len() - 1);
        cx.notify();
    }

    fn active_tab(&self) -> Option<&TabState> {
        self.active_tab.and_then(|i| self.tabs.get(i))
    }
}

// Simulates RequestEditor - reads InputState from AppState
struct Editor {
    app_state: Entity<AppState>,
}

impl Editor {
    fn new(app_state: Entity<AppState>, _cx: &mut Context<Self>) -> Self {
        Self { app_state }
    }
}

impl Render for Editor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tab = self.app_state.read(cx).active_tab();

        if let Some(tab) = tab {
            let url = tab.url_input.clone();
            let body = tab.body_input.clone();

            v_flex()
                .size_full()
                .p_4()
                .gap_4()
                .child(
                    v_flex().gap_2()
                        .child(h_flex().text_sm().text_color(cx.theme().muted_foreground).child("URL:"))
                        .child(Input::new(&url).appearance(true).bordered(true).w_full()),
                )
                .child(
                    v_flex().gap_2().flex_1()
                        .child(h_flex().text_sm().text_color(cx.theme().muted_foreground).child("Body:"))
                        .child(Input::new(&body).h_full()),
                )
        } else {
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(
                    Button::new("new-tab").label("New Tab").primary()
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.app_state.update(cx, |state, cx| {
                                state.create_tab(window, cx);
                            });
                        })),
                )
        }
    }
}

fn main() {
    gpui::Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        let app_state = cx.new(|_cx| AppState::new());
        let editor = cx.new(|cx| Editor::new(app_state, cx));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(gpui::WindowBounds::Windowed(Bounds {
                    origin: Point { x: px(100.0), y: px(100.0) },
                    size: Size { width: px(600.0), height: px(400.0) },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("Input Test v2".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| Root::new(editor, window, cx)),
        )
        .unwrap();
    });
}
