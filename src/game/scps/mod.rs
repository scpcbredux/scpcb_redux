pub mod scp_106;
pub mod scp_173;

use bevy::{app::PluginGroupBuilder, prelude::*};

use scp_106::Scp106Plugin;
use scp_173::Scp173Plugin;

pub mod prelude {
    pub use super::scp_106::components::Scp106;
    pub use super::scp_173::components::Scp173;
}

pub struct ScpPlugins;

impl PluginGroup for ScpPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add(Scp173Plugin);
        group = group.add(Scp106Plugin);
        // TODO: Add more scps
        group
    }
}
