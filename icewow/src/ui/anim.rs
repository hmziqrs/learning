use std::collections::HashMap;
use std::time::Duration;

use iced::animation::Animation;
use iced::Color;

use crate::model::NodeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ButtonId {
    Send,
    Save,
    DangerConfirm,
    Cancel,
    AddNew,
    // Context menu items
    MenuItem(&'static str),
    // Dynamic buttons keyed by entity
    TabClose(u64),
    FolderToggle(u64),
    ItemMenu(u64),
}

pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

impl From<NodeId> for ButtonId {
    fn from(id: NodeId) -> Self {
        ButtonId::ItemMenu(id.into())
    }
}

#[derive(Debug, Clone)]
pub struct ButtonAnimations {
    hovers: HashMap<ButtonId, Animation<bool>>,
}

impl ButtonAnimations {
    pub fn new() -> Self {
        Self {
            hovers: HashMap::new(),
        }
    }

    pub fn set_hover(&mut self, id: ButtonId, hovered: bool, now: iced::time::Instant) {
        let anim = self
            .hovers
            .entry(id)
            .or_insert_with(|| Animation::new(false).duration(Duration::from_millis(140)));
        anim.go_mut(hovered, now);
    }

    /// Returns 0.0 (not hovered) to 1.0 (fully hovered)
    pub fn hover_t(&self, id: &ButtonId, now: iced::time::Instant) -> f32 {
        self.hovers
            .get(id)
            .map(|a| a.interpolate(0.0f32, 1.0f32, now))
            .unwrap_or(0.0)
    }

    pub fn is_animating(&self, now: iced::time::Instant) -> bool {
        self.hovers.values().any(|a| a.is_animating(now))
    }
}
