use std::path::Path;

use anyhow::Result;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::{
        BuildWorldChildren, FromWorld, Image, Mesh, PbrBundle, SpatialBundle, StandardMaterial,
        World,
    },
    render::{
        mesh::Indices,
        render_resource::PrimitiveTopology,
        renderer::RenderDevice,
        texture::{CompressedImageFormats, ImageType},
    },
    scene::Scene,
    utils::BoxedFuture,
};
use russimp::material::PropertyTypeInfo;
use russimp::scene::{PostProcess, Scene as AssimpScene};

use crate::XFile;

pub struct XFileLoader {
    supported_compressed_formats: CompressedImageFormats,
}

impl AssetLoader for XFileLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(
            async move { load_xfile(bytes, load_context, self.supported_compressed_formats).await },
        )
    }

    fn extensions(&self) -> &[&str] {
        &["x"]
    }
}

impl FromWorld for XFileLoader {
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

/// Loads an entire rmesh file.
async fn load_xfile<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
    supported_compressed_formats: CompressedImageFormats,
) -> Result<()> {
    let assimp_scene = AssimpScene::from_buffer(
        &bytes,
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
        "x",
    )
    .unwrap();

    let assimp_mesh = &assimp_scene.meshes[0];
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let positions: Vec<_> = assimp_mesh
        .vertices
        .iter()
        .map(|v| [v.x, v.y, v.z])
        .collect();
    let normals: Vec<_> = assimp_mesh
        .normals
        .iter()
        .map(|v| [v.x, v.y, v.z])
        .collect();
    let indices: Vec<_> = assimp_mesh
        .faces
        .iter()
        .flat_map(|strip| strip.0.iter().copied())
        .collect();
    let mut tex_coords = vec![];
    for tc in assimp_mesh.texture_coords.iter().flatten() {
        for stc in tc {
            tex_coords.push([stc.x, stc.y]);
        }
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_coords);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices)));

    let mut base_color_texture = None;
    for mat in assimp_scene.materials {
        for prop in mat.properties {
            if let PropertyTypeInfo::String(t) = prop.data {
                let texture = load_texture(&t, &load_context, supported_compressed_formats).await?;
                base_color_texture =
                    Some(load_context.set_labeled_asset("Texture", LoadedAsset::new(texture)));
            }
        }
    }

    let handle_mesh = load_context.set_labeled_asset("Mesh", LoadedAsset::new(mesh));

    let handle_material = load_context.set_labeled_asset(
        "Material",
        LoadedAsset::new(StandardMaterial {
            base_color_texture,
            ..Default::default()
        }),
    );

    let scene = {
        let mut world = World::default();

        world
            .spawn(SpatialBundle::INHERITED_IDENTITY)
            .with_children(|parent| {
                let mesh_asset_path = AssetPath::new_ref(load_context.path(), Some("Mesh"));
                let mat_asset_path = AssetPath::new_ref(load_context.path(), Some("Material"));
                parent.spawn(PbrBundle {
                    mesh: load_context.get_handle(mesh_asset_path),
                    material: load_context.get_handle(mat_asset_path),
                    ..Default::default()
                });
            });

        load_context.set_labeled_asset("Scene", LoadedAsset::new(Scene::new(world)))
    };

    load_context.set_default_asset(LoadedAsset::new(XFile {
        scene,
        mesh: handle_mesh,
        material: handle_material,
    }));

    Ok(())
}

async fn load_texture<'a>(
    uri: &str,
    load_context: &LoadContext<'a>,
    supported_compressed_formats: CompressedImageFormats,
) -> Result<Image> {
    let parent = load_context.path().parent().unwrap();
    let image_path = parent.join(uri);
    let bytes = load_context.read_asset_bytes(image_path.clone()).await?;

    let extension = Path::new(uri).extension().unwrap().to_str().unwrap();
    let image_type = ImageType::Extension(extension);

    Ok(Image::from_buffer(
        &bytes,
        image_type,
        supported_compressed_formats,
        true,
    )?)
}
