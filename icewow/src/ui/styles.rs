use iced::widget::{button, container, pick_list};
use iced::{border, Background, Color, Shadow, Theme, Vector};

use crate::model::HttpMethod;
use crate::ui::theme::*;

pub fn panel(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(CARD)),
        border: border::rounded(8).color(BORDER).width(1),
        ..container::Style::default()
    }
}

pub fn sidebar_panel(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SIDEBAR)),
        border: border::rounded(0).color(SIDEBAR_BORDER).width(0.0),
        ..container::Style::default()
    }
}

pub fn context_menu(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(CARD)),
        border: border::rounded(8).color(BORDER).width(1),
        ..container::Style::default()
    }
}

pub fn drag_preview(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(FOREGROUND),
        background: Some(Background::Color(CARD)),
        border: border::rounded(8).color(PRIMARY).width(1),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.35),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 16.0,
        },
        ..container::Style::default()
    }
}

pub fn tab_strip(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BACKGROUND)),
        border: border::rounded(0).color(BORDER).width(0.0),
        ..container::Style::default()
    }
}

pub fn method_badge(_theme: &Theme, method: HttpMethod) -> container::Style {
    let colors = theme_method_colors(method);
    container::Style {
        text_color: Some(colors.text),
        background: Some(Background::Color(colors.bg)),
        border: border::rounded(6).color(colors.border).width(1),
        ..container::Style::default()
    }
}

fn theme_method_colors(method: HttpMethod) -> MethodColors {
    crate::ui::theme::method_colors(method)
}

pub fn send_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(PRIMARY)),
        text_color: PRIMARY_FOREGROUND,
        border: border::rounded(8).width(1).color(PRIMARY),
        ..button::Style::default()
    };

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(blue::S600)),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(blue::S700)),
            ..base
        },
        _ => base,
    }
}

pub fn method_pick_list(_theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let background = match status {
        pick_list::Status::Hovered => SECONDARY,
        pick_list::Status::Opened { .. } => with_alpha(PRIMARY, 0.15),
        _ => CARD,
    };

    pick_list::Style {
        text_color: FOREGROUND,
        background: Background::Color(background),
        border: border::rounded(8).width(1).color(INPUT),
        placeholder_color: MUTED_FOREGROUND,
        handle_color: MUTED_FOREGROUND,
    }
}

pub fn response_panel(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BACKGROUND)),
        border: border::rounded(6).color(BORDER).width(1),
        ..container::Style::default()
    }
}

pub fn status_badge(_theme: &Theme, status_code: u16) -> container::Style {
    let (text, bg, border_color) = if status_code >= 200 && status_code < 300 {
        (emerald::S400, emerald::S950, emerald::S600)
    } else if status_code >= 400 && status_code < 500 {
        (orange::S400, orange::S950, orange::S600)
    } else {
        (red::S400, red::S950, red::S600)
    };

    container::Style {
        text_color: Some(text),
        background: Some(Background::Color(bg)),
        border: border::rounded(6).color(border_color).width(1),
        ..container::Style::default()
    }
}

pub fn tree_row(_theme: &Theme, selected: bool, drop_inside: bool) -> container::Style {
    let background = if drop_inside {
        with_alpha(PRIMARY, 0.15)
    } else if selected {
        SIDEBAR_ACCENT
    } else {
        Color::TRANSPARENT
    };

    container::Style {
        background: Some(Background::Color(background)),
        border: border::rounded(6)
            .color(if drop_inside { PRIMARY } else { Color::TRANSPARENT })
            .width(if drop_inside { 1 } else { 0 }),
        ..container::Style::default()
    }
}

pub fn drop_line(_theme: &Theme, active: bool) -> container::Style {
    container::Style {
        background: Some(Background::Color(if active {
            PRIMARY
        } else {
            Color::TRANSPARENT
        })),
        ..container::Style::default()
    }
}

pub fn tab_chip(_theme: &Theme, active: bool) -> container::Style {
    if active {
        container::Style {
            background: Some(Background::Color(CARD)),
            text_color: Some(FOREGROUND),
            border: border::rounded(0).color(PRIMARY).width(0.0),
            ..container::Style::default()
        }
    } else {
        container::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: Some(MUTED_FOREGROUND),
            border: border::rounded(0).color(Color::TRANSPARENT).width(0.0),
            ..container::Style::default()
        }
    }
}

