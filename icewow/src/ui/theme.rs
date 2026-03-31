use iced::Color;
use iced::Theme;
use iced::theme::Palette;

// ============================================================
// Iced Palette (feeds auto-generation — we override most of it
// in style functions, but Iced still needs this for defaults)
// ============================================================
const ICEWOW_DARK: Palette = Palette {
    background: Color::from_rgb8(0x09, 0x09, 0x0b), // zinc-950
    text: Color::from_rgb8(0xfa, 0xfa, 0xfa),       // zinc-50
    primary: Color::from_rgb8(0x3b, 0x82, 0xf6),    // blue-500
    success: Color::from_rgb8(0x22, 0xc5, 0x5e),    // green-500
    warning: Color::from_rgb8(0xea, 0xb3, 0x08),    // yellow-500
    danger: Color::from_rgb8(0xef, 0x44, 0x44),     // red-500
};

pub fn theme() -> Theme {
    Theme::custom("IceWow Dark", ICEWOW_DARK)
}

// ============================================================
// Tailwind Zinc Scale
// ============================================================
pub mod zinc {
    use iced::Color;
    pub const S50: Color = Color::from_rgb8(0xfa, 0xfa, 0xfa);
    pub const S100: Color = Color::from_rgb8(0xf4, 0xf4, 0xf5);
    pub const S200: Color = Color::from_rgb8(0xe4, 0xe4, 0xe7);
    pub const S300: Color = Color::from_rgb8(0xd4, 0xd4, 0xd8);
    pub const S400: Color = Color::from_rgb8(0xa1, 0xa1, 0xaa);
    pub const S500: Color = Color::from_rgb8(0x71, 0x71, 0x7a);
    pub const S600: Color = Color::from_rgb8(0x52, 0x52, 0x5b);
    pub const S700: Color = Color::from_rgb8(0x3f, 0x3f, 0x46);
    pub const S800: Color = Color::from_rgb8(0x27, 0x27, 0x2a);
    pub const S900: Color = Color::from_rgb8(0x18, 0x18, 0x1b);
    pub const S950: Color = Color::from_rgb8(0x09, 0x09, 0x0b);
}

// ============================================================
// Tailwind Color Scales (shades used in the app)
// ============================================================
pub mod green {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0x4a, 0xde, 0x80);
    pub const S500: Color = Color::from_rgb8(0x22, 0xc5, 0x5e);
    pub const S600: Color = Color::from_rgb8(0x16, 0xa3, 0x4a);
    pub const S950: Color = Color::from_rgb8(0x05, 0x2e, 0x16);
}

pub mod yellow {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0xfa, 0xcc, 0x15);
    pub const S500: Color = Color::from_rgb8(0xea, 0xb3, 0x08);
    pub const S600: Color = Color::from_rgb8(0xca, 0x8a, 0x04);
    pub const S950: Color = Color::from_rgb8(0x42, 0x20, 0x06);
}

pub mod sky {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0x38, 0xbd, 0xf8);
    pub const S500: Color = Color::from_rgb8(0x0e, 0xa5, 0xe9);
    pub const S600: Color = Color::from_rgb8(0x02, 0x84, 0xc7);
    pub const S950: Color = Color::from_rgb8(0x08, 0x2f, 0x49);
}

pub mod red {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0xf8, 0x71, 0x71);
    pub const S500: Color = Color::from_rgb8(0xef, 0x44, 0x44);
    pub const S600: Color = Color::from_rgb8(0xdc, 0x26, 0x26);
    pub const S900: Color = Color::from_rgb8(0x7f, 0x1d, 0x1d);
    pub const S950: Color = Color::from_rgb8(0x45, 0x0a, 0x0a);
}

pub mod violet {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0xa7, 0x8b, 0xfa);
    pub const S500: Color = Color::from_rgb8(0x8b, 0x5c, 0xf6);
    pub const S600: Color = Color::from_rgb8(0x7c, 0x3a, 0xed);
    pub const S950: Color = Color::from_rgb8(0x2e, 0x10, 0x65);
}

pub mod blue {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0x60, 0xa5, 0xfa);
    pub const S500: Color = Color::from_rgb8(0x3b, 0x82, 0xf6);
    pub const S600: Color = Color::from_rgb8(0x25, 0x63, 0xeb);
    pub const S700: Color = Color::from_rgb8(0x1d, 0x4e, 0xd8);
    pub const S950: Color = Color::from_rgb8(0x17, 0x25, 0x54);
}

