use bevy::prelude::*;

mod music_cmps;
mod music_sys;

use music_sys::*;

use crate::{main_res::IsDebugMode, AppState};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), stop_music.before(play_music))
            .add_systems(
                OnEnter(AppState::Game),
                play_music.run_if(resource_equals(IsDebugMode(false))),
            );
    }
}
