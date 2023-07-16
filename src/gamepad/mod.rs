use bevy::prelude::*;

pub mod gamepad_rcs;
mod gamepad_sys;

use gamepad_sys::*;

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, connections);
    }
}
