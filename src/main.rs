mod game;
mod main_menu;

use bevy::prelude::*;
use bevy_b3d::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rmesh::*;
// use bevy_xfile::*;
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::prelude::*;

use game::{player::PlayerAction, GamePlugin};
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SCP - Containment Breach Redux".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        // SCPCB Redux Plugins
        .add_plugins((MainMenuPlugin, GamePlugin))
        // File Formats
        .add_plugins((B3DPlugin, /*XFilePlugin,*/ RMeshPlugin))
        // Other Plugins
        .add_plugins((
            PhysicsPlugins::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            WorldInspectorPlugin::default(),
        ))
        .run();
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    // TODO: Loading and Preloading
}
