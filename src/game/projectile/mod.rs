use bevy::prelude::*;

pub mod projectile_cmps;
pub mod projectile_evs;
pub mod projectile_res;
mod projectile_sys;

use crate::{debug::debug_sys::unlock_cursor_condition, AppState};
use projectile_evs::*;
use projectile_res::*;
use projectile_sys::*;

pub const PROJECTILE_SPEED: f32 = 25.0;
pub const FIRE_RATE: u64 = 150;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HitEv>()
            .add_event::<HitEv>()
            .init_resource::<FireRate>()
            .add_systems(
                Update,
                (
                    shoot_projectile.run_if(unlock_cursor_condition()),
                    move_projectile,
                    hit_enemy,
                    despawn_projectile,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}
