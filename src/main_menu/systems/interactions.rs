use bevy::{app::AppExit, prelude::*};

use crate::{main_menu::components::*, AppState};

pub fn interact_with_new_game_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = button_query.get_single() {
        match *interaction {
            Interaction::Pressed => next_app_state.set(AppState::Game),
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    button_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
) {
    if let Ok(interaction) = button_query.get_single() {
        match *interaction {
            Interaction::Pressed => app_exit_event_writer.send(AppExit),
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
