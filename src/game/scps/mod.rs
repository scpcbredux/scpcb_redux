pub mod scp_106;

use bevy::{app::PluginGroupBuilder, prelude::*};

use scp_106::Scp106Plugin;

pub struct ScpPlugins;

impl PluginGroup for ScpPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add(Scp106Plugin);
        // TODO: Add more scps
        group
    }
}
