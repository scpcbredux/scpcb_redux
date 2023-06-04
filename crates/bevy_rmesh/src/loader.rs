use std::path::Path;

use crate::{Room, RoomMesh};
use anyhow::Result;
use bevy::asset::AssetPath;
use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::{CompressedImageFormats, ImageType};
use bevy::render::{
    mesh::{Indices, Mesh},
    render_resource::PrimitiveTopology,
};
use bevy::utils::BoxedFuture;
use rmesh::read_rmesh;

pub const ROOM_SCALE: f32 = 8. / 2048.;

pub struct RMeshLoader {
    supported_compressed_formats: CompressedImageFormats,
}

impl AssetLoader for RMeshLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(
            async move { load_rmesh(bytes, load_context, self.supported_compressed_formats).await },
        )
    }

    fn extensions(&self) -> &[&str] {
        &["rmesh"]
    }
}

impl FromWorld for RMeshLoader {
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
async fn load_rmesh<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
    supported_compressed_formats: CompressedImageFormats,
) -> Result<()> {
    let header = read_rmesh(bytes)?;

    let mut meshes = vec![];
    let mut entity_meshes = vec![];

    for i in 0..header.meshes.len() {
        let mesh = &header.meshes[i];
        let mut result_mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let positions: Vec<_> = mesh
            .vertices
            .iter()
            .map(|v| {
                [
                    v.position[0] * ROOM_SCALE,
                    v.position[1] * ROOM_SCALE,
                    -v.position[2] * ROOM_SCALE,
                ]
            })
            .collect();

        let tex_coords: Vec<_> = mesh
            .vertices
            .iter()
            .flat_map(|v| {
                [
                    [v.tex_coords[0][0], 1.0 - v.tex_coords[0][1]], // First UV channel
                    [v.tex_coords[1][0], 1.0 - v.tex_coords[1][1]], // Second UV channel
                ]
            })
            .collect();
        let indices = mesh
            .triangles
            .iter()
            .flat_map(|strip| strip.iter().rev().copied())
            .collect();
        result_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        result_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_coords);
        result_mesh.set_indices(Some(Indices::U32(indices)));
        result_mesh.duplicate_vertices();
        result_mesh.compute_flat_normals();

        let mesh_handle =
            load_context.set_labeled_asset(&format!("Mesh{0}", i), LoadedAsset::new(result_mesh));

        let base_color_texture = if let Some(path) = &mesh.textures[1].path {
            let texture = load_texture(
                &String::from(path),
                load_context,
                supported_compressed_formats,
            )
            .await?;
            Some(
                load_context
                    .set_labeled_asset(&format!("Texture{0}", i), LoadedAsset::new(texture)),
            )
        } else {
            None
        };

        let material = load_context.set_labeled_asset(
            &format!("Material{0}", i),
            LoadedAsset::new(StandardMaterial {
                base_color_texture,
                ..Default::default()
            }),
        );

        meshes.push(RoomMesh {
            mesh: mesh_handle,
            material,
        });
    }

    // for entity in &header.entities {
    //     if let Some(rmesh::EntityType::Model(data)) = &entity.entity_type {
    //         let name = &String::from(data.name.clone());
    //         let (mesh, tex_path) = load_xfile(name, load_context).await?;

    //         let mesh_handle = load_context
    //             .set_labeled_asset(&format!("EntityMesh{0}", name), LoadedAsset::new(mesh));

    //         let base_color_texture = {
    //             let texture = load_xtexture(&tex_path, load_context).await?;
    //             Some(load_context.set_labeled_asset(
    //                 &format!("EntityTexture{0}", name),
    //                 LoadedAsset::new(texture),
    //             ))
    //         };

    //         let material_handle = load_context.set_labeled_asset(
    //             &format!("EntityMaterial{0}", name),
    //             LoadedAsset::new(StandardMaterial {
    //                 base_color_texture,
    //                 ..Default::default()
    //             }),
    //         );

    //         entity_meshes.push(RoomMesh {
    //             mesh: mesh_handle,
    //             material: material_handle,
    //         });
    //     }
    // }

    let scene = {
        let mut world = World::default();

        world
            .spawn(SpatialBundle::INHERITED_IDENTITY)
            .with_children(|parent| {
                for i in 0..header.meshes.len() {
                    let mesh_label = format!("Mesh{0}", i);
                    let mesh_asset_path =
                        AssetPath::new_ref(load_context.path(), Some(&mesh_label));
                    let mat_label = format!("Material{0}", i);
                    let mat_asset_path = AssetPath::new_ref(load_context.path(), Some(&mat_label));
                    parent.spawn(PbrBundle {
                        mesh: load_context.get_handle(mesh_asset_path),
                        material: load_context.get_handle(mat_asset_path),
                        ..Default::default()
                    });
                }
                for entity in header.entities {
                    if let Some(entity_type) = entity.entity_type {
                        match entity_type {
                            rmesh::EntityType::Light(data) => {
                                parent.spawn(PointLightBundle {
                                    transform: Transform::from_translation(Vec3::new(
                                        data.position[0] * ROOM_SCALE,
                                        data.position[1] * ROOM_SCALE,
                                        -data.position[2] * ROOM_SCALE,
                                    )),
                                    point_light: PointLight {
                                        range: data.range,
                                        shadows_enabled: true,
                                        intensity: (data.intensity * 0.8).min(1.) * 60.,
                                        color: Color::rgb_u8(
                                            data.color.0[0],
                                            data.color.0[1],
                                            data.color.0[2],
                                        ),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                            }
                            rmesh::EntityType::SpotLight(data) => {
                                parent.spawn(SpotLightBundle {
                                    transform: Transform::from_translation(Vec3::new(
                                        data.position[0] * ROOM_SCALE,
                                        data.position[1] * ROOM_SCALE,
                                        -data.position[2] * ROOM_SCALE,
                                    )),
                                    spot_light: SpotLight {
                                        range: data.range,
                                        shadows_enabled: true,
                                        intensity: (data.intensity * 0.8).min(1.) * 60.,
                                        color: Color::rgb_u8(
                                            data.color.0[0],
                                            data.color.0[1],
                                            data.color.0[2],
                                        ),
                                        inner_angle: data.inner_cone_angle,
                                        outer_angle: data.outer_cone_angle,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                            }
                            rmesh::EntityType::Model(data) => {
                                let name = &String::from(data.name.clone());
                                let mesh_label = format!("EntityMesh{0}", name);
                                let mesh_asset_path =
                                    AssetPath::new_ref(load_context.path(), Some(&mesh_label));
                                let mat_label = format!("EntityMaterial{0}", name);
                                let mat_asset_path =
                                    AssetPath::new_ref(load_context.path(), Some(&mat_label));

                                parent.spawn(PbrBundle {
                                    transform: Transform {
                                        translation: (
                                            data.position[0] * ROOM_SCALE,
                                            data.position[1] * ROOM_SCALE,
                                            -data.position[2] * ROOM_SCALE,
                                        )
                                            .into(),
                                        rotation: Quat::from_euler(
                                            EulerRot::XYZ,
                                            data.rotation[0],
                                            data.rotation[1],
                                            data.rotation[2],
                                        ),
                                        scale: (
                                            data.scale[0] * ROOM_SCALE,
                                            data.scale[1] * ROOM_SCALE,
                                            data.scale[2] * ROOM_SCALE,
                                        )
                                            .into(),
                                    },
                                    mesh: load_context.get_handle(mesh_asset_path),
                                    material: load_context.get_handle(mat_asset_path),
                                    ..Default::default()
                                });
                            }
                            _ => (),
                        }
                    }
                }
            });

        load_context.set_labeled_asset("Scene", LoadedAsset::new(Scene::new(world)))
    };

    load_context.set_default_asset(LoadedAsset::new(Room {
        scene,
        entity_meshes,
        meshes,
    }));

    Ok(())
}

async fn load_texture<'a>(
    path: &str,
    load_context: &LoadContext<'a>,
    supported_compressed_formats: CompressedImageFormats,
) -> Result<Image> {
    let parent = load_context.path().parent().unwrap();
    let image_path = parent.join(path);
    let bytes = load_context.read_asset_bytes(image_path.clone()).await?;

    let extension = Path::new(path).extension().unwrap().to_str().unwrap();
    let image_type = ImageType::Extension(extension);

    Ok(Image::from_buffer(
        &bytes,
        image_type,
        supported_compressed_formats,
        true,
    )?)
}
