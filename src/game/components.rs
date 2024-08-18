use bevy::prelude::*;

#[derive(Component)]
pub struct Map;

#[derive(Resource)]
pub struct Cubemap {
    pub is_loaded: bool,
    pub image_handle: Handle<Image>,
}
