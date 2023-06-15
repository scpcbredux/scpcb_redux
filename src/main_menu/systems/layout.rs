use bevy::prelude::*;

use crate::main_menu::{components::*, resources::*, styles::*};

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let _main_menu_entity = build_main_menu(&mut commands, &asset_server);

    let handle = audio_sinks.get_handle(audio.play_with_settings(
        asset_server.load("sounds/music/menu.ogg"),
        PlaybackSettings::LOOP,
    ));
    commands.insert_resource(MusicController(handle));

    commands.spawn((Camera2dBundle::default(), MainMenuCamera));
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, (With<MainMenu>, Without<MainMenuCamera>)>,
    main_menu_camera_query: Query<Entity, (With<MainMenuCamera>, Without<MainMenu>)>,
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Res<MusicController>,
) {
    if let Some(sink) = audio_sinks.get(&music_controller.0) {
        sink.toggle();
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
                    size: Size::new(Val::Px(888.0), Val::Px(657.0)),
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
