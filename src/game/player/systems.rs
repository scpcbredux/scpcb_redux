use bevy::{input::mouse::MouseMotion, prelude::*};
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

pub fn player_move(mut query: Query<(&mut Player, &mut Velocity)>, input: Res<PlayerInput>) {
    for (mut player, mut velocity) in &mut query {
        let mut test = Mat3::from_axis_angle(Vec3::Y, input.yaw);
        test.z_axis *= -1.0;

        player.speed = if input.sprinting {
            player.run_speed
        } else {
            player.walk_speed
        };
        velocity.linvel = test * input.movement * player.speed;
    }
}

pub fn player_bob(
    time: Res<Time>,
    mut camera_query: Query<(&mut PlayerCamera, &mut Transform), Without<Player>>,
    player_query: Query<(&Velocity, &Player), Without<PlayerCamera>>,
) {
    let dt = time.delta_seconds();

    for (player_velocity, player) in &player_query {
        for (mut camera, mut transform) in &mut camera_query {
            camera.timer += dt * player_velocity.linvel.length() / player.speed;

            let off = Vec3::new(
                (camera.timer * camera.speed / 2.0).cos(),
                -(camera.timer * camera.speed).sin(),
                0.0,
            );

            let rot = -(camera.timer * camera.speed / 2.0).cos() * camera.tilt;

            transform.rotate_z(rot);
            transform.translation += off * camera.max_bob.extend(0.0);
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
