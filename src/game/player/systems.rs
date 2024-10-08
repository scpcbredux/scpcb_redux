use crate::game::player::{components::*, resources::*, ANGLE_EPSILON};
use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use rand::seq::SliceRandom;
use std::{f32::consts::*, time::Duration};

pub fn player_input(
    query: Query<(&ActionState<PlayerAction>, &Player, &PlayerStamina)>,
    windows: Query<&mut Window>,
    mut input: ResMut<PlayerInput>,
) {
    for (action_state, player, player_stamina) in &query {
        if let Ok(window) = windows.get_single() {
            if window.focused {
                let vector = action_state.axis_pair(&PlayerAction::MouseMotion);
                let mut delta = Vec2::ZERO;
                delta.x += vector.x;
                delta.y += vector.y;
                delta *= player.mouse_sensitivity;

                input.pitch = (input.pitch - delta.y)
                    .clamp(-FRAC_PI_2 + ANGLE_EPSILON, FRAC_PI_2 - ANGLE_EPSILON);
                input.yaw -= delta.x;
                if input.yaw.abs() > PI {
                    input.yaw = input.yaw.rem_euclid(TAU);
                }
            }
        }

        input.movement = Vec3::new(
            get_input_axis(
                &PlayerAction::MoveRight,
                &PlayerAction::MoveLeft,
                action_state,
            ),
            0.0,
            get_input_axis(&PlayerAction::MoveUp, &PlayerAction::MoveDown, action_state),
        )
        .normalize_or_zero();

        input.blinking = action_state.pressed(&PlayerAction::Blink);
        if player_stamina.amount > 0.0 {
            input.sprinting = action_state.pressed(&PlayerAction::Sprint);
        }
        input.crouched = action_state.pressed(&PlayerAction::Crouch);
    }
}

pub fn player_move(
    mut query: Query<(&mut Player, &PlayerStamina, &mut LinearVelocity)>,
    input: Res<PlayerInput>,
) {
    for (mut player, player_stamina, mut linear_velocity) in &mut query {
        let mut move_to_world = Mat3::from_axis_angle(Vec3::Y, input.yaw);
        move_to_world.z_axis *= -1.0;

        player.speed = if input.crouched {
            player.crouch_speed
        } else if input.sprinting && player_stamina.amount > 0.0 {
            player.run_speed
        } else {
            player.walk_speed
        };
        let y_component = linear_velocity.0.y;

        linear_velocity.0 = move_to_world * (input.movement * player.speed);
        linear_velocity.y = y_component;
    }
}

pub fn player_look(
    time: Res<Time>,
    q_player: Query<(&Transform, &LinearVelocity, &Player), Without<PlayerCamera>>,
    mut q_camera: Query<(&mut PlayerCamera, &mut Transform), Without<Player>>,
    input: Res<PlayerInput>,
) {
    let dt = time.delta_seconds();

    for (p_transform, linear_velocity, player) in &q_player {
        for (mut camera, mut c_transform) in &mut q_camera {
            camera.timer += dt * linear_velocity.length() / player.speed;

            let c_height_off = if input.crouched {
                player.co_crouched
            } else {
                player.co_default
            };
            let c_off = Vec3::new(
                (camera.timer * camera.speed / 2.0).cos(),
                -(camera.timer * camera.speed).sin(),
                0.0,
            );

            let rot = -(camera.timer * camera.speed / 2.0).cos() * camera.tilt;

            c_transform.translation =
                p_transform.translation + c_height_off + c_off * camera.max_bob;
            c_transform.rotation = Quat::from_euler(EulerRot::YXZ, input.yaw, input.pitch, 0.0)
                * Quat::from_rotation_z(rot);
        }
    }
}

pub fn player_footsteps(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &LinearVelocity, &mut PlayerFootsteps, &Transform), With<Player>>,
    input: Res<PlayerInput>,
) {
    let dt = time.delta_seconds();

    for (entity, linear_velocity, mut footsteps, _transform) in &mut query {
        let mut rng = rand::thread_rng();

        footsteps
            .timer
            .tick(Duration::from_secs_f32(dt * linear_velocity.length()));

        if footsteps.timer.finished() {
            let rand_step = if input.sprinting {
                footsteps.run_footsteps.choose(&mut rng)
            } else {
                footsteps.walk_footsteps.choose(&mut rng)
            };

            if let Some(source) = rand_step {
                commands.entity(entity).insert(AudioBundle {
                    source: source.clone(),
                    settings: PlaybackSettings::REMOVE,
                });
            }
        }
    }
}

pub fn player_blink(
    time: Res<Time>,
    mut query: Query<&mut PlayerBlinkTimer>,
    input: Res<PlayerInput>,
) {
    if let Ok(mut blink_timer) = query.get_single_mut() {
        blink_timer.tick(time.delta());

        if input.blinking {
            blink_timer.reset();
        }
    }
}

pub fn player_stamina(
    mut commands: Commands,
    mut query: Query<(&mut PlayerStamina, Entity)>,
    input: Res<PlayerInput>,
) {
    for (mut stamina, entity) in &mut query {
        let mut rng = rand::thread_rng();

        if input.sprinting {
            stamina.amount -= 0.2;
            /*if stamina.amount < 0.0 {
                stamina.amount = -10.0;
            }*/

            if stamina.amount < 5.0 {
                commands.entity(entity).insert(AudioBundle {
                    source: stamina.breath_sounds[0].clone(),
                    settings: PlaybackSettings::REMOVE,
                });
            } else if stamina.amount < 50.0 {
                let rand_sound = stamina.breath_sounds.choose(&mut rng);

                if let Some(source) = rand_sound {
                    commands.entity(entity).insert(AudioBundle {
                        source: source.clone(),
                        settings: PlaybackSettings::REMOVE,
                    });
                }
            }
        } else {
            stamina.amount = (stamina.amount + 0.15).min(stamina.max_amount);
        }
    }
}

fn get_input_axis<A: Actionlike>(paction: &A, saction: &A, action_state: &ActionState<A>) -> f32 {
    get_input_value(paction, action_state) - get_input_value(saction, action_state)
}

fn get_input_value<A: Actionlike>(action: &A, action_state: &ActionState<A>) -> f32 {
    if action_state.pressed(action) {
        1.0
    } else {
        0.0
    }
}
