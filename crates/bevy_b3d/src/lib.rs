mod loader;
pub use loader::*;

use bevy::{prelude::*, reflect::TypeUuid};

/// Adss support for b3d file loading to the app.
#[derive(Default)]
pub struct B3DPlugin;

impl Plugin for B3DPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<B3DLoader>()
            .add_asset::<B3D>()
            .add_asset::<B3DNode>()
            .add_asset::<B3DMesh>();
    }
}

/// Representation of a loaded b3d file.
#[derive(Debug, TypeUuid)]
#[uuid = "bea01a2e-984c-4e7d-ace8-64a580e882cc"]
pub struct B3D {
    pub scene: Handle<Scene>,
    pub meshes: Vec<Handle<B3DMesh>>,
    pub materials: Vec<Handle<StandardMaterial>>,
    pub nodes: Vec<Handle<B3DNode>>,
}

/// A b3d node with all of its child nodes, its [`B3DMesh`] and [`Transform`]
#[derive(Debug, TypeUuid)]
#[uuid = "c0375473-fdfa-4f47-99d7-60ab2f5c6b88"]
pub struct B3DNode {
    pub children: Vec<B3DNode>,
    pub mesh: Option<Handle<B3DMesh>>,
    pub transform: Transform,
}

/// A b3d mesh, which may contists of a [`Mesh`] and an optional [`StandardMaterial`].
#[derive(Debug, TypeUuid)]
#[uuid = "26c8d9d7-2197-4ce9-a250-1b9a8b775689"]
pub struct B3DMesh {
    pub mesh: Handle<Mesh>,
    pub material: Option<Handle<StandardMaterial>>,
}
