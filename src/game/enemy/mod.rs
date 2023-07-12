use bevy::prelude::*;

pub mod enemy_cmps;
pub mod enemy_evs;
pub mod enemy_res;
mod enemy_sys;

use crate::AppState;
use enemy_evs::*;
use enemy_res::*;
use enemy_sys::*;

pub const ENEMY_SPAWN_TIME: f32 = 2.0;
pub const ENEMY_SPEED: f32 = 2.6; // slightly faster than player
pub const ENEMY_HP: f32 = 100.0;
pub const ENEMY_SIZE: f32 = 0.5;
pub const ENEMY_ATTACK_RATE: f32 = 2.0;
pub const RAISE_DIFFICULTY_TIME: f32 = 20.0;
pub const HP_GAIN: f32 = 25.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .init_resource::<RaiseDifficultyTimer>()
            .init_resource::<EnemyHp>()
            .add_event::<HitPlayerEv>()
            .add_event::<EnemyDeathEv>()
            .add_system(reset_hp.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    decrease_hp,
                    despawn,
                    // spawn,
                    tracking,
                    attack,
                    increase_hp_over_time,
                    play_hit_noise,
                )
                    .in_set(OnUpdate(AppState::Game)),
            );
    }
}
