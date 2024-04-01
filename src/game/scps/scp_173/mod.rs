pub mod components;
mod systems;

// use components::*;
use bevy::prelude::*;
use systems::*;

pub struct Scp173Plugin;

impl Plugin for Scp173Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, scp173_update);
    }
}
