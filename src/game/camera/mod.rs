use bevy::prelude::*;

pub mod camera_cmps;
mod camera_sys;

use camera_sys::*;

use crate::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn).add_systems(
            Update,
            (
                orbit_gamepad,
                orbit_mouse,
                sync_player_camera,
                zoom_gamepad,
                zoom_mouse,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
