use iced::widget::{button, container, pick_list};
use iced::{border, Background, Color, Shadow, Theme, Vector};

use crate::model::HttpMethod;

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

pub fn method_badge(theme: &Theme, method: HttpMethod) -> container::Style {
    let palette = theme.extended_palette();

    let (text, bg, border_color) = match method {
        HttpMethod::Get => (
            palette.success.strong.color,
            palette.success.weak.color,
            palette.success.base.color,
        ),
        HttpMethod::Post => (
            palette.warning.strong.color,
            palette.warning.weak.color,
            palette.warning.base.color,
        ),
        HttpMethod::Put => (
            palette.primary.strong.color,
            palette.primary.weak.color,
            palette.primary.base.color,
        ),
        HttpMethod::Delete => (
            palette.danger.strong.color,
            palette.danger.weak.color,
            palette.danger.base.color,
        ),
        HttpMethod::Patch => (
            palette.secondary.strong.color,
            palette.secondary.weak.color,
            palette.secondary.base.color,
        ),
    };

    container::Style {
        text_color: Some(text),
        background: Some(Background::Color(bg)),
        border: border::rounded(8).color(border_color).width(1),
        ..container::Style::default()
    }
}

pub fn send_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    let base = button::Style {
        background: Some(Background::Color(palette.primary.strong.color)),
        text_color: palette.primary.strong.text,
        border: border::rounded(8)
            .width(1)
            .color(palette.primary.strong.color),
        ..button::Style::default()
    };

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(palette.primary.base.color)),
            text_color: palette.primary.base.text,
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(palette.primary.weak.color)),
            text_color: palette.primary.weak.text,
            ..base
        },
        _ => base,
    }
}

pub fn method_pick_list(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let palette = theme.extended_palette();

    let background = match status {
        pick_list::Status::Hovered => palette.background.weak.color,
        pick_list::Status::Opened { .. } => palette.primary.weak.color,
        _ => palette.background.base.color,
    };

    pick_list::Style {
        text_color: palette.background.base.text,
        background: Background::Color(background),
        border: border::rounded(8)
            .width(1)
            .color(palette.background.strong.color),
        placeholder_color: palette.background.strong.color,
        handle_color: palette.background.strong.text,
    }
}

pub fn response_panel(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(Background::Color(palette.background.base.color)),
        border: border::rounded(8)
            .color(palette.background.strong.color)
            .width(1),
        ..container::Style::default()
    }
}

pub fn status_badge(theme: &Theme, status_code: u16) -> container::Style {
    let palette = theme.extended_palette();

    let (text, bg, border_color) = if status_code >= 200 && status_code < 300 {
        (
            palette.success.strong.color,
            palette.success.weak.color,
            palette.success.base.color,
        )
    } else if status_code >= 400 && status_code < 500 {
        (
            palette.warning.strong.color,
            palette.warning.weak.color,
            palette.warning.base.color,
        )
    } else {
        (
            palette.danger.strong.color,
            palette.danger.weak.color,
            palette.danger.base.color,
        )
    };

    container::Style {
        text_color: Some(text),
        background: Some(Background::Color(bg)),
        border: border::rounded(6).color(border_color).width(1),
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

pub fn body_type_button(theme: &Theme, status: button::Status, active: bool) -> button::Style {
    let palette = theme.extended_palette();

    let bg = if active {
        palette.primary.weak.color
    } else if matches!(status, button::Status::Hovered) {
        palette.background.weak.color
    } else {
        palette.background.base.color
    };

    button::Style {
        background: Some(Background::Color(bg)),
        text_color: if active {
            palette.primary.strong.color
        } else {
            palette.background.base.text
        },
        border: border::rounded(8)
            .color(if active {
                palette.primary.strong.color
            } else {
                palette.background.strong.color
            })
            .width(1),
        ..button::Style::default()
    }
}
