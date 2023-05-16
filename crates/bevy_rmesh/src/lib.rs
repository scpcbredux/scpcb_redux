mod loader;
pub use loader::*;

use bevy::{prelude::*, reflect::TypeUuid};

#[derive(Default)]
pub struct RMeshPlugin;

impl Plugin for RMeshPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<RMeshLoader>()
            .add_asset::<Room>()
            .add_asset::<RoomMesh>();
    }
}

#[derive(Debug, TypeUuid)]
#[uuid = "f652caee-aace-49da-9ea3-a314de33b38f"]
pub struct Room {
    pub scene: Handle<Scene>,
    pub meshes: Vec<RoomMesh>,
    pub entity_meshes: Vec<RoomMesh>,
}

#[derive(Debug, TypeUuid)]
#[uuid = "2d373e3d-cb2a-468c-ab25-2724f624dd2f"]
pub struct RoomMesh {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

// #[derive(Debug, TypeUuid)]
// #[uuid = "b33fbaee-ea1b-4de9-86b4-e4ee5bb20284"]
// pub struct XMesh {
//     pub mesh: Handle<Mesh>,
//     pub material: Handle<StandardMaterial>,
// }
