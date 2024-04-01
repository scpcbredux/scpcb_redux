use bevy::prelude::*;

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct ProgressMeter(pub usize);
