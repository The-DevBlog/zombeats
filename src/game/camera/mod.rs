use crate::AppState;
use bevy::prelude::*;
use camera_sys::*;

mod camera_sys;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn);
    }
}
