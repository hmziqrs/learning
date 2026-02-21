use iced::widget::{button, container};
use iced::{border, Background, Color, Shadow, Theme, Vector};

pub fn panel(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(palette.background.weak.color)),
        border: border::rounded(10)
            .color(palette.background.strong.color)
            .width(1),
        ..container::Style::default()
    }
}

pub fn sidebar_panel(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(palette.background.base.color)),
        border: border::rounded(12)
            .color(palette.background.strong.color)
            .width(1),
        ..container::Style::default()
    }
}

pub fn context_menu(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(palette.background.weak.color)),
        border: border::rounded(8)
            .color(palette.background.strong.color)
            .width(1),
        ..container::Style::default()
    }
}

pub fn drag_preview(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        text_color: Some(palette.background.base.text),
        background: Some(Background::Color(palette.background.base.color)),
        border: border::rounded(10)
            .color(palette.primary.strong.color)
            .width(1),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.35),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 16.0,
        },
        ..container::Style::default()
    }
}

pub fn tab_strip(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(palette.background.weak.color)),
        border: border::rounded(10)
            .color(palette.background.strong.color)
            .width(1),
        ..container::Style::default()
    }
}

pub fn method_badge(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        text_color: Some(palette.success.strong.color),
        background: Some(Background::Color(palette.success.weak.color)),
        border: border::rounded(8)
            .color(palette.success.base.color)
            .width(1),
        ..container::Style::default()
    }
}

pub fn tree_row(theme: &Theme, selected: bool, drop_inside: bool) -> container::Style {
    let palette = theme.extended_palette();

    let background = if drop_inside {
        palette.primary.weak.color
    } else if selected {
        palette.secondary.weak.color
    } else {
        Color::TRANSPARENT
    };

    container::Style {
        background: Some(Background::Color(background)),
        border: border::rounded(8)
            .color(if drop_inside {
                palette.primary.strong.color
            } else {
                Color::TRANSPARENT
            })
            .width(if drop_inside { 1 } else { 0 }),
        ..container::Style::default()
    }
}

pub fn drop_line(theme: &Theme, active: bool) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(if active {
            palette.primary.strong.color
        } else {
            Color::TRANSPARENT
        })),
        ..container::Style::default()
    }
}

pub fn tab_chip(theme: &Theme, active: bool) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(if active {
            palette.primary.weak.color
        } else {
            palette.background.base.color
        })),
        border: border::rounded(8)
            .color(if active {
                palette.primary.strong.color
            } else {
                palette.background.strong.color
            })
            .width(1),
        ..container::Style::default()
    }
}

pub fn tab_insert(theme: &Theme, active: bool) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(if active {
            palette.primary.strong.color
        } else {
            Color::TRANSPARENT
        })),
        ..container::Style::default()
    }
}

pub fn handle_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    let mut style = button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: palette.background.strong.text,
        border: border::rounded(6).color(Color::TRANSPARENT).width(0),
        ..button::Style::default()
    };

    if matches!(status, button::Status::Hovered | button::Status::Pressed) {
        style.background = Some(Background::Color(palette.background.weak.color));
    }

    style
}

pub fn menu_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    let mut style = button::Style {
        background: Some(Background::Color(palette.background.base.color)),
        text_color: palette.background.base.text,
        border: border::rounded(6)
            .color(palette.background.strong.color)
            .width(1),
        ..button::Style::default()
    };

    if matches!(status, button::Status::Hovered | button::Status::Pressed) {
        style.background = Some(Background::Color(palette.secondary.weak.color));
    }

    style
}

pub fn secondary_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(palette.secondary.strong.color)),
            text_color: palette.secondary.strong.text,
            border: border::rounded(8)
                .width(1)
                .color(palette.secondary.strong.color),
            ..button::Style::default()
        },
        _ => button::Style {
            background: Some(Background::Color(palette.secondary.base.color)),
            text_color: palette.secondary.base.text,
            border: border::rounded(8)
                .width(1)
                .color(palette.secondary.strong.color),
            ..button::Style::default()
        },
    }
}

pub fn danger_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(palette.danger.strong.color)),
            text_color: palette.danger.strong.text,
            border: border::rounded(8)
                .width(1)
                .color(palette.danger.strong.color),
            ..button::Style::default()
        },
        _ => button::Style {
            background: Some(Background::Color(palette.danger.base.color)),
            text_color: palette.danger.base.text,
            border: border::rounded(8)
                .width(1)
                .color(palette.danger.strong.color),
            ..button::Style::default()
        },
    }
}

pub fn modal_backdrop(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.02, 0.02, 0.03, 0.65))),
        ..container::Style::default()
    }
}

pub fn modal_card(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(palette.background.base.color)),
        border: border::rounded(12)
            .color(palette.background.strong.color)
            .width(1),
        ..container::Style::default()
    }
}
