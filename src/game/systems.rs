use bevy::{prelude::*, window::CursorGrabMode, ecs::{world::EntityRef, system::EntityCommands}};
use bevy_rapier3d::prelude::*;
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use super::{SimulationState, player::*, components::Map};

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    mut windows: Query<&mut Window>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();

        if simulation_state.0 == SimulationState::Running {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;

            next_simulation_state.set(SimulationState::Paused);
        } else if simulation_state.0 == SimulationState::Paused {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;

            next_simulation_state.set(SimulationState::Running);
        }
    }
}

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>, mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;

    // SCP-106
    commands.spawn((
        PbrBundle {
            mesh: asset_server.load("npcs/106_2.b3d#Mesh0"),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("npcs/106_diffuse.jpg")),
                normal_map_texture: Some(asset_server.load("npcs/106_normals.png")),
                ..Default::default()
            }),
            transform: Transform::from_scale(Vec3::ONE * (0.25 / 2.2)),
            ..Default::default()
        },
        Map,
    ));

    // Player
    commands.spawn((
        Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.1, 0.3),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        ActiveEvents::COLLISION_EVENTS,
        Velocity::zero(),
        RigidBody::Dynamic,
        Sleeping::disabled(),
        LockedAxes::ROTATION_LOCKED,
        AdditionalMassProperties::Mass(1.0),
        GravityScale(0.0),
        Ccd { enabled: true },
        TransformBundle::from(Transform::from_translation((1.8, 0., 1.5).into())),
        Player::default(),
        Map,
    ));

    // 173 Start Room
    commands.spawn((
        HookedSceneBundle {
            scene: SceneBundle {
                scene: asset_server.load("map/173_opt.rmesh#Scene"),
                ..default()
            },
            hook: SceneHook::new(scene_collider_hook),
        },
        Map,
    ));
}

pub fn despawn_map(mut commands: Commands, query: Query<Entity, With<Map>>) {
    for map in &query {
        commands.entity(map).despawn_recursive();
    }
}

fn scene_collider_hook(entity: &EntityRef, cmds: &mut EntityCommands) {
    // if let Some(meshes) = entity.world().get_resource::<Assets<Mesh>>() {
    //     if let Some(mesh) = entity.get::<Handle<Mesh>>() {
    //         cmds.insert((
    //             Collider::from_bevy_mesh(meshes.get(mesh).unwrap(), &ComputedColliderShape::TriMesh).unwrap(),
    //             RigidBody::Fixed,
    //         ));
    //     }
    // }
    if let Some(meshes) = entity.world().get_resource::<Assets<Mesh>>() {
        if let Some(handle_mesh) = entity.get::<Handle<Mesh>>() {
            if let Some(mesh) = meshes.get(handle_mesh) {
                // info!("Mesh Vertices Count: {:#?}", mesh.count_vertices());
                if mesh.count_vertices() == 1485 {
                    cmds.insert((
                        Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap(),
                        RigidBody::Fixed,
                    ));
                }
            }
        }
    }
}
