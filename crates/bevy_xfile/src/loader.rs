use bevy::{asset::{AssetLoader, LoadContext, LoadedAsset}, utils::BoxedFuture, render::{mesh::Indices, render_resource::PrimitiveTopology}, prelude::Mesh};
use russimp::scene::{Scene, PostProcess};
use anyhow::Result;

#[derive(Default)]
pub struct XFileLoader;

impl AssetLoader for XFileLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            Ok(load_xfile(bytes, load_context).await?)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["x"]
    }
}

pub async fn load_xfile<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> Result<()> {
    let scene = Scene::from_buffer(bytes,
        vec![PostProcess::CalculateTangentSpace,
        PostProcess::Triangulate,
        PostProcess::JoinIdenticalVertices,
        PostProcess::SortByPrimitiveType], "x").unwrap();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let positions: Vec<_> = scene.meshes[0].vertices
        .iter()
        .map(|v|  [
            v.x,
            v.y,
            v.z
        ])
        .collect();
    let normals: Vec<_> = scene.meshes[0].normals
        .iter()
        .map(|v| [
            v.x,
            v.y,
            v.z
        ])
        .collect();
    let indices: Vec<_> = scene.meshes[0].faces
        .iter()
        .flat_map(|strip| strip.0.iter().map(|index| *index as u32))
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices)));

    load_context.set_default_asset(LoadedAsset::new(mesh));

    Ok(())
}
