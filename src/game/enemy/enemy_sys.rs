use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::game::{
    game_cmps::{Damage, Game, Hp, Speed},
    player::player_cmps::Player,
    world::MAP_SIZE,
};

use super::{
    enemy_cmps::{AttackRate, Enemy},
    enemy_res::{EnemyHp, EnemySpawnTimer, RaiseDifficultyTimer},
    ENEMY_HP, ENEMY_SIZE, ENEMY_SPEED,
};

pub fn increase_hp_over_time(
    mut timer: ResMut<RaiseDifficultyTimer>,
    mut enemy_hp: ResMut<EnemyHp>,
    time: Res<Time>,
) {
    if timer.0.just_finished() {
        enemy_hp.0 += 25.0;
    }

    timer.0.tick(time.delta());
}

pub fn spawn_enemies(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    enemy_hp: Res<EnemyHp>,
) {
    spawn_timer.0.tick(time.delta());

    let mut rng = rand::thread_rng();

    let map_bounds = MAP_SIZE / 2.0;
    let x = rng.gen_range(-map_bounds..=map_bounds);
    let z = rng.gen_range(-map_bounds..=map_bounds);

    if spawn_timer.0.finished() {
        let size_half = ENEMY_SIZE / 2.0;
        cmds.spawn((
            PbrBundle {
                material: materials.add(Color::RED.into()),
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: size_half,
                    depth: size_half,
                    ..default()
                })),
                transform: Transform::from_xyz(x, 0.5, z),
                ..default()
            },
            AttackRate::default(),
            Collider::cylinder(size_half, size_half),
            Damage::new(10.0),
            Enemy,
            Game,
            Hp::new(enemy_hp.0),
            Name::new("Enemy"),
            RigidBody::Dynamic,
            Speed(ENEMY_SPEED),
        ));
    }
}

/// enemies track towards player
pub fn enemy_tracking(
    mut enemy_q: Query<(&mut Transform, &Speed), With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    for (mut enemy_trans, enemy_speed) in enemy_q.iter_mut() {
        if let Ok(player_trans) = player.get_single() {
            let direction = (player_trans.translation - enemy_trans.translation).normalize();

            enemy_trans.translation += direction * enemy_speed.0 * time.delta_seconds();
        }
    }
}

pub fn enemy_attack(
    time: Res<Time>,
    assets: Res<AssetServer>,
    audio: Res<Audio>,
    mut enemy_q: Query<(&mut Transform, &mut AttackRate, &Damage), With<Enemy>>,
    mut player: Query<(&Transform, &mut Hp), (With<Player>, Without<Enemy>)>,
) {
    for (enemy_trans, mut attack_rate, enemy_dmg) in enemy_q.iter_mut() {
        if let Ok((player_trans, mut player_hp)) = player.get_single_mut() {
            let distance = Vec3::distance(enemy_trans.translation, player_trans.translation);

            if distance < ENEMY_SIZE {
                if attack_rate.0.percent_left() == 1.0 {
                    player_hp.value -= enemy_dmg.value;
                    let sound = assets.load(r"audio\hurt.ogg");
                    audio.play(sound);
                    attack_rate.0.tick(time.delta());
                }
            }

            if attack_rate.0.percent_left() < 1.0 {
                attack_rate.0.tick(time.delta());
            }

            if attack_rate.0.finished() {
                attack_rate.0.reset();
            }
        }
    }
}

pub fn reset_health(mut enemy_hp: ResMut<EnemyHp>) {
    enemy_hp.0 = ENEMY_HP;
}
