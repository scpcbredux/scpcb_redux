mod loader;
pub use loader::*;

use bevy::prelude::*;

#[derive(Default)]
pub struct XFilePlugin;

impl Plugin for XFilePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<XFileLoader>();
    }
}
