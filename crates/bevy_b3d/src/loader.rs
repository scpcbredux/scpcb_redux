use anyhow::Result;
use bevy::{prelude::*, asset::{AssetLoader, LoadContext, LoadedAsset, AssetPath, AssetIoError}, utils::{BoxedFuture}, render::{render_resource::PrimitiveTopology, mesh::Indices, texture::{CompressedImageFormats, ImageType, TextureError}, renderer::RenderDevice}};
use std::path::Path;
use thiserror::Error;

use crate::B3D;

/// An error that occurs when loading a b3d file.
#[derive(Error, Debug)]
pub enum B3DError {
    #[error(transparent)]
    B3D(#[from] b3d::Error),
    #[error("You may need to add the feature for the file format: {0}")]
    ImageError(#[from] TextureError),
    #[error("failed to load an asset path: {0}")]
    AssetIoError(#[from] AssetIoError),
}

pub struct B3DLoader {
    supported_compressed_formats: CompressedImageFormats,
}

impl AssetLoader for B3DLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            Ok(load_b3d(bytes, load_context, self.supported_compressed_formats).await?)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["b3d"]
    }
}

impl FromWorld for B3DLoader {
    fn from_world(world: &mut World) -> Self {
        let supported_compressed_formats = match world.get_resource::<RenderDevice>() {
            Some(render_device) => CompressedImageFormats::from_features(render_device.features()),
            None => CompressedImageFormats::all(),
        };
        Self {
            supported_compressed_formats,
        }
    }
}

/// Loads an entire b3d file.
async fn load_b3d<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
    supported_compressed_formats: CompressedImageFormats,
) -> Result<(), B3DError> {
    let b3d = b3d::B3D::read(bytes)?;

    let mut materials = vec![];
    for (texture_index, texture) in b3d.textures.into_iter().enumerate() {
        if let Ok(texture) = load_texture(&texture, load_context, supported_compressed_formats).await {
            let texture_handle = load_context.set_labeled_asset(&format!("Texture{}", texture_index), LoadedAsset::new(texture));
        
            let handle = load_context.set_labeled_asset(
                &format!("Material{}", texture_index),
                LoadedAsset::new(StandardMaterial {
                    base_color_texture: Some(texture_handle),
                    ..Default::default()
                }),
            );
            materials.push(handle);
        }
    }

    info!("Mesh key_flags: {:#?}", b3d.node.key_flags);

    let mut meshes = vec![];
    let (mesh, mesh_label) = load_mesh(&b3d.node.mesh, 0)?;
    let mesh_handle = load_context.set_labeled_asset(&mesh_label, LoadedAsset::new(mesh));
    let mat_asset_path = AssetPath::new_ref(load_context.path(), Some("Material0"));
    let bmesh_handle = load_context.set_labeled_asset(
        "B3DMesh0",
        LoadedAsset::new(crate::B3DMesh {
            mesh: mesh_handle,
            material: Some(load_context.get_handle(mat_asset_path)),
        })
    );
    meshes.push(bmesh_handle);

    let nodes = vec![];

    let scene = {
        let mut err = None;
        let mut world = World::default();

        world
            .spawn(SpatialBundle::INHERITED_IDENTITY)
            .with_children(|parent| {
                let result = load_node(
                    &b3d.node,
                    parent,
                    load_context,
                );
                if result.is_err() {
                    err = Some(result)
                }
            });
        if let Some(Err(err)) = err {
            return Err(err);
        }

        load_context.set_labeled_asset(
            "Scene",
            LoadedAsset::new(Scene::new(world)),
        )
    };

    load_context.set_default_asset(LoadedAsset::new(B3D {
        scene,
        materials,
        nodes,
        meshes,
    }));

    Ok(())
}

/// Loads a b3d node.
fn load_node(
    b3d_node: &b3d::Node,
    world_builder: &mut WorldChildBuilder,
    load_context: &mut LoadContext<'_>,
) -> Result<(), B3DError> {
    let transform = Transform {
        translation: b3d_node.position.into(),
        rotation: Quat::from_euler(EulerRot::XYZ, b3d_node.rotation[0], b3d_node.rotation[1], b3d_node.rotation[2]),
        scale: b3d_node.scale.into(),
    };
    let mut b3d_error = None;
    let mut node = world_builder.spawn(SpatialBundle::from(transform));

    node.insert(node_name(b3d_node));

    node.with_children(|parent| {
        let mesh = &b3d_node.mesh;

        let mesh_label = mesh_label(0);

        let mesh_asset_path = AssetPath::new_ref(load_context.path(), Some(&mesh_label));
        let material_asset_path = AssetPath::new_ref(load_context.path(), Some("Material0"));

        let mut mesh_entity = parent.spawn(PbrBundle {
            mesh: load_context.get_handle(mesh_asset_path),
            material: load_context.get_handle(material_asset_path),
            ..Default::default()
        });

        mesh_entity.insert(Name::new(mesh_label));

        // append other nodes
        for child in &b3d_node.children {
            if let Err(err) = load_node(
                child,
                parent,
                load_context
            ) {
                b3d_error = Some(err);
                return;
            }
        }
    });

    if let Some(err) = b3d_error {
        Err(err)
    } else {
        Ok(())
    }
}

fn load_mesh(b3d_mesh: &b3d::Mesh, index: u32) -> Result<(Mesh, String), B3DError> {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    if let Some(vertex_attribute) = b3d_mesh
        .vertices
        .vertices
        .iter()
        .map(|v| v.position)
        .collect::<Vec<_>>()
        .into()
    {
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertex_attribute);
    }

    if let Some(vertex_attribute) = b3d_mesh
        .vertices
        .vertices
        .iter()
        .map(|v| v.normal)
        .collect::<Vec<_>>()
        .into()
    {
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertex_attribute);
    }

    if let Some(vertex_attribute) = b3d_mesh
        .vertices
        .vertices
        .iter()
        .map(|v| v.tex_coords)
        .collect::<Vec<_>>()
        .into()
    {
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertex_attribute);
    }

    if let Some(vertex_attribute) = b3d_mesh
        .triangles
        .iter()
        .flat_map(|tri| tri.indices.iter())
        .flat_map(|face| face.iter().copied())
        .collect::<Vec<_>>()
        .into()
    {
        mesh.set_indices(Some(Indices::U32(vertex_attribute)));
    }

    if let Err(err) = mesh.generate_tangents() {
        warn!(
            "Failed to generate vertex tangents using the mikktspace algorithm: {:?}",
            err
        );
    }

    Ok((mesh, mesh_label(index)))
}

/// Loads a b3d texture as a bevy [`Image`] and returns it together with its label.
async fn load_texture<'a>(
    b3d_texture: &b3d::Texture,
    load_context: &LoadContext<'a>,
    supported_compressed_formats: CompressedImageFormats,
) -> Result<Image, B3DError> {
    let parent = load_context.path().parent().unwrap();
    let image_path = parent.join(&b3d_texture.file);
    let bytes = load_context.read_asset_bytes(image_path.clone()).await?;

    let extension = Path::new(&b3d_texture.file).extension().unwrap().to_str().unwrap();
    let image_type = ImageType::Extension(extension);

    Ok(Image::from_buffer(
        &bytes,
        image_type,
        supported_compressed_formats,
        true,
    )?)
}

fn mesh_label(index: u32) -> String {
    format!("Mesh{}", index)
}

fn node_name(node: &b3d::Node) -> Name {
    let name = format!("B3DNode{}", node.name);
    Name::new(name)
}
