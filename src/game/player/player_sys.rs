use super::player_res::KillCount;
use super::{player_cmps::*, *};
use crate::game::camera::camera_cmps::CustomCamera;
use crate::game::enemy::enemy_evs::{EnemyDeathEv, HitPlayerEv};
use crate::game::game_cmps::{Hp, Speed};
use crate::game::game_evs::GameOver;
use crate::gamepad::gamepad_rcs::MyGamepad;

pub fn spawn_player(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((
        SceneBundle {
            scene: assets.load("models/Player.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.25, 0.0),
                ..default()
            },
            ..default()
        },
        PlayerBundle::default(),
    ));
}

pub fn keyboard_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<
        (
            &mut Transform,
            &Speed,
            &mut IsSprinting,
            &Stamina,
            &IsShooting,
        ),
        With<Player>,
    >,
    cam_q: Query<&Transform, (With<CustomCamera>, Without<Player>)>,
) {
    for (mut player_trans, speed, mut is_sprinting, stamina, is_shooting) in player_q.iter_mut() {
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
        if keys.pressed(KeyCode::ShiftLeft) && stamina.value > 0.0 {
            sprint = SPRINT_SPEED;
            is_sprinting.0 = true;
        }

        direction.y = 0.0;
        player_trans.translation += speed.0 * sprint * direction * time.delta_seconds();

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 && !is_shooting.0 {
            player_trans.look_to(direction, Vec3::Y);
        }
    }
}

pub fn gamepad_movement(
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

    for (mut player_trans, speed, mut sprinting, stamina) in player_q.iter_mut() {
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
        let left_thumb = GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger);
        if btns.pressed(left_thumb) && stamina.value > 0.0 {
            sprint = SPRINT_SPEED;
            sprinting.0 = true;
        }

        direction.y = 0.0;
        player_trans.translation += speed.0 * sprint * direction * time.delta_seconds();

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            player_trans.look_to(direction, Vec3::Y);
        }
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
            stamina.value += STAMINA_REGEN_AMOUNT;

        // if stamina is less than the max, tick the regen timer
        } else if stamina.value < stamina.max {
            stamina.regen_time.tick(time.delta());
        }

        sprinting.0 = false;
    }
}

pub fn player_death(player_q: Query<&Hp, With<Player>>, mut game_over_evw: EventWriter<GameOver>) {
    if let Ok(hp) = player_q.get_single() {
        if hp.value <= 0.0 {
            game_over_evw.send(GameOver);
        }
    }
}

pub fn decrease_hp(mut evr: EventReader<HitPlayerEv>, mut player_q: Query<&mut Hp, With<Player>>) {
    for ev in evr.iter() {
        if let Ok(mut hp) = player_q.get_single_mut() {
            // only decrease hp if hp is > 0
            if hp.value > 0.0 {
                hp.value -= ev.0;

                if hp.value < 0.0 {
                    hp.value = 0.0;
                }
            }
        }
    }
}
pub fn reset_killcount(mut kills: ResMut<KillCount>) {
    kills.0 = 0;
}

pub fn increase_killcount(
    mut kills: ResMut<KillCount>,
    mut enemy_death_evr: EventReader<EnemyDeathEv>,
) {
    for _ev in enemy_death_evr.iter() {
        kills.0 += 1;
    }
}
