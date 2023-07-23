use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, TypePath, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
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

// TODO: Remove this
#[derive(Resource, Default)]
pub struct PlayerInput {
    pub movement: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub sprinting: bool,
}
