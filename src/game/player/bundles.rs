use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{components::*, resources::*};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub footsteps: PlayerFootsteps,
    pub input_bundle: InputManagerBundle<PlayerAction>,
    pub velocity: Velocity,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Default::default(),
            footsteps: Default::default(),
            input_bundle: InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (KeyCode::W, PlayerAction::MoveUp),
                    (KeyCode::S, PlayerAction::MoveDown),
                    (KeyCode::A, PlayerAction::MoveLeft),
                    (KeyCode::D, PlayerAction::MoveRight),
                    (KeyCode::Space, PlayerAction::Blink),
                    (KeyCode::ShiftLeft, PlayerAction::Sprint),
                    (KeyCode::Tab, PlayerAction::Inventory),
                    (KeyCode::ControlLeft, PlayerAction::Crouch),
                    (KeyCode::F3, PlayerAction::Console),
                ]),
            },
            velocity: Default::default(),
        }
    }
}
