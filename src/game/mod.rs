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

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTime>()
            .add_event::<GameOver>()
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(MusicPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(PowerUpsPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ProjectilePlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(HudPlugin)
            .add_systems((exit_game, hide_cursor, game_over).in_set(OnUpdate(AppState::Game)))
            .add_systems((despawn_game, show_cursor).in_schedule(OnExit(AppState::Game)));
    }
}
