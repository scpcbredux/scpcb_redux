use bevy::{input::mouse::MouseMotion, math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::prelude::*;
use std::f32::consts::*;

use crate::game::player::{components::*, resources::*, ANGLE_EPSILON};

pub fn player_input(
    settings: Res<MovementSettings>,
    keys: Res<Input<KeyCode>>,
    windows: Query<&mut Window>,
    mut motion_evr: EventReader<MouseMotion>,
    mut input: ResMut<PlayerInput>,
) {
    let window = windows.single();
    if window.focused {
        let mut delta = Vec2::ZERO;
        for ev in motion_evr.iter() {
            delta += ev.delta;
        }
        delta *= settings.sensitivity;

        input.pitch =
            (input.pitch - delta.y).clamp(-FRAC_PI_2 + ANGLE_EPSILON, FRAC_PI_2 - ANGLE_EPSILON);
        input.yaw -= delta.x;
        if input.yaw.abs() > PI {
            input.yaw = input.yaw.rem_euclid(TAU);
        }
    }

    input.movement = Vec3::new(
        get_input_axis(settings.key_right, settings.key_left, &keys),
        0.0,
        get_input_axis(settings.key_up, settings.key_down, &keys),
    )
    .normalize_or_zero();

    input.sprinting = keys.pressed(settings.key_sprint);
}

pub fn player_look(
    mut q_player: Query<(&Transform, &Player), Without<PlayerCamera>>,
    mut q_camera: Query<&mut Transform, (Without<Player>, With<PlayerCamera>)>,
    input: Res<PlayerInput>,
) {
    for (p_transform, player) in &mut q_player {
        if let Ok(mut c_transform) = q_camera.get_single_mut() {
            c_transform.translation = p_transform.translation + Vec3::Y * player.height;
            c_transform.rotation = Quat::from_euler(EulerRot::YXZ, input.yaw, input.pitch, 0.0);
        }
    }
}

pub fn player_move(mut query: Query<(&Player, &mut Velocity)>, input: Res<PlayerInput>) {
    for (player, mut velocity) in &mut query {
        let mut test = Mat3::from_axis_angle(Vec3::Y, input.yaw);
        test.z_axis *= -1.0;

        let speed = if input.sprinting {
            player.run_speed
        } else {
            player.walk_speed
        };
        velocity.linvel = test * input.movement * speed;
    }
}

pub fn player_bob(
    mut camera_query: Query<(&mut PlayerCamera, &mut Transform), Without<Player>>,
    player_query: Query<&Velocity, (With<Player>, Without<PlayerCamera>)>,
) {
    if let Ok(player_velocity) = player_query.get_single() {
        let move_distance_squared = player_velocity.linvel.xz().length_squared();

        for (mut camera, mut transform) in &mut camera_query {
            let adjusted_move_speed = 1.0 + move_distance_squared.ln() / 3.0;

            // let off = Vec3::new(
            //     -(camera.bobbing_state * camera.speed).sin() * camera.amount.y,
            //     (camera.bobbing_state * camera.speed / 2.0).cos() * camera.amount.x,
            //     0.0,
            // );

            let rot = -(camera.bobbing_state * camera.speed / 2.0).cos() * camera.tilt;

            camera.bobbing_state += (adjusted_move_speed).max(0.0);
            camera.bobbing_state %= (PI / 2.0) / (camera.speed / 2.0);

            transform.rotate_z(rot);
            // transform.translation += off;
        }
    }
}

pub fn get_input_axis(pkey: KeyCode, skey: KeyCode, keys: &Res<Input<KeyCode>>) -> f32 {
    get_input_value(pkey, keys) - get_input_value(skey, keys)
}

pub fn get_input_value(key: KeyCode, keys: &Res<Input<KeyCode>>) -> f32 {
    if keys.pressed(key) {
        1.0
    } else {
        0.0
    }
}
