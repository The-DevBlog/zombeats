use bevy::prelude::*;

use super::ENEMY_ATTACK_RATE;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct AttackRate(pub Timer);

impl Default for AttackRate {
    fn default() -> Self {
        AttackRate(Timer::from_seconds(ENEMY_ATTACK_RATE, TimerMode::Repeating))
    }
}
