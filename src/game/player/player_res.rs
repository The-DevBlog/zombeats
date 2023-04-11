use bevy::prelude::*;

#[derive(Resource)]
pub struct KillCount(pub u32);

impl Default for KillCount {
    fn default() -> Self {
        KillCount(0)
    }
}
