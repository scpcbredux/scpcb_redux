use bevy::prelude::*;

#[derive(Component)]
pub struct Scp173 {
    pub speed: f32,
    pub idle: u32,
}

impl Default for Scp173 {
    fn default() -> Self {
        Self {
            speed: 0.38,
            idle: 0,
        }
    }
}
