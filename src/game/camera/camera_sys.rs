use crate::game::game_cmps::Game;
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCamera;

pub fn spawn(mut cmds: Commands) {
    let translation = Vec3::new(0.0, 1.0, 2.0);
    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera::default(),
        Name::new("Camera"),
        Game,
    ));
}
