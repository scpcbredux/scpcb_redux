mod components;
mod styles;
mod systems;
mod resources;

use systems::layout::*;

use bevy::prelude::*;

use crate::AppState;

use self::systems::interactions::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            // OnExit State Systems
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            // Systems
            .add_systems(
                (interact_with_new_game_button, interact_with_quit_button)
                    .in_set(OnUpdate(AppState::MainMenu))
            );
    }
}
