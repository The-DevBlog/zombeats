use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::player_res::KillCount;
use super::{player_cmps::*, PLAYER_HP, PLAYER_SIZE, PLAYER_SPEED, SPRINT_SPEED, STAMINA};
use crate::game::camera::camera_cmps::CustomCamera;
use crate::game::enemy::enemy_evs::{EnemyDeathEv, HitPlayerEv};
use crate::game::game_cmps::{Damage, Game, Hp, Speed};
use crate::game::world::MAP_SIZE;
use crate::gamepad::gamepad_rcs::MyGamepad;

pub fn spawn_player(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let half_size = PLAYER_SIZE / 2.0;
    let player = cmds
        .spawn((
            PbrBundle {
                material: materials.add(Color::BLUE.into()),
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: half_size,
                    depth: half_size,
                    ..default()
                })),
                transform: Transform::from_xyz(0.0, 0.25, 0.0),
                ..default()
            },
            Collider::cylinder(half_size, half_size),
            Damage::new(25.0),
            Hp::new(PLAYER_HP),
            Game,
            IsSprinting(false),
            Name::new("Player"),
            Player,
            RigidBody::Fixed,
            Speed(PLAYER_SPEED),
            Stamina::new(STAMINA),
        ))
        .id();

    let translation = Vec3::new(0.0, 1.0, 2.0);
    let radius = translation.length();
    let camera = cmds
        .spawn((
            Camera3dBundle {
                transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            CustomCamera {
                radius,
                ..default()
            },
            Name::new("Camera"),
        ))
        .id();

    // make camera have same transform as player
    cmds.entity(player).push_children(&[camera]);
}

/// keeps player within the map bounds
pub fn player_map_bounds(mut player_q: Query<&mut Transform, With<Player>>) {
    if let Ok(mut trans) = player_q.get_single_mut() {
        // +Z bounds
        if trans.translation.z + PLAYER_SIZE / 2.0 > MAP_SIZE / 2.0 {
            trans.translation.z = MAP_SIZE / 2.0 - PLAYER_SIZE / 2.0;
        }

        // -Z bounds
        if trans.translation.z - PLAYER_SIZE / 2.0 < -MAP_SIZE / 2.0 {
            trans.translation.z = -MAP_SIZE / 2.0 + PLAYER_SIZE / 2.0;
        }

        // +X bounds
        if trans.translation.x + PLAYER_SIZE / 2.0 > MAP_SIZE / 2.0 {
            trans.translation.x = MAP_SIZE / 2.0 - PLAYER_SIZE / 2.0;
        }

        // -X bounds
        if trans.translation.x - PLAYER_SIZE / 2.0 < -MAP_SIZE / 2.0 {
            trans.translation.x = -MAP_SIZE / 2.0 + PLAYER_SIZE / 2.0;
        }
    }
}

pub fn move_player_keyboard(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<(&mut Transform, &Speed, &mut IsSprinting, &Stamina), With<Player>>,
    cam_q: Query<&Transform, (With<CustomCamera>, Without<Player>)>,
) {
    for (mut trans, speed, mut sprinting, stamina) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::W) {
            direction += cam.forward().normalize();
        }

        // back
        if keys.pressed(KeyCode::S) {
            direction += cam.back().normalize();
        }

        // left
        if keys.pressed(KeyCode::A) {
            direction += cam.left().normalize();
        }

        // right
        if keys.pressed(KeyCode::D) {
            direction += cam.right().normalize();
        }

        // sprint
        let mut sprint = 1.0;
        if keys.pressed(KeyCode::LShift) && stamina.value > 0.0 {
            sprint = SPRINT_SPEED;
            sprinting.0 = true;
        }

        direction.y = 0.0;
        trans.translation += speed.0 * sprint * direction * time.delta_seconds();
    }
}

pub fn move_player_gamepad(
    time: Res<Time>,
    axis: Res<Axis<GamepadAxis>>,
    btns: Res<Input<GamepadButton>>,
    mut player_q: Query<(&mut Transform, &Speed, &mut IsSprinting, &Stamina), With<Player>>,
    cam_q: Query<&Transform, (With<CustomCamera>, Without<Player>)>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    // return id of gamepad if one is connected
    let gamepad = if let Some(gp) = my_gamepad {
        gp.gamepad
    } else {
        return;
    };

    // get X & Y axis of left joystick
    let x_axis = GamepadAxis {
        axis_type: GamepadAxisType::LeftStickX,
        gamepad,
    };
    let y_axis = GamepadAxis {
        axis_type: GamepadAxisType::LeftStickY,
        gamepad,
    };

    let mut left_joystick = Vec2::ZERO;
    if let (Some(x), Some(y)) = (axis.get(x_axis), axis.get(y_axis)) {
        left_joystick = Vec2::new(x, y);
    }

    for (mut trans, speed, mut sprinting, stamina) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        if left_joystick.length() > 0.5 {
            // Get the direction of the joystick relative to the camera
            let forward = cam.forward().normalize();
            let right = cam.right().normalize();
            let mut joystick_direction = forward * left_joystick.y + right * left_joystick.x;
            joystick_direction.y = 0.0;
            joystick_direction = joystick_direction.normalize();

            // Move the player in the joystick direction
            direction += joystick_direction;
        }

        // sprint
        let mut sprint = 1.0;
        let left_thumb = GamepadButton::new(gamepad, GamepadButtonType::LeftThumb);
        if btns.pressed(left_thumb) && stamina.value > 0.0 {
            sprint = SPRINT_SPEED;
            sprinting.0 = true;
        }

        direction.y = 0.0;
        trans.translation += speed.0 * sprint * direction * time.delta_seconds();
    }
}

pub fn update_stamina(
    mut player_q: Query<(&mut Stamina, &mut IsSprinting), With<Player>>,
    time: Res<Time>,
) {
    for (mut stamina, mut sprinting) in player_q.iter_mut() {
        // if sprinting & stamina is greater than zero, drain stamina & reset regen timer
        if sprinting.0 && stamina.value >= 0.0 {
            stamina.value -= 0.1;
            stamina.regen_time.reset();

        // if regen timer finished & stamina is less than max, regenerate stamina
        } else if stamina.regen_time.just_finished() && stamina.value < stamina.max {
            stamina.value += 0.025;

        // if stamina is less than the max, tick the regen timer
        } else if stamina.value < stamina.max {
            stamina.regen_time.tick(time.delta());
        }

        sprinting.0 = false;
    }
}

pub fn update_health(mut player_q: Query<&mut Hp, With<Player>>) {
    if let Ok(mut hp) = player_q.get_single_mut() {
        if hp.value < 0.0 {
            hp.value = 0.0;
        }
    }
}

pub fn player_is_dead(player_q: Query<&Hp, With<Player>>) -> bool {
    if let Ok(hp) = player_q.get_single() {
        hp.value <= 0.0
    } else {
        false
    }
}

pub fn reset_killcount(mut kills: ResMut<KillCount>) {
    kills.0 = 0;
}

pub fn remove_hp(mut evr: EventReader<HitPlayerEv>, mut player_q: Query<&mut Hp, With<Player>>) {
    for ev in evr.iter() {
        if let Ok(mut hp) = player_q.get_single_mut() {
            hp.value -= ev.0;
        }
    }
}

pub fn increase_killcount(
    mut kills: ResMut<KillCount>,
    mut enemy_death_evr: EventReader<EnemyDeathEv>,
) {
    for _ev in enemy_death_evr.iter() {
        kills.0 += 1;
    }
}
