use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub stamina: f32,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub gravity: f32,
    pub walk_footsteps: Vec<String>,
    pub run_footsteps: Vec<String>,
    pub shake: f32,
    pub camera_shake: f32,
    pub camera_height: f32,
    pub crouch_state: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            stamina: 1.,
            walk_speed: 3.5,
            run_speed: 5.,
            gravity: -70.5,
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
            shake: 0.0,
            camera_shake: 0.0,
            camera_height: 1.0,
            crouch_state: 0.0,
        }
    }
}

#[derive(Component)]
pub struct PlayerCamera;
