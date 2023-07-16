use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct HitPlayerEv(pub f32);

#[derive(Event)]
pub struct EnemyDeathEv(pub Entity);
