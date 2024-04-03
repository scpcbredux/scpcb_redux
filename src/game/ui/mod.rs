mod components;
pub mod pause_menu;
mod systems;

use self::{pause_menu::*, systems::*};
use crate::{game::SimulationState, AppState};
use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State System
            .add_systems(OnEnter(AppState::Game), spawn_hud)
            // OnExit State System
            .add_systems(OnExit(AppState::Game), despawn_hud)
            // OnEnter State System
            .add_systems(OnEnter(SimulationState::Paused), show_pause_menu)
            // OnExit State System
            .add_systems(OnExit(SimulationState::Paused), hide_pause_menu)
            // Update State Systems
            .add_systems(
                Update,
                (update_blink_bar, update_sprint_bar)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (interact_with_resume_button, interact_with_quit_button)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Paused)),
            );
    }
}
