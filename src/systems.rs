use bevy::prelude::*;
use bevy_rmesh::ROOM_SCALE;

use crate::game::player::PlayerCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 70.0_f32.to_radians(),
                near: 3.0 * ROOM_SCALE,
                far: 200.0 * ROOM_SCALE,
                ..Default::default()
            }),
            ..Default::default()
        },
        PlayerCamera,
    ));
}
