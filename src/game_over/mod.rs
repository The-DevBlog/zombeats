use bevy::prelude::*;

use crate::AppState;

pub mod game_over_cmps;
mod game_over_sys;

use game_over_sys::*;

pub const PLAY_AGAIN_BTN_COLOR: Color = Color::rgba(0.38, 0.0, 0.99, 0.9);
pub const PLAY_AGAIN_BTN_COLOR_HOVER: Color = Color::rgb(0.5, 0.0, 1.0);

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(
                Update,
                (select_play_again_gamepad, select_play_again_mouse)
                    .run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
