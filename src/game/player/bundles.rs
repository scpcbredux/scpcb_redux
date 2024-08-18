use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{components::*, resources::*};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub blink_timer: PlayerBlinkTimer,
    pub stamina: PlayerStamina,
    pub footsteps: PlayerFootsteps,
    pub input_bundle: InputManagerBundle<PlayerAction>,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Default::default(),
            blink_timer: Default::default(),
            stamina: Default::default(),
            footsteps: Default::default(),
            input_bundle: InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (PlayerAction::MoveUp, KeyCode::KeyW),
                    (PlayerAction::MoveDown, KeyCode::KeyS),
                    (PlayerAction::MoveLeft, KeyCode::KeyA),
                    (PlayerAction::MoveRight, KeyCode::KeyD),
                    (PlayerAction::Blink, KeyCode::Space),
                    (PlayerAction::Sprint, KeyCode::ShiftLeft),
                    (PlayerAction::Inventory, KeyCode::Tab),
                    (PlayerAction::Crouch, KeyCode::ControlLeft),
                    (PlayerAction::Console, KeyCode::F3),
                ])
                .with_dual_axis(PlayerAction::MouseMotion, MouseMove::default()),
            },
        }
    }
}

impl PlayerBundle {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            stamina: PlayerStamina::new(asset_server),
            footsteps: PlayerFootsteps::new(asset_server),
            ..Default::default()
        }
    }
}
