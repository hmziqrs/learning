use iced::widget::{column, row, text_input};
use iced::{Element, Length};

use crate::ui::scale::UiScale;

/// Generic key-value pair editor.
///
/// Renders a list of key/value input rows with remove buttons and an "add" button.
/// All message construction is done via closures, so this is feature-agnostic.
pub fn kv_editor<'a, M: Clone + 'static>(
    pairs: &'a [(String, String)],
    key_placeholder: &'a str,
    value_placeholder: &'a str,
    on_key: impl Fn(usize, String) -> M + Clone + 'a,
    on_value: impl Fn(usize, String) -> M + Clone + 'a,
    on_remove: impl Fn(usize) -> M + Clone + 'a,
    on_add_label: &'a str,
    on_add: M,
    scale: &'a UiScale,
) -> Element<'a, M> {
    let mut rows = column![].spacing(scale.space_sm());

    for (i, (key, value)) in pairs.iter().enumerate() {
        let on_key_clone = on_key.clone();
        let on_value_clone = on_value.clone();
        let on_remove_clone = on_remove.clone();

        let key_input = text_input(key_placeholder, key)
            .on_input(move |v| on_key_clone(i, v))
            .width(Length::Fill)
            .size(scale.text_body());

        let value_input = text_input(value_placeholder, value)
            .on_input(move |v| on_value_clone(i, v))
            .width(Length::Fill)
            .size(scale.text_body());

        let remove = crate::ui::components::icon_button(
            crate::ui::icons::lucide_icon("x", scale.icon_sm()),
            scale,
        )
        .on_press(on_remove_clone(i));

        rows = rows.push(
            row![key_input, value_input, remove]
                .spacing(scale.space_sm())
                .align_y(iced::Alignment::Center),
        );
    }

    rows = rows.push(
        crate::ui::components::secondary_button(on_add_label, scale)
            .on_press(on_add)
            .padding([scale.space_sm(), scale.space_md()]),
    );

    column![rows].into()
}
