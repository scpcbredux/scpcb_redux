use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub walk_footsteps: Vec<String>,
    pub run_footsteps: Vec<String>,
    pub height: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 3.5,
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
            height: 1.0,
        }
    }
}

#[derive(Component)]
pub struct PlayerCamera {
    pub speed: f32,
    pub max_bob: Vec2,
    pub tilt: f32,
    pub timer: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            speed: 3.5,
            max_bob: Vec2::splat(0.03),
            tilt: 2.0f32.to_radians(),
            timer: 0.0,
        }
    }
}
