use std::time::Duration;

use bevy::prelude::*;

use super::FIRE_RATE;

#[derive(Resource)]
pub struct FireRate(pub Timer);

impl Default for FireRate {
    fn default() -> Self {
        let mut fire_rate = Timer::new(Duration::from_millis(FIRE_RATE), TimerMode::Repeating);
        fire_rate.unpause();
        FireRate(fire_rate)
    }
}
