use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

use crate::game::game_cmps::{Damage, Game, Hp, Speed};

use super::{ENEMY_ATTACK_RATE, ENEMY_SIZE, ENEMY_SPEED};

#[derive(Bundle)]
pub struct EnemyBundle {
    pub attack_rate: AttackRate,
    pub collider: Collider,
    pub damage: Damage,
    pub enemy: Enemy,
    pub game: Game,
    pub hp: Hp,
    pub name: Name,
    pub rigid_body: RigidBody,
    pub speed: Speed,
}

impl EnemyBundle {
    pub fn new(hp: f32) -> Self {
        let size_half = ENEMY_SIZE / 2.0;

        Self {
            attack_rate: AttackRate::default(),
            collider: Collider::cylinder(size_half, size_half),
            damage: Damage::new(10.0),
            enemy: Enemy,
            game: Game,
            hp: Hp::new(hp),
            name: Name::new("Enemy"),
            rigid_body: RigidBody::Dynamic,
            speed: Speed(ENEMY_SPEED),
        }
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct AttackRate(pub Timer);

impl Default for AttackRate {
    fn default() -> Self {
        AttackRate(Timer::from_seconds(ENEMY_ATTACK_RATE, TimerMode::Repeating))
    }
}
