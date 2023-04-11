use bevy::prelude::*;

use super::{ENEMY_HP, ENEMY_SPAWN_TIME, RAISE_DIFFICULTY_TIME};

#[derive(Resource)]
pub struct RaiseDifficultyTimer(pub Timer);

impl Default for RaiseDifficultyTimer {
    fn default() -> Self {
        RaiseDifficultyTimer(Timer::from_seconds(
            RAISE_DIFFICULTY_TIME,
            TimerMode::Repeating,
        ))
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer(Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct EnemyHp(pub f32);

impl Default for EnemyHp {
    fn default() -> Self {
        EnemyHp(ENEMY_HP)
    }
}
