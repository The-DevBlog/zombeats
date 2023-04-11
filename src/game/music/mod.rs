use bevy::prelude::*;

pub mod music_res;
mod music_sys;

use music_res::*;
use music_sys::*;

use crate::AppState;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MusicController>()
            .add_system(setup_music.in_schedule(OnEnter(AppState::Game)));
    }
}