pub mod orange {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0xfb, 0x92, 0x3c);
    pub const S500: Color = Color::from_rgb8(0xf9, 0x73, 0x16);
    pub const S600: Color = Color::from_rgb8(0xea, 0x58, 0x0c);
    pub const S950: Color = Color::from_rgb8(0x43, 0x14, 0x07);
}

pub mod emerald {
    use iced::Color;
    pub const S400: Color = Color::from_rgb8(0x34, 0xd3, 0x99);
    pub const S500: Color = Color::from_rgb8(0x10, 0xb9, 0x81);
    pub const S600: Color = Color::from_rgb8(0x05, 0x96, 0x69);
    pub const S950: Color = Color::from_rgb8(0x02, 0x2c, 0x22);
}

// ============================================================
// shadcn Semantic Tokens (mapped from zinc scale)
// ============================================================

/// App background — zinc-950
pub const BACKGROUND: Color = zinc::S950;
/// Primary text — zinc-50
pub const FOREGROUND: Color = zinc::S50;

/// Card / popover surface — zinc-900
pub const CARD: Color = zinc::S900;
/// Card text — zinc-50
pub const CARD_FOREGROUND: Color = zinc::S50;

/// Primary accent — blue-500
pub const PRIMARY: Color = blue::S500;
/// Text on primary — zinc-50
pub const PRIMARY_FOREGROUND: Color = zinc::S50;

/// Secondary surfaces — zinc-800
pub const SECONDARY: Color = zinc::S800;
/// Secondary text — zinc-50
pub const SECONDARY_FOREGROUND: Color = zinc::S50;

/// Muted surfaces — zinc-800
pub const MUTED: Color = zinc::S800;
/// Muted text — zinc-400
pub const MUTED_FOREGROUND: Color = zinc::S400;

/// Accent surface (hover bg) — zinc-800
pub const ACCENT: Color = zinc::S800;
/// Accent text — zinc-50
pub const ACCENT_FOREGROUND: Color = zinc::S50;

/// Destructive — red-500
pub const DESTRUCTIVE: Color = red::S500;
/// Destructive foreground — zinc-50
pub const DESTRUCTIVE_FOREGROUND: Color = zinc::S50;

/// Border — white at 10% opacity (shadcn v4 style)
pub const BORDER: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.10 };
/// Input border — white at 15% opacity
pub const INPUT: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.15 };
/// Focus ring — zinc-500
pub const RING: Color = zinc::S500;

/// Sidebar background — zinc-900
pub const SIDEBAR: Color = zinc::S900;
/// Sidebar text — zinc-50
pub const SIDEBAR_FOREGROUND: Color = zinc::S50;
/// Sidebar accent (hover/selected) — zinc-800
pub const SIDEBAR_ACCENT: Color = zinc::S800;
/// Sidebar border — white at 10%
pub const SIDEBAR_BORDER: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.10 };

// ============================================================
// Opacity helpers
// ============================================================

pub const fn with_alpha(c: Color, a: f32) -> Color {
    Color { r: c.r, g: c.g, b: c.b, a }
}

pub const WHITE_5: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.05 };
pub const WHITE_10: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.10 };
pub const WHITE_15: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.15 };
pub const WHITE_20: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.20 };

// ============================================================
// Method badge colors
// ============================================================

pub struct MethodColors {
    pub text: Color,
    pub bg: Color,
    pub border: Color,
}

pub fn method_colors(method: crate::model::HttpMethod) -> MethodColors {
    use crate::model::HttpMethod;
    match method {
        HttpMethod::Get => MethodColors {
            text: green::S400, bg: green::S950, border: green::S600,
        },
        HttpMethod::Post => MethodColors {
            text: yellow::S400, bg: yellow::S950, border: yellow::S600,
        },
        HttpMethod::Put => MethodColors {
            text: sky::S400, bg: sky::S950, border: sky::S600,
        },
        HttpMethod::Delete => MethodColors {
            text: red::S400, bg: red::S950, border: red::S600,
        },
        HttpMethod::Patch => MethodColors {
            text: violet::S400, bg: violet::S950, border: violet::S600,
        },
    }
}

pub fn method_text_color(method: crate::model::HttpMethod) -> Color {
    method_colors(method).text
}
