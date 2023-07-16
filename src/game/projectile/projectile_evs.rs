use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct HitEv {
    pub ent: Entity,
    pub dmg: f32,
}
