use std::time::Duration;

use bevy::prelude::*;

use super::STAMINA_REGEN_TIME;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stamina {
    pub max: f32,
    pub value: f32,
    pub regen_time: Timer,
}

impl Stamina {
    pub fn new(max: f32) -> Self {
        Self {
            max,
            value: max,
            regen_time: Timer::new(Duration::from_secs_f32(STAMINA_REGEN_TIME), TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct IsSprinting(pub bool);

#[derive(Component)]
pub struct IsShooting(pub bool);
