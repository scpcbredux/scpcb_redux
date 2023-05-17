pub use loader::*;

mod loader;

use bevy::{prelude::*, reflect::TypeUuid};

pub struct XFilePlugin;

impl Plugin for XFilePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<XFileLoader>();
    }
}

#[derive(Debug, TypeUuid)]
#[uuid = "14c685d6-35d1-43e9-8940-1ef9d039756b"]
pub struct XFile {
    pub scene: Handle<Scene>,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}
