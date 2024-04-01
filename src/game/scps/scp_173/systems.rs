use bevy::prelude::*;
use super::components::Scp173;

pub fn scp173_update(mut query: Query<&mut Scp173>) {
    if let Ok(mut scp_173) = query.get_single_mut() {
        if scp_173.idle != 3 {
            
        }
    }
}
