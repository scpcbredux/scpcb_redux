mod components;
mod styles;
mod systems;

// use self::systems::interactions::*;
// use crate::{game::SimulationState, AppState};
// use bevy::prelude::*;
pub use systems::interactions::*;
pub use systems::layout::*;

// pub struct PauseMenuPlugin;

// impl Plugin for PauseMenuPlugin {
//     fn build(&self, app: &mut App) {
//         app
//             // OnEnter State System
//             .add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
//             // OnExit State System
//             .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu)
//             // Update State Systems
//             .add_systems(
//                 Update,
//                 (interact_with_resume_button, interact_with_quit_button)
//                     .run_if(in_state(AppState::Game))
//                     .run_if(in_state(SimulationState::Paused)),
//             );
//     }
// }
