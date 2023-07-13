use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

use crate::game::game_cmps::{Damage, Game, Hp, Speed};

use super::{PLAYER_HP, PLAYER_SIZE, PLAYER_SPEED, STAMINA, STAMINA_REGEN_TIME};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub collider: Collider,
    pub damage: Damage,
    pub game: Game,
    pub hp: Hp,
    pub is_sprinting: IsSprinting,
    pub is_shooting: IsShooting,
    pub name: Name,
    pub player: Player,
    pub rigid_body: RigidBody,
    pub stamina: Stamina,
    pub speed: Speed,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            collider: Collider::cylinder(PLAYER_SIZE, PLAYER_SIZE / 2.0),
            damage: Damage::new(25.0),
            game: Game,
            hp: Hp::new(PLAYER_HP),
            is_sprinting: IsSprinting(false),
            is_shooting: IsShooting(false),
            name: Name::new("Player"),
            player: Player,
            rigid_body: RigidBody::Dynamic,
            stamina: Stamina::new(STAMINA),
            speed: Speed(PLAYER_SPEED),
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stamina {
    pub max: f32,
    pub regen_time: Timer,
    pub value: f32,
}

impl Stamina {
    pub fn new(max: f32) -> Self {
        Self {
            max,
            regen_time: Timer::new(Duration::from_secs_f32(STAMINA_REGEN_TIME), TimerMode::Once),
            value: max,
        }
    }
}

#[derive(Component)]
pub struct IsSprinting(pub bool);

#[derive(Component)]
pub struct IsShooting(pub bool);
