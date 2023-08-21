use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::game_cmps::{Damage, Game, Hp, Speed};

use super::{ENEMY_ATTACK_RATE, ENEMY_SIZE, ENEMY_SPEED};

#[derive(Bundle)]
pub struct EnemyBundle {
    pub attack_rate: AttackRate,
    pub collider: Collider,
    pub damage: Damage,
    pub enemy: Enemy,
    pub friction: Friction,
    pub game: Game,
    pub hp: Hp,
    pub locked_axes: LockedAxes,
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
            friction: Friction::coefficient(0.0),
            game: Game,
            hp: Hp::new(hp),
            locked_axes: LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
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
