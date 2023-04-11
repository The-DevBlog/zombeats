use bevy::prelude::*;

pub mod main_menu_cmps;
mod main_menu_sys;

use main_menu_sys::*;

use crate::AppState;

pub const PLAY_BTN_COLOR: Color = Color::rgba(0.38, 0.0, 0.99, 0.9);
pub const PLAY_BTN_COLOR_HOVER: Color = Color::rgb(0.5, 0.0, 1.0);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_systems(
                (select_play_gamepad, select_play_mouse).in_set(OnUpdate(AppState::MainMenu)),
            );
    }
}
