use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub height: f32,
    pub crouch_state: f32,
    pub mouse_sensitivity: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 2.5,
            walk_speed: 2.5,
            run_speed: 4.0,
            height: 1.0,
            crouch_state: 0.0,
            mouse_sensitivity: 0.003,
        }
    }
}

#[derive(Component)]
pub struct PlayerFootsteps {
    pub walk_footsteps: Vec<String>,
    pub run_footsteps: Vec<String>,
    pub timer: f32,
    pub delay: f32,
}

impl Default for PlayerFootsteps {
    fn default() -> Self {
        Self {
            walk_footsteps: vec![
                "sounds/sfx/footsteps/step01.ogg".to_string(),
                "sounds/sfx/footsteps/step02.ogg".to_string(),
                "sounds/sfx/footsteps/step03.ogg".to_string(),
                "sounds/sfx/footsteps/step04.ogg".to_string(),
                "sounds/sfx/footsteps/step05.ogg".to_string(),
                "sounds/sfx/footsteps/step06.ogg".to_string(),
                "sounds/sfx/footsteps/step07.ogg".to_string(),
                "sounds/sfx/footsteps/step08.ogg".to_string(),
            ],
            run_footsteps: vec![
                "sounds/sfx/footsteps/run01.ogg".to_string(),
                "sounds/sfx/footsteps/run02.ogg".to_string(),
                "sounds/sfx/footsteps/run03.ogg".to_string(),
                "sounds/sfx/footsteps/run04.ogg".to_string(),
                "sounds/sfx/footsteps/run05.ogg".to_string(),
                "sounds/sfx/footsteps/run06.ogg".to_string(),
                "sounds/sfx/footsteps/run07.ogg".to_string(),
                "sounds/sfx/footsteps/run08.ogg".to_string(),
            ],
            timer: 0.0,
            delay: 0.6,
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
