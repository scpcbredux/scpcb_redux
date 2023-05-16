use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rmesh::RModel;

use crate::scp_106::Scp106;
use crate::player::{Player, PlayerCamera};
use crate::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            game_setup.in_schedule(OnEnter(GameState::Game)),
            scene_colliders,
        ));
    }
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.insert_resource(CollisionScene::new(asset_server.load("map/173_opt.rmesh")));

    // SCP-106
    commands.spawn((
        PbrBundle {
            mesh: asset_server.load("npcs/106.b3d#Mesh"),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("npcs/106_diffuse.jpg")),
                normal_map_texture: Some(asset_server.load("npcs/106_normals.png")),
                ..Default::default()
            }),
            transform: Transform::from_scale(Vec3::ONE * (0.25 / 2.2)),
            ..Default::default()
        },
        Scp106,
    ));

    // Player
    commands.spawn((
        Player {
            camera_height: 1.1,
            ..default()
        },
        Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.0, 0.3),
        TransformBundle::from(Transform::from_translation((1.8, 0., 1.5).into())),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        ActiveEvents::COLLISION_EVENTS,
        Velocity::zero(),
        RigidBody::Dynamic,
        Sleeping::disabled(),
        LockedAxes::ROTATION_LOCKED,
        AdditionalMassProperties::Mass(0.0),
        GravityScale(7.0),
        Ccd { enabled: true },
    ));
    
    // Player Camera
    commands.spawn((
        Camera3dBundle::default(),
        PlayerCamera::default()
    ));

    commands.spawn(SceneBundle {
        scene: asset_server.load("items/eyedrops.b3d"),
        transform: Transform {
            translation: (0., 1., 0.).into(),
            scale: (0.0012, 0.0012, 0.0012).into(),
            rotation: Quat::IDENTITY,
        },
        ..Default::default()
    });
}

#[derive(Resource)]
struct CollisionScene {
    handle: Handle<RModel>,
    is_loaded: bool,
}

impl CollisionScene {
    pub fn new(handle: Handle<RModel>) -> Self {
        Self {
            handle,
            is_loaded: false,
        }
    }
}

fn scene_colliders(
    mut commands: Commands,
    collider_scene: Option<ResMut<CollisionScene>>,
    rmodel_assets: Res<Assets<RModel>>,
    mesh_assets: Res<Assets<Mesh>>,
) {
    if let Some(mut collider_scene) = collider_scene {
        if collider_scene.is_loaded {
            return;
        }
    
        let rmodel = rmodel_assets.get(&collider_scene.handle);
    
        if let Some(rmodel) = rmodel {
            commands.spawn(SceneBundle {
                scene: rmodel.scene.clone(),
                ..default()
            })
            .with_children(|parent| {
                for rmesh_mesh in &rmodel.colliders {
                    let mesh = mesh_assets.get(&rmesh_mesh).unwrap();
                    parent.spawn((
                        Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap(),
                        RigidBody::Fixed,
                    ));
                }
            });
            collider_scene.is_loaded = true;
        }
    }
}
