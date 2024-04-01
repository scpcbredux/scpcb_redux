// pub mod video;

use crate::{AppState, StartupCamera};
use bevy::prelude::*;

// use self::video::{Video, VideoClip};

pub struct PreloadPlugin;

impl Plugin for PreloadPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::Preload), spawn_preload);
            // OnExit State Systems
            // .add_systems(OnExit(AppState::Preload), despawn_preload)
            // Systems
            // .add_systems(Update, update_video.run_if(in_state(AppState::Preload)));
    }
}

fn spawn_preload(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn((
    //     ImageBundle {
    //         style: Style {
    //             width: Val::Percent(100.0),
    //             height: Val::Percent(100.0),
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     Video {
    //         handle: asset_server.load("videos/startup_undertow.avi"),
    //         ..default()
    //     },
    // ));

    commands.spawn((Camera2dBundle::default(), StartupCamera));
}

// fn despawn_preload(mut commands: Commands, query: Query<Entity, With<Video>>) {
//     if let Ok(entity) = query.get_single() {
//         commands.entity(entity).despawn();
//     }
// }

// fn update_video(
//     time: Res<Time>,
//     video_clips: Res<Assets<VideoClip>>,
//     mut query: Query<(&mut Video, &mut UiImage)>,
//     mut next_app_state: ResMut<NextState<AppState>>,
// ) {
//     for (mut video, mut ui_image) in query.iter_mut() {
//         // Update the texture with the current frame
//         if let Some(frame) = video.current_frame(&video_clips) {
//             video.update(time.delta());
//             *ui_image = frame.into();
//         } else {
//             // next_app_state.set(AppState::MainMenu);
//         }
//     }
// }
