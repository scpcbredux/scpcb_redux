use bevy::{prelude::*, window::CursorGrabMode};
use bevy_xpbd_3d::prelude::*;

use crate::{
    game::{ui::pause_menu::components::*, SimulationState},
    AppState,
};

pub fn interact_with_resume_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    mut windows: Query<&mut Window>,
    mut v_time: ResMut<Time<Virtual>>,
    mut p_time: ResMut<Time<Physics>>,
) {
    if let Ok(interaction) = button_query.get_single() {
        let mut window = windows.single_mut();

        match *interaction {
            Interaction::Pressed => {
                window.cursor.visible = false;
                window.cursor.grab_mode = CursorGrabMode::Locked;

                next_simulation_state.set(SimulationState::Running);
                v_time.unpause();
                p_time.unpause();
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn interact_with_quit_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = button_query.get_single() {
        match *interaction {
            Interaction::Pressed => next_app_state.set(AppState::MainMenu),
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
