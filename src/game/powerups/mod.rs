use bevy::prelude::*;

pub mod powerups_cmps;
pub mod powerups_res;
mod powerups_sys;

use powerups_res::*;
use powerups_sys::*;

use crate::AppState;

pub const POWERUP_SPAWN_TIME: f32 = 8.0;
pub const DMG_BOOST_DURATION: u64 = 15;
pub const DMG_BOOST: f32 = 25.0;
pub const HP_BOOST: f32 = 20.0;
pub const POWERUP_DISPLAY_DURATION: f32 = 2.0;

pub struct PowerUpsPlugin;

impl Plugin for PowerUpsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PowerUpSpawnTime>()
            .init_resource::<DamageBoostDuration>()
            .add_systems(
                (
                    spawn_powerups,
                    collect_stamina_powerup,
                    collect_dmg_powerup,
                    collect_hp_powerup,
                    tick_dmg_duration_timer,
                    despawn_powerup_display,
                    update_dmg_powerup_duration_display,
                )
                    .in_set(OnUpdate(AppState::Game)),
            )
            .add_system(spawn_dmg_powerup_duration_display.in_schedule(OnEnter(AppState::Game)));
    }
}
