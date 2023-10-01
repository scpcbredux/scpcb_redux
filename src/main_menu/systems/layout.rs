use bevy::prelude::*;

use crate::main_menu::{components::*, styles::*};

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let _main_menu_entity = build_main_menu(&mut commands, &asset_server);

    commands.spawn((Camera2dBundle::default(), MainMenuCamera));
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/music/menu.wav"),
            settings: PlaybackSettings::LOOP,
        },
        MainMenuMusic,
    ));
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, (With<MainMenu>, Without<MainMenuCamera>, Without<MainMenuMusic>)>,
    main_menu_camera_query: Query<Entity, (With<MainMenuCamera>, Without<MainMenu>, Without<MainMenuMusic>)>,
    // music_controller: Query<&AudioSink, With<MainMenuMusic>>,
    music_controller: Query<Entity, With<MainMenuMusic>>,
) {
    // if let Ok(sink) = music_controller.get_single() {
    //     sink.stop();
    // }
    if let Ok(music) = music_controller.get_single() {
        commands.entity(music).despawn_recursive();
    }

    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }

    if let Ok(main_menu_entity) = main_menu_camera_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: NODE_STYLE,
                background_color: Color::BLACK.into(),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(888.0),
                    height: Val::Px(657.0),
                    ..default()
                },
                image: asset_server.load("ui/menu/back.jpg").into(),
                ..default()
            });
            parent
                .spawn(NodeBundle {
                    style: NODE_ITEMS_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // === New Game Button ===
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                image: asset_server.load("ui/menu/menu_white.jpg").into(),
                                ..default()
                            },
                            NewGameButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: BUTTON_BACK_STYLE,
                                image: asset_server.load("ui/menu/menu_black.jpg").into(),
                                ..default()
                            });
                            parent.spawn(
                                TextBundle::from_section(
                                    "NEW GAME",
                                    TextStyle {
                                        font: asset_server.load("ui/fonts/courier_new.ttf"),
                                        font_size: BUTTON_FONT_SIZE,
                                        color: BUTTON_FONT_COLOR,
                                    },
                                )
                                .with_text_alignment(TextAlignment::Center),
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
                                .with_text_alignment(TextAlignment::Center),
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
                                .with_text_alignment(TextAlignment::Center),
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
                                .with_text_alignment(TextAlignment::Center),
                            );
                        });
                });
        })
        .id();

    main_menu_entity
}
