use bevy::prelude::*;

use crate::{despawn_screen, GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            menu_setup.in_schedule(OnEnter(GameState::Menu)),
            button_system.in_set(OnUpdate(GameState::Menu)),
            despawn_screen::<OnMenuScreen>.in_schedule(OnExit(GameState::Menu)),
            despawn_screen::<Camera2d>.in_schedule(OnExit(GameState::Menu)),
        ));
    }
}

#[derive(Component)]
struct OnMenuScreen;

#[derive(Resource)]
struct MenuMusicController(Handle<AudioSink>);

fn button_system(
    mut game_state: ResMut<NextState<GameState>>,
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Res<MenuMusicController>,
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>),
    >,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Some(sink) = audio_sinks.get(&music_controller.0) {
                    sink.toggle();
                }
                game_state.set(GameState::Game);
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }
}

fn menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let handle = audio_sinks.get_handle(audio.play_with_settings(
        asset_server.load("sounds/music/menu.ogg"),
        PlaybackSettings::LOOP,
    ));
    commands.insert_resource(MenuMusicController(handle));

    // commands.spawn(ImageBundle {
    //     style: Style {
    //         size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    //         ..default()
    //     },
    //     image: asset_server.load("main_menu.png").into(),
    //     ..default()
    // })
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        background_color: Color::BLACK.into(),
        ..default()
    })
    .insert(OnMenuScreen)
    .with_children(|parent| {
        parent.spawn(ImageBundle {
            style: Style {
                size: Size::new(Val::Percent(70.0), Val::Percent(94.0)),
                ..default()
            },
            image: asset_server.load("ui/menu/back.jpg").into(),
            ..default()
        });
        parent.spawn(ImageBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Percent(94.0)),
                flex_grow: 1.0,
                ..default()
            },
            image: asset_server.load("ui/menu/back_extend.jpg").into(),
            ..default()
        });
        // parent.spawn(
        //     TextBundle::from_section(
        //         format!("v{0}", env!("CARGO_PKG_VERSION")),
        //         TextStyle {
        //             font: asset_server.load("ui/fonts/courier_new.ttf"),
        //             font_size: 16.0,
        //             color: Color::WHITE,
        //         },
        //     )
        //     .with_text_alignment(TextAlignment::Center)
        //     .with_style(Style {
        //         position_type: PositionType::Absolute,
        //         align_self: AlignSelf::FlexEnd,
        //         position: UiRect {
        //             left: Val::Percent(1.4),
        //             bottom: Val::Percent(2.0),
        //             ..default()
        //         },
        //         ..default()
        //     })
        // );
        parent.spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::FlexEnd,
                size: Size::new(Val::Percent(30.0), Val::Percent(7.0)),
                position: UiRect {
                    left: Val::Percent(35.0),
                    bottom: Val::Percent(1.5),
                    ..default()
                },
                ..default()
            },
            image: asset_server.load("ui/menu/scp_text.jpg").into(),
            ..default()
        });
        parent.spawn(ImageBundle {
            style: Style {
                position: UiRect {
                    right: Val::Percent(0.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                align_self: AlignSelf::FlexEnd,
                size: Size::new(Val::Percent(17.0), Val::Percent(42.0)),
                ..default()
            },
            image: asset_server.load("ui/menu/173_back.jpg").into(),
            ..default()
        });

        parent.spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(22.0), Val::Percent(6.6)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Percent(30.3),
                    bottom: Val::Percent(18.7),
                    ..default()
                },
                overflow: Overflow::Hidden,
                ..default()
            },
            image: asset_server.load("ui/menu/menu_white.jpg").into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(98.0), Val::Percent(92.0)),
                    ..default()
                },
                image: asset_server.load("ui/menu/menu_black.jpg").into(),
                ..default()
            });
            parent.spawn(TextBundle::from_section(
                "PLAY",
                TextStyle {
                    font: asset_server.load("ui/fonts/courier_new.ttf"),
                    font_size: 44.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                ..default()
            }));
        });
    });
}
