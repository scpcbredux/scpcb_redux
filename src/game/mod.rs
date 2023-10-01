pub mod player;
pub mod scps;

mod components;
mod systems;
mod ui;

use player::PlayerPlugin;
use scps::ScpPlugins;
use systems::*;

use bevy::prelude::*;

use crate::AppState;

use self::ui::GameUiPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            // Plugins
            .add_plugins((PlayerPlugin, ScpPlugins, GameUiPlugin))
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::Game), (pause_simulation, spawn_map))
            // Systems
            .add_systems(Update, (init_async_scene_colliders, toggle_simulation).run_if(in_state(AppState::Game)))
            // OnExit System
            .add_systems(OnExit(AppState::Game), (resume_simulation, despawn_map));
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
