use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub crouch_speed: f32,
    /// Camera offset default
    pub co_default: Vec3,
    /// Camera offset crouched
    pub co_crouched: Vec3,
    pub crouch_state: f32,
    pub mouse_sensitivity: f32,
}

impl Default for Player {
    fn default() -> Self {
        // ROOM_SCALE is 0.00390625
        // In scpcb the player moves 0.018
        // In modern game engines units are by 100.0
        // (0.00390625 + 0.018) * 100.0
        // Making the walk_speed 2.19
        // In addition, run_speed = speed * sprint
        // sprint in scpcb is 2.5 making the run_speed 4.89
        let walk_speed = 2.2;

        Self {
            speed: walk_speed,
            walk_speed,
            run_speed: 5.0,
            crouch_speed: 1.0,
            co_default: Vec3::Y * 0.5,
            co_crouched: Vec3::Y * 0.2,
            crouch_state: 0.0,
            mouse_sensitivity: 0.003,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct PlayerBlinkTimer(pub Timer);

impl Default for PlayerBlinkTimer {
    fn default() -> Self {
        // Self(Timer::from_seconds(100.0, TimerMode::Repeating))
        Self(Timer::from_seconds(10.0, TimerMode::Repeating))
    }
}

#[derive(Component, Default)]
pub struct PlayerStamina {
    pub amount: f32,
    pub max_amount: f32,
    pub breath_sounds: Vec<Handle<AudioSource>>,
}

impl PlayerStamina {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            amount: 100.0,
            max_amount: 100.0,
            breath_sounds: vec![
                asset_server.load("audio/sfx/player/breath/breath01.wav"),
                asset_server.load("audio/sfx/player/breath/breath02.wav"),
                asset_server.load("audio/sfx/player/breath/breath03.wav"),
                asset_server.load("audio/sfx/player/breath/breath04.wav"),
                asset_server.load("audio/sfx/player/breath/breath05.wav"),
            ],
        }
    }
}

#[derive(Component, Default)]
pub struct PlayerFootsteps {
    pub walk_footsteps: Vec<Handle<AudioSource>>,
    pub run_footsteps: Vec<Handle<AudioSource>>,
    pub timer: Timer,
}

impl PlayerFootsteps {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            walk_footsteps: vec![
                asset_server.load("audio/sfx/player/footsteps/step01.wav"),
                asset_server.load("audio/sfx/player/footsteps/step02.wav"),
                asset_server.load("audio/sfx/player/footsteps/step03.wav"),
                asset_server.load("audio/sfx/player/footsteps/step04.wav"),
                asset_server.load("audio/sfx/player/footsteps/step05.wav"),
                asset_server.load("audio/sfx/player/footsteps/step06.wav"),
                asset_server.load("audio/sfx/player/footsteps/step07.wav"),
                asset_server.load("audio/sfx/player/footsteps/step08.wav"),
            ],
            run_footsteps: vec![
                asset_server.load("audio/sfx/player/footsteps/run01.wav"),
                asset_server.load("audio/sfx/player/footsteps/run02.wav"),
                asset_server.load("audio/sfx/player/footsteps/run03.wav"),
                asset_server.load("audio/sfx/player/footsteps/run04.wav"),
                asset_server.load("audio/sfx/player/footsteps/run05.wav"),
                asset_server.load("audio/sfx/player/footsteps/run06.wav"),
                asset_server.load("audio/sfx/player/footsteps/run07.wav"),
                asset_server.load("audio/sfx/player/footsteps/run08.wav"),
            ],
            timer: Timer::new(Duration::from_secs_f32(0.3), TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct PlayerCamera {
    pub speed: f32,
    pub max_bob: Vec3,
    pub tilt: f32,
    pub timer: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            speed: 7.0,
            max_bob: Vec3::splat(0.07),
            tilt: 0.5f32.to_radians(),
            timer: 0.0,
        }
    }
}
