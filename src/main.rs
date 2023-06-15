mod game;
mod main_menu;

use bevy::prelude::*;
use bevy_b3d::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rmesh::*;
// use bevy_xfile::*;
use bevy_rapier3d::prelude::*;

use bevy_scene_hook::HookPlugin;
use game::GamePlugin;
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
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // Other Plugins
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(B3DPlugin)
        // .add_plugin(XFilePlugin)
        .add_plugin(RMeshPlugin)
        .add_plugin(HookPlugin)
        .run();
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}
