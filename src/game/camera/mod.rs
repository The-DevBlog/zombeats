use crate::{debug::debug_sys::unlock_cursor_condition, AppState};
use bevy::prelude::*;
use camera_sys::*;

pub mod camera_cmps;
mod camera_sys;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn).add_systems(
            Update,
            (
                orbit_gamepad,
                orbit_mouse.run_if(unlock_cursor_condition()),
                sync_player_camera,
                zoom_gamepad,
                zoom_mouse,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
