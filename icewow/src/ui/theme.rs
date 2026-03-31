use iced::Theme;
use iced::theme::Palette;

const ICEWOW_DARK: Palette = Palette {
    background: iced::Color::from_rgb8(0x09, 0x09, 0x0b), // zinc-950
    text: iced::Color::from_rgb8(0xfa, 0xfa, 0xfa),       // zinc-50
    primary: iced::Color::from_rgb8(0x3b, 0x82, 0xf6),    // blue-500 (accent)
    success: iced::Color::from_rgb8(0x22, 0xc5, 0x5e),    // green-500
    warning: iced::Color::from_rgb8(0xea, 0xb3, 0x08),    // yellow-500
    danger: iced::Color::from_rgb8(0xef, 0x44, 0x44),     // red-500
};

pub fn theme() -> Theme {
    Theme::custom("IceWow Dark", ICEWOW_DARK)
}
