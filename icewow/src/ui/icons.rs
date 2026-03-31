use iced::widget::{text, Text};
use iconflow::{try_icon, Pack, Size, Style};

pub fn lucide_icon(name: &str, size: f32) -> Text<'static> {
    let icon = try_icon(Pack::Lucide, name, Style::Regular, Size::Regular)
        .expect("missing lucide icon");

    let glyph = char::from_u32(icon.codepoint).expect("invalid lucide codepoint");

    raw_icon(glyph, icon.family, size)
}

pub fn raw_icon(glyph: char, family: &'static str, size: f32) -> Text<'static> {
    text(glyph.to_string())
        .size(size)
        .font(iced::font::Font::with_name(family))
}
