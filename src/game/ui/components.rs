use bevy::prelude::*;

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct BlinkMeter(pub usize);

#[derive(Component)]
pub struct SprintMeter(pub usize);
