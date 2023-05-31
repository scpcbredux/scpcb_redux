use scp_106::Scp106Plugin;
use player::PlayerPlugin;
use systems::*;

use bevy::prelude::*;

use crate::AppState;

use self::ui::GameUiPlugin;

pub mod scp_106;
pub mod player;

mod systems;
mod ui;
mod components;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SimulationState>()
            // Plugins
            .add_plugin(Scp106Plugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(GameUiPlugin)
            // OnEnter State Systems
            .add_systems((pause_simulation, spawn_map).in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(toggle_simulation.in_set(OnUpdate(AppState::Game))
            )
            // OnExit System
            .add_systems((resume_simulation, despawn_map).in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
