mod components;
mod styles;
mod systems;

use systems::layout::*;

use bevy::prelude::*;

use crate::game::SimulationState;

use self::systems::interactions::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State System
            .add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
            // OnExit State System
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu)
            // Systems
            .add_systems(
                Update,
                (interact_with_resume_button, interact_with_quit_button)
                    .run_if(in_state(SimulationState::Paused)),
            );
    }
}
