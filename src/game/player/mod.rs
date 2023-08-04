use bevy::prelude::*;

pub mod player_cmps;
pub mod player_res;
pub mod player_sys;

use crate::AppState;
use player_res::*;
use player_sys::*;

pub const PLAYER_SPEED: f32 = 2.5;
pub const PLAYER_HP: f32 = 100.0;
pub const STAMINA: f32 = 200000.0;
pub const SPRINT_SPEED: f32 = 5.0;
pub const PLAYER_SIZE: f32 = 0.5;
pub const STAMINA_REGEN_TIME: f32 = 1.5;
pub const STAMINA_REGEN_AMOUNT: f32 = 0.025;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KillCount>()
            .add_systems(OnEnter(AppState::Game), (spawn_player, reset_killcount))
            .add_systems(
                Update,
                (
                    decrease_hp,
                    keyboard_movement,
                    gamepad_movement,
                    update_stamina,
                    increase_killcount,
                    player_death,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}
