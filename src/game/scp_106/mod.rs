pub mod components;
mod systems;

// use components::*;
use bevy::prelude::*;
use systems::*;

pub struct Scp106Plugin;

impl Plugin for Scp106Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(scp106_update);
    }
}
