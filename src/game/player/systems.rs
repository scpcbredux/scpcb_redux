use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use std::f32::consts::*;

use crate::game::player::{components::*, resources::*, ANGLE_EPSILON};

pub fn player_input(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    windows: Query<&mut Window>,
    mut motion_evr: EventReader<MouseMotion>,
    mut input: ResMut<PlayerInput>,
) {
    if let Ok(action_state) = query.get_single() {
        let window = windows.single();
        if window.focused {
            let mut delta = Vec2::ZERO;
            for ev in motion_evr.iter() {
                delta += ev.delta;
            }
            delta *= MOUSE_SENSITIVITY;
    
            input.pitch =
                (input.pitch - delta.y).clamp(-FRAC_PI_2 + ANGLE_EPSILON, FRAC_PI_2 - ANGLE_EPSILON);
            input.yaw -= delta.x;
            if input.yaw.abs() > PI {
                input.yaw = input.yaw.rem_euclid(TAU);
            }
        }
    
        input.movement = Vec3::new(
            get_input_axis(PlayerAction::MoveRight, PlayerAction::MoveLeft, action_state),
            0.0,
            get_input_axis(PlayerAction::MoveUp, PlayerAction::MoveDown, action_state),
        )
        .normalize_or_zero();
    
        input.sprinting = action_state.pressed(PlayerAction::Sprint);
    }
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
        let mut move_to_world = Mat3::from_axis_angle(Vec3::Y, input.yaw);
        move_to_world.z_axis *= -1.0;

        player.speed = if input.sprinting {
            player.run_speed
        } else {
            player.walk_speed
        };
        velocity.linvel = move_to_world * (input.movement * player.speed);
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
            transform.translation += off * camera.max_bob;
        }
    }
}

fn get_input_axis<A: Actionlike>(paction: A, saction: A, action_state: &ActionState<A>) -> f32 {
    get_input_value(paction, action_state) - get_input_value(saction, action_state)
}

fn get_input_value<A: Actionlike>(action: A, action_state: &ActionState<A>) -> f32 {
    if action_state.pressed(action) {
        1.0
    } else {
        0.0
    }
}
