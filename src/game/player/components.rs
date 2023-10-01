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
        Self {
            speed: 2.5,
            walk_speed: 2.5,
            run_speed: 4.5,
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
        Self(Timer::from_seconds(100.0, TimerMode::Repeating))
    }
}

#[derive(Component)]
pub struct PlayerFootsteps {
    pub walk_footsteps: Vec<Handle<AudioSource>>,
    pub run_footsteps: Vec<Handle<AudioSource>>,
    pub timer: Timer,
}

impl PlayerFootsteps {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            walk_footsteps: vec![
                asset_server.load("sounds/sfx/footsteps/step01.wav"),
                asset_server.load("sounds/sfx/footsteps/step02.wav"),
                asset_server.load("sounds/sfx/footsteps/step03.wav"),
                asset_server.load("sounds/sfx/footsteps/step04.wav"),
                asset_server.load("sounds/sfx/footsteps/step05.wav"),
                asset_server.load("sounds/sfx/footsteps/step06.wav"),
                asset_server.load("sounds/sfx/footsteps/step07.wav"),
                asset_server.load("sounds/sfx/footsteps/step08.wav"),
            ],
            run_footsteps: vec![
                asset_server.load("sounds/sfx/footsteps/run01.wav"),
                asset_server.load("sounds/sfx/footsteps/run02.wav"),
                asset_server.load("sounds/sfx/footsteps/run03.wav"),
                asset_server.load("sounds/sfx/footsteps/run04.wav"),
                asset_server.load("sounds/sfx/footsteps/run05.wav"),
                asset_server.load("sounds/sfx/footsteps/run06.wav"),
                asset_server.load("sounds/sfx/footsteps/run07.wav"),
                asset_server.load("sounds/sfx/footsteps/run08.wav"),
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
