mod app;
mod model;
mod state;
mod ui;

use app::PostmanUiApp;
use iced::Size;

fn main() -> iced::Result {
    iced::application(PostmanUiApp::new, PostmanUiApp::update, PostmanUiApp::view)
        .theme(PostmanUiApp::theme)
        .subscription(PostmanUiApp::subscription)
        .window_size(Size::new(1300.0, 820.0))
        .centered()
        .run()
}
