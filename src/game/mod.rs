use bevy::prelude::*;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};

use bevy_rapier3d::render::RapierDebugRenderPlugin;

pub mod camera;
pub mod enemy;
pub mod game_cmps;
pub mod game_evs;
pub mod game_res;
mod game_sys;
pub mod hud;
pub mod music;
pub mod player;
pub mod powerups;
pub mod projectile;
pub mod world;

use camera::CameraPlugin;
use enemy::EnemyPlugin;
use game_evs::*;
use game_res::*;
use game_sys::*;
use hud::HudPlugin;
use music::MusicPlugin;
use player::PlayerPlugin;
use powerups::PowerUpsPlugin;
use projectile::ProjectilePlugin;
use world::WorldPlugin;

use crate::debug::debug_res::EnableDebugMode;
use crate::debug::debug_sys::unlock_cursor_condition;
use crate::AppState;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // determine if debug mode is on
        let is_debug_mode = app
            .world
            .get_resource::<EnableDebugMode>()
            .map(|debug| debug.0)
            .unwrap_or(false);

        app.init_resource::<GameTime>()
            .add_event::<GameOver>()
            .add_plugins((
                CameraPlugin,
                EnemyPlugin,
                HudPlugin,
                MusicPlugin,
                PowerUpsPlugin,
                PlayerPlugin,
                ProjectilePlugin,
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin {
                    enabled: is_debug_mode,
                    ..default()
                },
                WorldPlugin,
            ))
            .add_systems(
                Update,
                (
                    exit_game,
                    hide_cursor.run_if(unlock_cursor_condition()),
                    game_over,
                )
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), (despawn_game, show_cursor));
    }
}
