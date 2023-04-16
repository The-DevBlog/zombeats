use bevy::prelude::*;

pub mod camera_cmps;
mod camera_sys;

use camera_sys::*;

use crate::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((orbit_gamepad, zoom_gamepad, zoom_mouse).in_set(OnUpdate(AppState::Game)));
    }
}
