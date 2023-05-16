use bevy::prelude::*;

use crate::{despawn_screen, GameState};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((
            splash_setup.in_schedule(OnEnter(GameState::Splash)),
            splash_update.in_set(OnUpdate(GameState::Splash)),
            despawn_screen::<OnSplashScreen>.in_schedule(OnExit(GameState::Splash)),
        ));
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Component)]
struct SplashPercent;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

#[derive(Resource)]
struct SplashMusicController(Handle<AudioSink>);

fn splash_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let handle = audio_sinks.get_handle(audio.play_with_settings(
        asset_server.load("sounds/music/the_dread.ogg"),
        PlaybackSettings::LOOP,
    ));
    commands.insert_resource(SplashMusicController(handle));

    // commands.spawn(ImageBundle {
    //     style: Style {
    //         align_items: AlignItems::Center,
    //         justify_content: JustifyContent::Center,
    //         size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    //         ..default()
    //     },
    //     image: asset_server.load("loading_screen_template.png").into(),
    //     ..default()
    // })
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::BLACK.into(),
        ..default()
    })
    .insert(OnSplashScreen)
    .with_children(|parent| {
        parent.spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Percent(0.),
                    ..default()
                },
                ..default()
            },
            image: asset_server.load("ui/load_screens/914.jpg").into(),
            ..default()
        });
        parent.spawn((
            TextBundle::from_section(
                format!("LOADING - {} %", 0),
                TextStyle {
                    font: asset_server.load("ui/fonts/courier_new.ttf"),
                    font_size: 11.1,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(35.3),
                    ..default()
                },
                ..default()
            }),
            SplashPercent,
        ));
        parent.spawn(
            TextBundle::from_section(
                "SCP-914",
                TextStyle {
                    font: asset_server.load("ui/fonts/courier_new.ttf"),
                    font_size: 43.3,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Percent(36.),
                    ..default()
                },
                ..default()
            })
        );
        parent.spawn(
            TextBundle::from_section(
                "SCP-914 is a large clockwork device weighing several tons and\ncovering an area of eighteen square meters, consisting of screw\ndrives, belts, pulleys, gears, springs and other clockwork.",
                TextStyle {
                    font: asset_server.load("ui/fonts/courier_new.ttf"),
                    font_size: 11.5,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Percent(28.3),
                    ..default()
                },
                ..default()
            })
        );
    });
    // TODO: Scrap this and add preloading (Only if modding works with preloading)
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn splash_update(
    mut game_state: ResMut<NextState<GameState>>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Res<SplashMusicController>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut query: Query<&mut Text, With<SplashPercent>>,
) {
    if timer.tick(time.delta()).finished() &&
        (!matches!(keys.get_just_pressed().next(), None)
        || !matches!(buttons.get_just_pressed().next(), None)) {
        if let Some(sink) = audio_sinks.get(&music_controller.0) {
            sink.toggle();
        }
        game_state.set(GameState::Menu);
    } else {
        for mut text in &mut query {
            text.sections[0].value = format!("LOADING - {} %", (timer.elapsed_secs() * 100.) as i32);
        }
    }
}
