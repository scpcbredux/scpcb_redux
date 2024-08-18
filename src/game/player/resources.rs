use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    MouseMotion,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Blink,
    Sprint,
    Inventory,
    Crouch,
    Save,
    Console,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::MouseMotion => InputControlKind::DualAxis,
            _ => InputControlKind::Button,
        }
    }
}

// TODO: Remove this
#[derive(Resource, Default)]
pub struct PlayerInput {
    pub movement: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub blinking: bool,
    pub sprinting: bool,
    pub crouched: bool,
}
