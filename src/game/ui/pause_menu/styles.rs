use bevy::prelude::*;

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    overflow: Overflow::Hidden,
    size: Size::new(Val::Px(280.0), Val::Px(50.0)),
    ..Style::DEFAULT
};

pub const BUTTON_BACK_STYLE: Style = Style {
    position_type: PositionType::Absolute,
    size: Size::new(Val::Percent(98.0), Val::Percent(90.0)),
    ..Style::DEFAULT
};

pub const BUTTON_FONT_COLOR: Color = Color::WHITE;
pub const BUTTON_FONT_SIZE: f32 = 44.0;

pub const PAUSE_BACKGROUND_STYLE: Style = Style {
    size: Size {
        width: Val::Px(420.0),
        height: Val::Px(420.0),
    },
    flex_direction: FlexDirection::Column,
    ..Style::DEFAULT
};

pub const NODE_ITEMS_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Start,
    align_items: AlignItems::Center,
    position: UiRect {
        left: Val::Px(28.0),
        top: Val::Px(20.0),
        ..UiRect::DEFAULT
    },
    gap: Size::new(Val::Px(20.0), Val::Px(20.0)),
    ..Style::DEFAULT
};
