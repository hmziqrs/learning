/// UI density preset — controls padding and spacing multipliers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Density {
    Compact,
    Comfortable,
    Spacious,
}

/// All configurable UI dimensions in one place.
/// Stored in AppState, passed to view functions via `&UiScale`.
/// Can be serialized to disk for persistence.
#[derive(Debug, Clone)]
pub struct UiScale {
    pub density: Density,
    pub font_scale: f32,
}

impl UiScale {
    // --- Text sizes (scaled by font_scale) ---

    pub fn text_caption(&self) -> f32 {
        10.0 * self.font_scale
    }
    pub fn text_small(&self) -> f32 {
        12.0 * self.font_scale
    }
    pub fn text_body(&self) -> f32 {
        13.0 * self.font_scale
    }
    pub fn text_label(&self) -> f32 {
        14.0 * self.font_scale
    }
    pub fn text_title(&self) -> f32 {
        16.0 * self.font_scale
    }
    pub fn text_heading(&self) -> f32 {
        20.0 * self.font_scale
    }

    // --- Icon sizes (scaled by font_scale) ---

    pub fn icon_sm(&self) -> f32 {
        14.0 * self.font_scale
    }
    pub fn icon_md(&self) -> f32 {
        16.0 * self.font_scale
    }
    pub fn icon_lg(&self) -> f32 {
        20.0 * self.font_scale
    }

    // --- Spacing (scaled by density) ---

    fn density_factor(&self) -> f32 {
        match self.density {
            Density::Compact => 0.75,
            Density::Comfortable => 1.0,
            Density::Spacious => 1.35,
        }
    }

    pub fn space_xs(&self) -> f32 {
        (2.0 * self.density_factor()).round()
    }
    pub fn space_sm(&self) -> f32 {
        (4.0 * self.density_factor()).round()
    }
    pub fn space_md(&self) -> f32 {
        (8.0 * self.density_factor()).round()
    }
    pub fn space_lg(&self) -> f32 {
        (12.0 * self.density_factor()).round()
    }
    pub fn space_xl(&self) -> f32 {
        (16.0 * self.density_factor()).round()
    }

    // --- Padding presets (scaled by density) ---

    pub fn pad_chip(&self) -> [f32; 2] {
        [
            4.0 * self.density_factor(),
            10.0 * self.density_factor(),
        ]
    }
    pub fn pad_button(&self) -> [f32; 2] {
        [
            6.0 * self.density_factor(),
            12.0 * self.density_factor(),
        ]
    }
    pub fn pad_input(&self) -> f32 {
        (10.0 * self.density_factor()).round()
    }
    pub fn pad_panel(&self) -> f32 {
        (10.0 * self.density_factor()).round()
    }

    // --- Layout constants (not scaled -- structural) ---

    pub const SIDEBAR_WIDTH: f32 = 280.0;
    pub const TREE_INDENT: f32 = 16.0;
    pub const RESPONSE_MIN_HEIGHT: f32 = 200.0;
    pub const MODAL_WIDTH: f32 = 380.0;
    pub const CONTEXT_MENU_WIDTH: f32 = 210.0;

    // --- Border radii (not scaled) ---

    pub const RADIUS_SM: f32 = 4.0;
    pub const RADIUS_MD: f32 = 6.0;
    pub const RADIUS_LG: f32 = 8.0;
}

impl Default for UiScale {
    fn default() -> Self {
        Self {
            density: Density::Comfortable,
            font_scale: 1.0,
        }
    }
}
