use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{
        camera::camera_cmps::CustomCamera,
        enemy::enemy_cmps::Enemy,
        game_cmps::{Damage, Game, Hp},
        player::{player_cmps::Player, player_res::KillCount},
        world::MAP_SIZE,
    },
    gamepad::gamepad_rcs::MyGamepad,
};

use super::{projectile_cmps::Projectile, projectile_res::FireRate, PROJECTILE_SPEED};

pub fn shoot_projectile(
    mut cmds: Commands,
    time: Res<Time>,
    btns: Res<Input<GamepadButton>>,
    mouse: Res<Input<MouseButton>>,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    mut fire_rate: ResMut<FireRate>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    my_gamepad: Option<Res<MyGamepad>>,
    player_q: Query<&Transform, With<Player>>,
    cam_q: Query<&Transform, (With<CustomCamera>, Without<Player>)>,
) {
    // return id of gamepad if one is connected
    let gamepad = if let Some(gp) = my_gamepad {
        Some(gp.gamepad)
    } else {
        None
    };

    if let Ok(player_trans) = player_q.get_single() {
        // Get the camera's forward direction vector on the xz plane
        let cam_trans = cam_q.iter().next().unwrap();

        let right_trigger = if let Some(g) = gamepad {
            btns.pressed(GamepadButton::new(g, GamepadButtonType::RightTrigger2))
        } else {
            false
        };

        if right_trigger || mouse.pressed(MouseButton::Left) {
            if fire_rate.0.finished() || fire_rate.0.percent_left() == 1.0 {
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
                    Projectile {
                        direction: Vec3::new(cam_trans.translation.x, 0.0, cam_trans.translation.z),
                    },
                    Game,
                );

                cmds.spawn(projectile);

                let sound = assets.load("audio/shoot.ogg");
                audio.play_with_settings(
                    sound,
                    PlaybackSettings {
                        volume: 0.5,
                        ..default()
                    },
                );
            }

            fire_rate.0.tick(time.delta());
        } else {
            fire_rate.0.reset();
        }
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
/// Despawn enemy if hp is <= 0
/// Increase kill count
pub fn hit_enemy(
    mut cmds: Commands,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    player_q: Query<&Damage, (With<Player>, Without<Enemy>)>,
    mut enemy_q: Query<(Entity, &Transform, &mut Hp), With<Enemy>>,
    projectile_q: Query<(Entity, &Transform), With<Projectile>>,
    mut kills: ResMut<KillCount>,
) {
    for (enemy_ent, enemy_trans, mut enemy_hp) in enemy_q.iter_mut() {
        for (projectile_ent, projectile_trans) in projectile_q.iter() {
            let distance = enemy_trans
                .translation
                .distance(projectile_trans.translation);

            let dmg = player_q.get_single().unwrap();

            // reduce enemy hp and despawn projectile
            if distance < 0.25 {
                // despawn enemy
                if enemy_hp.value <= 0.0 {
                    cmds.entity(enemy_ent).despawn_recursive();

                    // increase kill count
                    kills.0 += 1;
                }

                // decrease enemy hp and despawn projectile
                enemy_hp.value -= dmg.value;
                cmds.entity(projectile_ent).despawn_recursive();

                // play enemy hit noise
                let num = rand::thread_rng().gen_range(0..=4);
                let file = format!(r"audio\enemy\hurt_{}.ogg", num);
                let sound = assets.load(file);
                audio.play(sound);
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
