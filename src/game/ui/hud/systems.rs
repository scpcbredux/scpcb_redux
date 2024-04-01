use crate::game::player::PlayerBlinkTimer;

use super::components::{Hud, ProgressMeter};
use bevy::prelude::*;

const PROGRESS_BAR_WIDTH: f32 = 204.5;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _hud_entity = build_hud(&mut commands, &asset_server);
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<Hud>>) {
    if let Ok(hud_entity) = hud_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}

pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
            // ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                // image: asset_server.load("pause_menu.png").into(),
                ..default()
            },
            Hud,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(2.35),
                        bottom: Val::Percent(9.0),
                        width: Val::Px(255.0),
                        height: Val::Px(30.5),
                        align_self: AlignSelf::End,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: asset_server.load("ui/game/hud_icons/blink_icon.png").into(),
                            style: Style {
                                width: Val::Px(30.5),
                                height: Val::Px(30.5),
                                border: UiRect::all(Val::Px(0.5)),
                                ..default()
                            },
                            ..default()
                        },
                        BorderColor(Color::WHITE),
                    ));
                    parent
                        .spawn(NodeBundle {
                            border_color: Color::WHITE.into(),
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(19.5),
                                width: Val::Px(PROGRESS_BAR_WIDTH),
                                height: Val::Px(20.0),
                                padding: UiRect::all(Val::Px(1.5)),
                                border: UiRect::all(Val::Px(0.5)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for i in 0..20 {
                                parent.spawn((
                                    ImageBundle {
                                        image: asset_server
                                            .load("ui/game/hud_icons/blink_meter.jpg")
                                            .into(),
                                        style: Style {
                                            width: Val::Px(10.0),
                                            height: Val::Px(14.2),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ProgressMeter(i),
                                ));
                            }
                        });
                });
        })
        .id();

    hud_entity
}

pub fn update_progress_bar(
    query: Query<&PlayerBlinkTimer, Without<ProgressMeter>>,
    mut q_progress: Query<(&mut Visibility, &ProgressMeter), Without<PlayerBlinkTimer>>,
) {
    if let Ok(blink_timer) = query.get_single() {
        let value = (blink_timer.fraction_remaining() / 10.0 * PROGRESS_BAR_WIDTH).ceil();
        for (mut visibility, meter) in &mut q_progress {
            *visibility = if meter.0 > value as usize {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            };
        }
        // info!("Blink Meter Value: {:#?}", value);
    }
}
