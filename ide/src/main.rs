use iced::widget::{column, container, text};
use iced::{Element, Task, Theme};
use iced_code_editor::{CodeEditor, Message as EditorMessage, theme};

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Code Editor")
        .theme(App::theme)
        .run()
}

struct App {
    editor: CodeEditor,
}

#[derive(Debug, Clone)]
enum Message {
    Editor(EditorMessage),
}

impl Default for App {
    fn default() -> Self {
        let code = r#"fn main() {
    println!("Hello, world!");
}
"#;
        let mut editor = CodeEditor::new(code, "rust");
        editor.set_theme(theme::from_iced_theme(&Theme::TokyoNightStorm));
        Self { editor }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Editor(event) => self.editor.update(&event).map(Message::Editor),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let header = text("iced-code-editor demo").size(18);

        container(
            column![header, self.editor.view().map(Message::Editor)]
                .spacing(10),
        )
        .padding(20)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }
}
