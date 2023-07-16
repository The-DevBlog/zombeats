use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*};

mod game;
pub mod game_over;
pub mod gamepad;
mod main_menu;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugin;
use game_over::GameOverPlugin;
use gamepad::GamepadPlugin;
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // You can now give it a configurable delay. This is a safe default.
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
            ..default()
        }))
        .add_plugins((
            GamepadPlugin,
            MainMenuPlugin,
            // WorldInspectorPlugin::new(),
            GamePlugin,
            GameOverPlugin,
        ))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
