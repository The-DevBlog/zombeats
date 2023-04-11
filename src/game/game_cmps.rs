use bevy::prelude::*;

#[derive(Component)]
pub struct Hp {
    pub max: f32,
    pub value: f32,
}

impl Hp {
    pub fn new(max: f32) -> Self {
        Self { max, value: max }
    }
}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Damage {
    pub max: f32,
    pub value: f32,
}

impl Damage {
    pub fn new(max: f32) -> Self {
        Self { max, value: max }
    }
}

#[derive(Component)]
pub struct Game;
