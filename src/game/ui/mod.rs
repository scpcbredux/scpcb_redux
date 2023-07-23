use hud::HudPlugin;
use pause_menu::PauseMenuPlugin;

mod hud;
mod pause_menu;

use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(HudPlugin)
            .add_plugins(PauseMenuPlugin);
    }
}
