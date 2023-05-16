use bevy::prelude::*;

#[derive(Resource)]
pub struct MovementSettings {
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,

    pub key_blink: KeyCode,
    pub key_sprint: KeyCode,
    pub key_inv: KeyCode,
    pub key_crouch: KeyCode,
    pub key_save: KeyCode,
    pub key_console: KeyCode,

    pub mouse_smooth: f32,
    pub sensitivity: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            key_up: KeyCode::W,
            key_down: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,

            key_blink: KeyCode::Space,
            key_sprint: KeyCode::LShift,
            key_inv: KeyCode::Tab,
            key_crouch: KeyCode::LControl,
            key_save: KeyCode::F5,
            key_console: KeyCode::F3,

            mouse_smooth: 1.0,
            sensitivity: 0.003,
        }
    }
}

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub movement: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub sprinting: bool,
}
