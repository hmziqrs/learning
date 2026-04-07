# Plan: Smooth Hover Animations for Buttons

## Context

All buttons currently use instant color changes on hover/press via `button::Status` matching in style closures. We want smooth ~140ms background color transitions using Iced 0.14's `Animation<bool>` API. Pressed state stays instant for snappy feedback.

**Key constraint**: `Color` doesn't implement `Interpolable` (lilt trait). We interpolate `f32` (0.0→1.0) and manually lerp colors.

## Files Overview

| File | Change |
|------|--------|
| `src/ui/anim.rs` | **NEW** — `ButtonId`, `ButtonAnimations`, `lerp_color` |
| `src/ui/mod.rs` | Add `pub mod anim` |
| `src/model.rs` | Add `button_anims: ButtonAnimations` to `AppState` |
| `src/app.rs` | Add `ButtonHover`/`AnimFrame` messages, subscription, update |
| `src/ui/styles.rs` | Add `hover_t: f32` param to 6 button style fns |
| `src/ui/components/buttons.rs` | Add `hover_t: f32` param, wrap with `MouseArea` |
| `src/features/{sidebar,editor,tabs,response}.rs` | Add `ButtonHover` variant to each msg enum |

## Step 1: Animation Infrastructure (`src/ui/anim.rs`)

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};
use iced::animation::Animation;
use iced::Color;

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
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}

#[derive(Debug, Clone)]
pub struct ButtonAnimations {
    hovers: HashMap<ButtonId, Animation<bool>>,
    pub now: Instant,
}

impl ButtonAnimations {
    pub fn new() -> Self { /* HashMap::new(), Instant::now() */ }

    pub fn set_hover(&mut self, id: ButtonId, hovered: bool) {
        self.now = Instant::now();
        let anim = self.hovers.entry(id).or_insert_with(||
            Animation::new(false).duration(Duration::from_millis(140))
        );
        anim.go_mut(hovered, self.now);
    }

    pub fn tick(&mut self, now: Instant) { self.now = now; }

    pub fn is_animating(&self) -> bool {
        self.hovers.values().any(|a| a.is_animating(self.now))
    }

    /// Returns 0.0 (not hovered) to 1.0 (fully hovered)
    pub fn hover_t(&self, id: &ButtonId) -> f32 {
        self.hovers.get(id)
            .map(|a| a.interpolate(0.0f32, 1.0f32, self.now))
            .unwrap_or(0.0)
    }
}
```

## Step 2: Wire into AppState (`src/model.rs`)

Add `button_anims: ButtonAnimations` to `AppState`, initialize in `sample()`.

## Step 3: Messages & Update (`src/app.rs`)

```rust
pub enum Message {
    // ... existing
    ButtonHover(ButtonId, bool),
    AnimFrame(Instant),
}
```

In `update()`:
```rust
Message::ButtonHover(id, hovered) => {
    self.state.button_anims.set_hover(id, hovered);
}
Message::AnimFrame(now) => {
    self.state.button_anims.tick(now);
}
```

Intercept feature-level ButtonHover before delegating:
```rust
Message::Sidebar(SidebarMsg::ButtonHover(id, h)) |
Message::Editor(EditorMsg::ButtonHover(id, h)) |
... => {
    self.state.button_anims.set_hover(id, h);
}
```

## Step 4: Subscription (`src/app.rs`)

```rust
let anim_sub = if self.state.button_anims.is_animating() {
    window::frames().map(Message::AnimFrame)
} else {
    Subscription::none()
};
Subscription::batch(vec![pointer_sub, resize_events, anim_sub])
```

## Step 5: Style Functions (`src/ui/styles.rs`)

Add `hover_t: f32` to each button style fn. Example for `send_button`:

```rust
pub fn send_button(_theme: &Theme, status: button::Status, hover_t: f32) -> button::Style {
    let bg = lerp_color(PRIMARY, blue::S600, hover_t);
    let base = button::Style {
        background: Some(Background::Color(bg)),
        text_color: PRIMARY_FOREGROUND,
        border: border::rounded(UiScale::RADIUS_MD).width(0.0).color(PRIMARY),
        ..button::Style::default()
    };
    match status {
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(blue::S700)),
            ..base
        },
        _ => base,
    }
}
```

Same pattern for: `save_button`, `menu_button`, `secondary_button`, `danger_button`, `handle_button`.

**Not animated** (no bg hover transition): `section_tab`, `body_type_button`.

## Step 6: Button Components (`src/ui/components/buttons.rs`)

Add `hover_t: f32` parameter. Return type stays `widget::Button` — callers handle MouseArea wrapping.

```rust
pub fn icon_button<'a, M: 'a>(
    icon: impl Into<Element<'a, M>>,
    scale: &UiScale,
    hover_t: f32,
) -> widget::Button<'a, M> {
    button(icon)
        .padding(scale.pad_icon())
        .style(move |theme, status| styles::handle_button(theme, status, hover_t))
}
```

## Step 7: View Call Sites — MouseArea Wrapping

At each call site, get `hover_t` from state, wrap with `MouseArea`:

```rust
// Example: send button in app.rs (already uses global Message)
let send_id = ButtonId::Send;
let hover_t = state.button_anims.hover_t(&send_id);

let send_btn = button(text(send_label).size(scale.text_label()))
    .on_press_maybe(...)
    .padding(...)
    .style(move |theme, status| styles::send_button(theme, status, hover_t));

let send_btn = MouseArea::new(send_btn)
    .on_enter(Message::ButtonHover(send_id.clone(), true))
    .on_exit(Message::ButtonHover(send_id, false));
```

For feature views (sidebar, editor, etc.), emit the feature's `ButtonHover` variant:
```rust
// In sidebar view
let hover_t = state.button_anims.hover_t(&ButtonId::ItemMenu(node_id.into()));
let btn = icon_button(icon, scale, hover_t).on_press(SidebarMsg::ShowMenu(node_id));
MouseArea::new(btn)
    .on_enter(SidebarMsg::ButtonHover(ButtonId::ItemMenu(node_id.into()), true))
    .on_exit(SidebarMsg::ButtonHover(ButtonId::ItemMenu(node_id.into()), false))
```

## Step 8: Feature Message Enums

Add to each feature msg enum:
```rust
ButtonHover(ButtonId, bool),
```

Files: `sidebar.rs`, `editor.rs`, `tabs.rs`, `response.rs`.

## Verification

1. `cargo check` — compiles without errors
2. `cargo run` — hover over buttons, observe smooth 140ms bg color transition
3. Click buttons — pressed state is instant, no delay
4. Rapidly hover in/out — animation reverses mid-transition smoothly
5. Check CPU — `window::frames()` subscription only active during animation, idle otherwise
