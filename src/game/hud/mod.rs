use bevy::prelude::*;

pub mod hud_cmps;
mod hud_sys;

use hud_sys::*;

use crate::AppState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Game),
            (
                spawn_health_bar,
                spawn_stamina_bar,
                spawn_time_display,
                reset_game_time,
                spawn_kill_count,
            ),
        )
        .add_systems(
            Update,
            (
                update_stamina_bar,
                update_health_bar,
                update_game_time_display,
                update_kill_count,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
