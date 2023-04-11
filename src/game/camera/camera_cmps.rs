use bevy::prelude::*;

#[derive(Component)]
pub struct CustomCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for CustomCamera {
    fn default() -> Self {
        CustomCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}
