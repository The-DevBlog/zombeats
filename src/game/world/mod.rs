use bevy::prelude::*;

pub mod world_cmps;
pub mod world_res;
mod world_sys;

use world_res::*;
use world_sys::*;

use crate::AppState;

pub const MAP_SIZE: f32 = 25.0;
pub const WALL_HEIGHT: f32 = 2.0;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LightTimer>()
            .init_resource::<Colors>()
            .add_systems(
                OnEnter(AppState::Game),
                (
                    spawn_floor,
                    spawn_disco_light,
                    spawn_walls,
                    spawn_tables,
                    spawn_bar_table,
                ),
            )
            .add_systems(Update, (change_light_clr,).run_if(in_state(AppState::Game)));
    }
}
