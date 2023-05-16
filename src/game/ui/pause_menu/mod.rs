mod systems;
mod components;
mod styles;

use systems::layout::*;

use bevy::prelude::*;

use crate::game::SimulationState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State System
            .add_system(spawn_pause_menu.in_schedule(OnEnter(SimulationState::Paused)))
            // OnExit State System
            .add_system(despawn_pause_menu.in_schedule(OnExit(SimulationState::Paused)));
    }
}
