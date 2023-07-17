use bevy::prelude::*;

use self::{debug_res::EnableDebugMode, debug_sys::unlock_cursor};

pub mod debug_res;
pub mod debug_sys;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            unlock_cursor.run_if(resource_equals(EnableDebugMode(true))),
        );
    }
}
