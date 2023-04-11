use bevy::prelude::*;

pub mod hud_cmps;
mod hud_sys;

use hud_sys::*;

use crate::AppState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                spawn_health_bar,
                spawn_stamina_bar,
                spawn_time_display,
                reset_game_time,
                spawn_kill_count,
            )
                .in_schedule(OnEnter(AppState::Game)),
        )
        .add_systems(
            (
                update_stamina_bar,
                update_health_bar,
                update_game_time_display,
                update_kill_count,
            )
                .in_set(OnUpdate(AppState::Game)),
        );
    }
}
