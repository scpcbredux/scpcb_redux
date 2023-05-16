pub mod components;
mod systems;

// use components::*;
use systems::*;
use bevy::prelude::*;

pub struct Scp106Plugin;

impl Plugin for Scp106Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(scp106_update);
    }
}