pub fn tab_insert(_theme: &Theme, active: bool) -> container::Style {
    container::Style {
        background: Some(Background::Color(if active {
            PRIMARY
        } else {
            Color::TRANSPARENT
        })),
        ..container::Style::default()
    }
}

pub fn handle_button(_theme: &Theme, status: button::Status) -> button::Style {
    let mut style = button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: MUTED_FOREGROUND,
        border: border::rounded(6).color(Color::TRANSPARENT).width(0),
        ..button::Style::default()
    };

    if matches!(status, button::Status::Hovered | button::Status::Pressed) {
        style.background = Some(Background::Color(WHITE_5));
    }

    style
}

pub fn menu_button(_theme: &Theme, status: button::Status) -> button::Style {
    let mut style = button::Style {
        background: Some(Background::Color(CARD)),
        text_color: FOREGROUND,
        border: border::rounded(6).color(BORDER).width(1),
        ..button::Style::default()
    };

    if matches!(status, button::Status::Hovered | button::Status::Pressed) {
        style.background = Some(Background::Color(ACCENT));
    }

    style
}

pub fn secondary_button(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered | button::Status::Pressed => button::Style {
            background: Some(Background::Color(ACCENT)),
            text_color: ACCENT_FOREGROUND,
            border: border::rounded(8).width(1).color(BORDER),
            ..button::Style::default()
        },
        _ => button::Style {
            background: Some(Background::Color(SECONDARY)),
            text_color: SECONDARY_FOREGROUND,
            border: border::rounded(8).width(1).color(BORDER),
            ..button::Style::default()
        },
    }
}

pub fn danger_button(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered | button::Status::Pressed => button::Style {
            background: Some(Background::Color(red::S600)),
            text_color: DESTRUCTIVE_FOREGROUND,
            border: border::rounded(8).width(1).color(red::S600),
            ..button::Style::default()
        },
        _ => button::Style {
            background: Some(Background::Color(red::S900)),
            text_color: DESTRUCTIVE_FOREGROUND,
            border: border::rounded(8).width(1).color(red::S600),
            ..button::Style::default()
        },
    }
}

pub fn modal_backdrop(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.60))),
        ..container::Style::default()
    }
}

pub fn modal_card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(CARD)),
        border: border::rounded(12).color(BORDER).width(1),
        ..container::Style::default()
    }
}

pub fn body_type_button(_theme: &Theme, status: button::Status, active: bool) -> button::Style {
    let bg = if active {
        blue::S950
    } else if matches!(status, button::Status::Hovered) {
        SECONDARY
    } else {
        Color::TRANSPARENT
    };

    button::Style {
        background: Some(Background::Color(bg)),
        text_color: if active { blue::S400 } else { MUTED_FOREGROUND },
        border: border::rounded(6)
            .color(if active { PRIMARY } else { Color::TRANSPARENT })
            .width(if active { 1 } else { 0 }),
        ..button::Style::default()
    }
}

// ============================================================
// New styles for request/response section tabs
// ============================================================

pub fn section_tab(_theme: &Theme, active: bool) -> button::Style {
    if active {
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: FOREGROUND,
            border: border::rounded(0)
                .width(0),
            ..button::Style::default()
        }
    } else {
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: MUTED_FOREGROUND,
            border: border::rounded(0)
                .width(0),
            ..button::Style::default()
        }
    }
}

pub fn save_button(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered | button::Status::Pressed => button::Style {
            background: Some(Background::Color(blue::S600)),
            text_color: PRIMARY_FOREGROUND,
            border: border::rounded(6).width(1).color(blue::S600),
            ..button::Style::default()
        },
        _ => button::Style {
            background: Some(Background::Color(PRIMARY)),
            text_color: PRIMARY_FOREGROUND,
            border: border::rounded(6).width(1).color(PRIMARY),
            ..button::Style::default()
        },
    }
}

pub fn content_area(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(CARD)),
        border: border::rounded(0).color(BORDER).width(0),
        ..container::Style::default()
    }
}

pub fn section_divider(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BORDER)),
        ..container::Style::default()
    }
}
