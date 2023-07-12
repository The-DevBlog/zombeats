use std::f32::consts::PI;

use super::camera_cmps::CustomCamera;
use crate::{
    game::{game_cmps::Game, player::player_cmps::Player},
    gamepad::gamepad_rcs::MyGamepad,
};
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

pub fn spawn(mut cmds: Commands) {
    let translation = Vec3::new(0.0, 1.0, 2.0);
    let radius = translation.length();
    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CustomCamera {
            radius,
            ..default()
        },
        Name::new("Camera"),
        Game,
    ));
}

pub fn sync_player_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut CustomCamera, &mut Transform), Without<Player>>,
) {
    let Ok(player) = player.get_single() else { return };
    let Ok((mut camera, mut camera_transform)) = camera.get_single_mut() else { return };

    let delta = player.translation - camera.focus;

    if delta != Vec3::ZERO {
        camera.focus = player.translation;
        camera_transform.translation += delta;
    }
}

// heavily referenced https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html
pub fn orbit_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&CustomCamera, &mut Transform), With<CustomCamera>>,
    mut mouse_evr: EventReader<MouseMotion>,
) {
    let mut rotation = Vec2::ZERO;
    for ev in mouse_evr.iter() {
        rotation = ev.delta;
    }

    for (cam, mut trans) in cam_q.iter_mut() {
        if rotation.length_squared() > 0.0 {
            let window = window_q.get_single().unwrap();
            let delta_x = {
                let delta = rotation.x / window.width() * std::f32::consts::PI;
                if cam.upside_down {
                    -delta
                } else {
                    delta
                }
            };

            let delta_y = rotation.y / window.height() * PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            trans.rotation = yaw * trans.rotation; // rotate around global y axis
            trans.rotation = trans.rotation * pitch;
        }

        let rot_matrix = Mat3::from_quat(trans.rotation);
        trans.translation = cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.radius));
    }
}

pub fn orbit_gamepad(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&CustomCamera, &mut Transform), With<CustomCamera>>,
    axis: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    // return gamepad if one is connected
    let gamepad = if let Some(gp) = my_gamepad {
        gp
    } else {
        return;
    };

    // get X & Y axis of right joystick
    let x_axis = GamepadAxis::new(gamepad.gamepad, GamepadAxisType::RightStickX);
    let y_axis = GamepadAxis::new(gamepad.gamepad, GamepadAxisType::RightStickY);

    let mut rotation = Vec2::ZERO;
    if let (Some(x), Some(y)) = (axis.get(x_axis), axis.get(y_axis)) {
        if x.abs() > gamepad.deadzone || y.abs() > gamepad.deadzone {
            rotation = Vec2::new(x, y);
        }
    }

    for (cam, mut trans) in cam_q.iter_mut() {
        if rotation.length_squared() > 0.0 {
            let window = window_q.get_single().unwrap();
            let delta_x = {
                let delta = rotation.x / window.width()
                    * std::f32::consts::PI
                    * 2.0
                    * gamepad.sensitivity.0;
                if cam.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = -rotation.y / window.height() * PI * gamepad.sensitivity.1;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            trans.rotation = yaw * trans.rotation; // rotate around global y axis
            trans.rotation = trans.rotation * pitch;
        }

        let rot_matrix = Mat3::from_quat(trans.rotation);
        trans.translation = cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.radius));
    }
}

pub fn zoom_mouse(
    mut scroll_evr: EventReader<MouseWheel>,
    mut cam_q: Query<&mut CustomCamera, With<CustomCamera>>,
) {
    let mut scroll = 0.0;
    for ev in scroll_evr.iter() {
        scroll += ev.y;
    }

    if let Ok(mut cam) = cam_q.get_single_mut() {
        if scroll.abs() > 0.0 {
            cam.radius -= scroll * cam.radius * 0.1;
        }
    }
}

pub fn zoom_gamepad(
    btns: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut cam_q: Query<&mut CustomCamera, With<CustomCamera>>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        gp.gamepad
    } else {
        return;
    };

    if let Ok(mut cam) = cam_q.get_single_mut() {
        let d_pad_down = GamepadButton::new(gamepad, GamepadButtonType::DPadDown);
        let d_pad_up = GamepadButton::new(gamepad, GamepadButtonType::DPadUp);

        // zoom out
        if btns.pressed(d_pad_down) {
            cam.radius += cam.radius * 0.01;
        // zoom in
        } else if btns.pressed(d_pad_up) {
            cam.radius -= cam.radius * 0.01;
        }
    }
}
