use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
};
use bevy_third_person_camera::ThirdPersonCamera;

use crate::{
    debug::debug_res::EnableDebugMode,
    game::{
        enemy::{enemy_cmps::Enemy, ENEMY_SIZE},
        game_cmps::{Damage, Game},
        player::player_cmps::{IsShooting, Player},
        world::MAP_SIZE,
    },
    gamepad::gamepad_rcs::MyGamepad,
};

use super::{
    projectile_cmps::Projectile, projectile_evs::HitEv, projectile_res::FireRate, PROJECTILE_SPEED,
};

pub fn shoot_projectile(
    mut cmds: Commands,
    time: Res<Time>,
    btns: Res<Input<GamepadButton>>,
    mouse: Res<Input<MouseButton>>,
    assets: Res<AssetServer>,
    mut fire_rate: ResMut<FireRate>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut player_q: Query<(&mut Transform, &mut IsShooting), With<Player>>,
    cam_q: Query<&Transform, (With<ThirdPersonCamera>, Without<Player>)>,
) {
    // return id of gamepad if one is connected
    let gamepad = if let Some(gp) = my_gamepad {
        Some(gp.gamepad)
    } else {
        None
    };

    if let Ok((mut player_trans, mut is_shooting)) = player_q.get_single_mut() {
        let cam_trans = cam_q.iter().next().unwrap();

        let right_trigger = if let Some(g) = gamepad {
            btns.pressed(GamepadButton::new(g, GamepadButtonType::RightTrigger2))
        } else {
            false
        };

        if right_trigger || mouse.pressed(MouseButton::Left) {
            if fire_rate.0.finished() || fire_rate.0.percent_left() == 1.0 {
                let direction = Vec3::new(cam_trans.back().x, 0.0, cam_trans.back().z);
                let projectile = (
                    PbrBundle {
                        material: materials.add(StandardMaterial {
                            emissive: Color::ORANGE_RED.into(),
                            ..default()
                        }),
                        mesh: meshes.add(Mesh::from(shape::UVSphere {
                            radius: 0.025,
                            ..default()
                        })),
                        transform: Transform::from_translation(player_trans.translation),
                        ..default()
                    },
                    Projectile { direction },
                    Game,
                );

                cmds.spawn(projectile);

                // rotate player in direction he is shooting
                player_trans.look_to(-direction, Vec3::Y);

                cmds.spawn(AudioBundle {
                    source: assets.load(r"audio\shoot.ogg"),
                    settings: PlaybackSettings {
                        volume: Volume::Relative(VolumeLevel::new(0.5)),
                        ..default()
                    },
                    ..default()
                });

                is_shooting.0 = true;
            }

            fire_rate.0.tick(time.delta());
        } else {
            fire_rate.0.reset();
            is_shooting.0 = false;
        }
    }
}

/// Only shoot projectiles if:
/// 1.) NOT in debug mode
/// 2.) In debug mode and ThirdPersonCamera.cursor_lock=true
pub fn shoot_projectile_condition(
    cam_q: Query<&ThirdPersonCamera, With<Camera3d>>,
    debug_res: Res<EnableDebugMode>,
) -> bool {
    if !debug_res.0 {
        return true;
    }

    if let Ok(cam) = cam_q.get_single() {
        return cam.lock_cursor;
    } else {
        return true;
    }
}

pub fn move_projectile(
    time: Res<Time>,
    mut projectile_q: Query<(&mut Transform, &Projectile), With<Projectile>>,
) {
    for (mut trans, projectile) in projectile_q.iter_mut() {
        trans.translation -=
            projectile.direction.normalize() * PROJECTILE_SPEED * time.delta_seconds();
    }
}

/// Detect projectile-enemy collision
/// Fire hit event
pub fn hit_enemy(
    mut cmds: Commands,
    mut hit_evw: EventWriter<HitEv>,
    player_q: Query<&Damage, (With<Player>, Without<Enemy>)>,
    enemy_q: Query<(Entity, &Transform), With<Enemy>>,
    projectile_q: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (enemy_ent, enemy_trans) in enemy_q.iter() {
        for (projectile_ent, projectile_trans) in projectile_q.iter() {
            let distance = enemy_trans
                .translation
                .distance(projectile_trans.translation);

            let dmg = player_q.get_single().unwrap();

            // reduce enemy hp and despawn projectile
            if distance < ENEMY_SIZE {
                // fire hit event
                hit_evw.send(HitEv {
                    dmg: dmg.value,
                    ent: enemy_ent,
                });

                cmds.entity(projectile_ent).despawn_recursive();
            }
        }
    }
}

/// despawn projectiles once they pass beyond the map bounds
pub fn despawn_projectile(
    mut cmds: Commands,
    projectile_q: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (ent, trans) in projectile_q.iter() {
        if trans.translation.x.abs() > MAP_SIZE / 2.0 || trans.translation.z.abs() > MAP_SIZE / 2.0
        {
            cmds.entity(ent).despawn_recursive();
        }
    }
}
