use std::time::Duration;

use bevy::window::WindowMode;
use bevy::{asset::ChangeWatcher, prelude::*};

pub mod debug;
mod game;
pub mod game_over;
pub mod gamepad;
mod main_menu;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use debug::debug_res::*;
use game::GamePlugin;
use game_over::GameOverPlugin;
use gamepad::GamepadPlugin;
use main_menu::MainMenuPlugin;

fn main() {
    // debug mode
    let args: Vec<String> = std::env::args().collect();
    let is_debug = args.iter().any(|arg| arg == "debug");

    let window_mode = if is_debug {
        WindowMode::Windowed
    } else {
        WindowMode::BorderlessFullscreen
    };

    App::new()
        .insert_resource(EnableDebugMode::new(is_debug))
        .init_resource::<DebugProps>()
        .add_state::<AppState>()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // You can now give it a configurable delay. This is a safe default.
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: window_mode,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(WorldInspectorPlugin::run_if(
            WorldInspectorPlugin::new(),
            resource_equals(EnableDebugMode(true)),
        ))
        .add_plugins((GamepadPlugin, GamePlugin, GameOverPlugin, MainMenuPlugin))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
