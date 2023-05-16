use bevy::prelude::*;

use crate::game::player::PlayerCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        PlayerCamera,
    ));
}
