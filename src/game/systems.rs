use super::{components::*, player::*, scps::prelude::*, SimulationState};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    window::CursorGrabMode,
};
use bevy_rmesh::rmesh::ROOM_SCALE;
use bevy_xpbd_3d::prelude::*;

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    mut windows: Query<&mut Window>,
    mut v_time: ResMut<Time<Virtual>>,
    mut p_time: ResMut<Time<Physics>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();

        if simulation_state.eq(&SimulationState::Running) {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;

            next_simulation_state.set(SimulationState::Paused);
            v_time.pause();
            p_time.pause();
        } else if simulation_state.eq(&SimulationState::Paused) {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;

            next_simulation_state.set(SimulationState::Running);
            v_time.unpause();
            p_time.unpause();
        }
    }
}

pub fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;

    const COUNT: usize = 6;
    let position_range = -2.0..2.0;
    let radius_range = 0.0..0.4;
    let pos_len = position_range.end - position_range.start;
    let radius_len = radius_range.end - radius_range.start;
    let mesh = meshes.add(Sphere::new(1.0).mesh().uv(120, 64));

    for i in 0..COUNT {
        let percent = i as f32 / COUNT as f32;
        let radius = radius_range.start + percent * radius_len;

        // sphere light
        commands
            .spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.5, 0.5, 1.0),
                        emissive: Color::rgb(0.2, 0.2, 1.0),
                        unlit: true,
                        ..default()
                    }),
                    transform: Transform::from_xyz(
                        position_range.start + percent * pos_len,
                        0.3,
                        0.0,
                    )
                    .with_scale(Vec3::splat(radius)),
                    ..default()
                },
                BloomSettings::NATURAL,
            ))
            .with_children(|children| {
                children.spawn(PointLightBundle {
                    point_light: PointLight {
                        radius,
                        color: Color::rgb(0.2, 0.2, 1.0),
                        ..default()
                    },
                    ..default()
                });
            });
    }

    // SCP-106
    commands
        .spawn((
            Name::new("Scp106"),
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
                mesh: asset_server.load("npcs/scp106/106_2.b3d#Mesh0"),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load("npcs/scp106/106_diffuse.jpg")),
                    normal_map_texture: Some(asset_server.load("npcs/scp106/106_normals.png")),
                    ..Default::default()
                }),
                transform: Transform::from_scale(Vec3::ONE * (0.25 / 2.2))
                    .with_translation(-Vec3::Y * 0.55),
                ..Default::default()
            });
        });

    // SCP-173
    // commands
    //     .spawn((
    //         Collider::capsule(0.5, 0.3),
    //         RigidBody::Kinematic,
    //         TransformBundle::default(),
    //         Position(Vec3::Y * 0.55),
    //         Visibility::default(),
    //         InheritedVisibility::default(),
    //         ViewVisibility::default(),
    //         Scp173::default(),
    //         Map,
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn(PbrBundle {
    //             mesh: asset_server.load("npcs/scp173/173_2.b3d#Mesh0"),
    //             material: materials.add(StandardMaterial {
    //                 base_color_texture: Some(asset_server.load("npcs/scp173/173texture.jpg")),
    //                 normal_map_texture: Some(asset_server.load("npcs/scp173/173_Norm.jpg")),
    //                 ..Default::default()
    //             }),
    //             transform: Transform::from_scale(Vec3::ONE * (0.35 / 6.7))
    //                 .with_translation(-Vec3::Y * 0.55),
    //             ..Default::default()
    //         });
    //     });

    // Player
    commands.spawn((
        Name::new("MainPlayer"),
        Collider::capsule(0.5, 0.3),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        GravityScale(1.0),
        Position::from_xyz(1.8, 1.1, 1.5),
        TransformBundle::default(),
        PlayerBundle::default(),
        PlayerFootsteps::new(&asset_server),
        Map,
    ));

    // Player Camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 70.0_f32.to_radians(),
                near: 3.0 * ROOM_SCALE,
                far: 200.0 * ROOM_SCALE,
                ..Default::default()
            }),
            ..Default::default()
        },
        BloomSettings::NATURAL,
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
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        RigidBody::Static,
        Map,
    ));
}

pub fn despawn_map(mut commands: Commands, query: Query<Entity, With<Map>>) {
    for map in &query {
        commands.entity(map).despawn_recursive();
    }
}
