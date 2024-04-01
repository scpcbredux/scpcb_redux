mod components;
mod systems;

use self::systems::*;
use crate::{game::SimulationState, AppState};
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_hud)
            .add_systems(OnExit(AppState::Game), despawn_hud)
            .add_systems(
                Update,
                update_progress_bar
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
