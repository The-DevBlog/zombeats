use bevy::prelude::*;

pub mod player_cmps;
pub mod player_res;
pub mod player_sys;

use crate::AppState;
use player_res::*;
use player_sys::*;

pub const PLAYER_SPEED: f32 = 2.5;
pub const PLAYER_HP: f32 = 10000.0;
pub const STAMINA: f32 = 100.0;
pub const SPRINT_SPEED: f32 = 2.0;
pub const PLAYER_SIZE: f32 = 0.5;
pub const STAMINA_REGEN_TIME: f32 = 1.5;
pub const STAMINA_REGEN_AMOUNT: f32 = 0.025;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KillCount>()
            .add_systems((spawn, reset_killcount).in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    decrease_hp,
                    keyboard_movement,
                    gamepad_movement,
                    update_stamina,
                    increase_killcount,
                    death,
                )
                    .in_set(OnUpdate(AppState::Game)),
            );
    }
}
