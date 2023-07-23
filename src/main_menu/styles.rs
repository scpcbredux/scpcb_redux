use bevy::prelude::*;

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    // style.overflow = Overflow::Hidden;
    style.width = Val::Px(280.0);
    style.height = Val::Px(50.0);
    style
};

pub const BUTTON_BACK_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute;
    style.width = Val::Percent(98.0);
    style.height = Val::Percent(90.0);
    style
};

pub const BUTTON_FONT_COLOR: Color = Color::WHITE;
pub const BUTTON_FONT_SIZE: f32 = 44.0;

pub const NODE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    // style.flex_direction = FlexDirection::Column;
    // style.justify_content = JustifyContent::Center;
    // style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    // style.row_gap = Val::Px(20.0);
    // style.column_gap = Val::Px(20.0);
    style
};

pub const NODE_ITEMS_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Start;
    style.align_items = AlignItems::Start;
    style.border = UiRect {
        top: Val::Px(200.0),
        left: Val::Px(112.0),
        ..UiRect::DEFAULT
    };
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(20.0);
    style.column_gap = Val::Px(20.0);
    style
};
