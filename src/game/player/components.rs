use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub walk_footsteps: Vec<String>,
    pub run_footsteps: Vec<String>,
    pub height: f32,
    pub bob_speed: f32,
    pub bob_amount: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            walk_speed: 3.5,
            run_speed: 5.0,
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
            height: 1.1,
            bob_speed: 3.5,
            bob_amount: Vec2::splat(0.03),
        }
    }
}

#[derive(Component)]
pub struct PlayerCamera;
