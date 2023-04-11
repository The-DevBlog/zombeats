use bevy::prelude::*;

use super::POWERUP_DISPLAY_DURATION;

#[derive(Component)]
pub struct StaminaPowerUp;

#[derive(Component)]
pub struct HpPowerUp;

#[derive(Component)]
pub struct DamagePowerUp;

#[derive(Component)]
pub struct DamagePowerUpDurationDisplay;

#[derive(Component)]
pub struct PowerUpDisplay {
    pub duration: Timer,
}

impl Default for PowerUpDisplay {
    fn default() -> Self {
        PowerUpDisplay {
            duration: Timer::from_seconds(POWERUP_DISPLAY_DURATION, TimerMode::Once),
        }
    }
}
