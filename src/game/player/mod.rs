mod components;
mod resources;
mod systems;

pub use components::*;
pub use resources::*;
pub use systems::*;

use bevy::prelude::*;

use super::SimulationState;

pub const ANGLE_EPSILON: f32 = 0.001953125;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementSettings>()
            .init_resource::<PlayerInput>()
            .add_systems(
                (player_input, player_look, player_move)
                    .in_set(OnUpdate(SimulationState::Running))
            );
    }
}
