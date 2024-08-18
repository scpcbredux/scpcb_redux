mod game;
mod main_menu;
// mod preload;

use avian3d::prelude::*;
use bevy::{dev_tools::fps_overlay::*, prelude::*};
use bevy_b3d::*;
use bevy_directx_mesh::DirectXMeshPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rmesh::*;
use leafwing_input_manager::prelude::*;

use game::{player::PlayerAction, GamePlugin};
use main_menu::MainMenuPlugin;
// use preload::PreloadPlugin;
// use preload::video::AviPlugin;

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
        .init_state::<AppState>()
        // File Formats
        .add_plugins((DirectXMeshPlugin, B3DPlugin, RMeshPlugin))
        // SCPCB Redux Plugins
        .add_plugins((/*PreloadPlugin,*/ MainMenuPlugin, GamePlugin))
        // Other Plugins
        .add_plugins((
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        font: default(),
                    },
                },
            },
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            WorldInspectorPlugin::default(),
        ))
        .run();
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    // Preload,
    MainMenu,
    Game,
    // TODO: Loading
}

#[derive(Component)]
pub struct StartupCamera;
