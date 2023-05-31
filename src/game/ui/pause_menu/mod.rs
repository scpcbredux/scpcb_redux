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
            .add_system(spawn_pause_menu.in_schedule(OnEnter(SimulationState::Paused)))
            // OnExit State System
            .add_system(despawn_pause_menu.in_schedule(OnExit(SimulationState::Paused)))
            // Systems
            .add_systems(
                (interact_with_resume_button, interact_with_quit_button)
                    .in_set(OnUpdate(SimulationState::Paused)),
            );
    }
}
