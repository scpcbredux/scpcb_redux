use super::{components::*, player::*, scps::scp_106::components::Scp106, SimulationState};
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rmesh::rmesh::ROOM_SCALE;
use bevy_xpbd_3d::prelude::*;

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
    // mut time: ResMut<Time>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();

        if simulation_state.eq(&SimulationState::Running) {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;

            next_simulation_state.set(SimulationState::Paused);
            // time.pause();
        } else if simulation_state.eq(&SimulationState::Paused) {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;

            next_simulation_state.set(SimulationState::Running);
            // time.unpause();
        }
    }
}

pub fn spawn_map(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
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
            Collider::capsule(0.5, 0.3),
            RigidBody::Kinematic,
            TransformBundle::default(),
            Position(Vec3::Y * 0.55),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
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
                transform: Transform::from_scale(Vec3::ONE * (0.25 / 2.2))
                    .with_translation(-Vec3::Y * 0.55),
                ..Default::default()
            });
        });

    // Player
    commands.spawn((
        Collider::capsule(0.5, 0.3),
        Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.0),
        Position(Vec3::new(1.8, 0.55, 1.5)),
        TransformBundle::default(),
        PlayerBundle::default(),
        PlayerFootsteps::new(&asset_server),
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
        SceneBundle {
            scene: asset_server.load("map/173_opt.rmesh#Scene"),
            ..default()
        },
        Name::new("StartRoom"),
        AsyncCollider(ComputedCollider::TriMesh),
        Map,
    ));
}

pub fn despawn_map(mut commands: Commands, query: Query<Entity, With<Map>>) {
    for map in &query {
        commands.entity(map).despawn_recursive();
    }
}
