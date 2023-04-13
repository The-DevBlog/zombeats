use bevy::prelude::*;

pub mod projectile_cmps;
pub mod projectile_evs;
pub mod projectile_res;
mod projectile_sys;

use crate::AppState;
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
                (
                    shoot_projectile,
                    move_projectile,
                    hit_enemy,
                    despawn_projectile,
                )
                    .in_set(OnUpdate(AppState::Game)),
            );
    }
}
