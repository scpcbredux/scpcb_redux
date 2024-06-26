use crate::game::ui::pause_menu::{components::*, styles::*};
use bevy::prelude::*;

pub fn show_pause_menu(mut pause_menu_query: Query<&mut Visibility, With<PauseMenu>>) {
    if let Ok(mut visibility) = pause_menu_query.get_single_mut() {
        *visibility = Visibility::Inherited;
    }
}

pub fn hide_pause_menu(mut pause_menu_query: Query<&mut Visibility, With<PauseMenu>>) {
    if let Ok(mut visibility) = pause_menu_query.get_single_mut() {
        *visibility = Visibility::Hidden;
    }
}

pub fn build_pause_menu(commands: &mut ChildBuilder, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            NodeBundle {
                // ImageBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                // image: asset_server.load("pause_menu.png").into(),
                ..default()
            },
            PauseMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: PAUSE_BACKGROUND_STYLE,
                    image: asset_server.load("ui/game/pause_menu.jpg").into(),
                    ..default()
                })
                .with_children(|parent| {
                    // === Paused Title ===
                    parent.spawn(
                        TextBundle::from_section(
                            "PAUSED",
                            TextStyle {
                                font: asset_server.load("ui/fonts/courier_new.ttf"),
                                font_size: 44.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_text_justify(JustifyText::Center)
                        .with_style(Style {
                            margin: UiRect {
                                left: Val::Px(90.0),
                                top: Val::Px(8.0),
                                ..default()
                            },
                            ..default()
                        }),
                    );
                    // === Saved Stats (Difficulty, Save, Seed) ===
                    parent
                        .spawn(NodeBundle {
                            style: NODE_ITEMS_STYLE,
                            ..default()
                        })
                        .with_children(|parent| {
                            // === Resume Button ===
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: BUTTON_STYLE,
                                        image: asset_server.load("ui/menu/menu_white.jpg").into(),
                                        ..default()
                                    },
                                    ResumeButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: BUTTON_BACK_STYLE,
                                        image: asset_server.load("ui/menu/menu_black.jpg").into(),
                                        ..default()
                                    });
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "RESUME",
                                            TextStyle {
                                                font: asset_server.load("ui/fonts/courier_new.ttf"),
                                                font_size: BUTTON_FONT_SIZE,
                                                color: BUTTON_FONT_COLOR,
                                            },
                                        )
                                        .with_text_justify(JustifyText::Center),
                                    );
                                });
                            // === Load Game Button ===
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: BUTTON_STYLE,
                                        image: asset_server.load("ui/menu/menu_white.jpg").into(),
                                        ..default()
                                    },
                                    LoadGameButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: BUTTON_BACK_STYLE,
                                        image: asset_server.load("ui/menu/menu_black.jpg").into(),
                                        ..default()
                                    });
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "LOAD GAME",
                                            TextStyle {
                                                font: asset_server.load("ui/fonts/courier_new.ttf"),
                                                font_size: BUTTON_FONT_SIZE,
                                                color: BUTTON_FONT_COLOR,
                                            },
                                        )
                                        .with_text_justify(JustifyText::Center),
                                    );
                                });
                            // === Achievements Button ===
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: BUTTON_STYLE,
                                        image: asset_server.load("ui/menu/menu_white.jpg").into(),
                                        ..default()
                                    },
                                    AchievementsButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: BUTTON_BACK_STYLE,
                                        image: asset_server.load("ui/menu/menu_black.jpg").into(),
                                        ..default()
                                    });
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "ACHIEVEMENTS",
                                            TextStyle {
                                                font: asset_server.load("ui/fonts/courier_new.ttf"),
                                                font_size: BUTTON_FONT_SIZE,
                                                color: BUTTON_FONT_COLOR,
                                            },
                                        )
                                        .with_text_justify(JustifyText::Center),
                                    );
                                });
                            // === Options Button ===
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: BUTTON_STYLE,
                                        image: asset_server.load("ui/menu/menu_white.jpg").into(),
                                        ..default()
                                    },
                                    OptionsButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: BUTTON_BACK_STYLE,
                                        image: asset_server.load("ui/menu/menu_black.jpg").into(),
                                        ..default()
                                    });
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "OPTIONS",
                                            TextStyle {
                                                font: asset_server.load("ui/fonts/courier_new.ttf"),
                                                font_size: BUTTON_FONT_SIZE,
                                                color: BUTTON_FONT_COLOR,
                                            },
                                        )
                                        .with_text_justify(JustifyText::Center),
                                    );
                                });
                            // === Quit Button ===
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: BUTTON_STYLE,
                                        image: asset_server.load("ui/menu/menu_white.jpg").into(),
                                        ..default()
                                    },
                                    QuitButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: BUTTON_BACK_STYLE,
                                        image: asset_server.load("ui/menu/menu_black.jpg").into(),
                                        ..default()
                                    });
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "QUIT",
                                            TextStyle {
                                                font: asset_server.load("ui/fonts/courier_new.ttf"),
                                                font_size: BUTTON_FONT_SIZE,
                                                color: BUTTON_FONT_COLOR,
                                            },
                                        )
                                        .with_text_justify(JustifyText::Center),
                                    );
                                });
                        });
                });
        })
        .id();

    pause_menu_entity
}
