use bevy::{
    ecs::{system::EntityCommands, world::EntityRef},
    prelude::*,
    window::CursorGrabMode,
};
use bevy_rapier3d::prelude::*;
use bevy_rmesh::rmesh::ROOM_SCALE;
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use super::{components::Map, player::*, scps::scp_106::components::Scp106, SimulationState};

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

        if simulation_state.eq(&SimulationState::Running) {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;

            next_simulation_state.set(SimulationState::Paused);
        } else if simulation_state.eq(&SimulationState::Paused) {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;

            next_simulation_state.set(SimulationState::Running);
        }
    }
}

pub fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;

    // SCP-106
    commands
        .spawn((
            Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.0, 0.3),
            RigidBody::Fixed,
            TransformBundle::default(),
            Visibility::default(),
            ComputedVisibility::default(),
            Scp106::default(),
            Map,
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: asset_server.load("npcs/106_2.b3d#Mesh0"),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load("npcs/106_diffuse.jpg")),
                    normal_map_texture: Some(asset_server.load("npcs/106_normals.png")),
                    ..Default::default()
                }),
                transform: Transform::from_scale(Vec3::ONE * (0.25 / 2.2)),
                ..Default::default()
            });
        });

    // Player
    commands.spawn((
        Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.0, 0.3),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Dynamic,
        Sleeping::disabled(),
        LockedAxes::ROTATION_LOCKED,
        AdditionalMassProperties::Mass(1.0),
        GravityScale(0.0),
        Ccd { enabled: true },
        TransformBundle::from(Transform::from_translation((1.8, 0., 1.5).into())),
        PlayerBundle::default(),
        Map,
    ));

    // Player Camera
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 70.0_f32.to_radians(),
                near: 3.0 * ROOM_SCALE,
                far: 200.0 * ROOM_SCALE,
                ..Default::default()
            }),
            ..Default::default()
        },
        PlayerCamera::default(),
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

fn scene_collider_hook(_entity: &EntityRef, _cmds: &mut EntityCommands) {
    // if entity.contains::<Handle<Mesh>>() {
    //     cmds.insert(AsyncCollider::default());
    // }
}
