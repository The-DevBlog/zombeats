use bevy::prelude::*;
use rand::Rng;

use crate::game::{
    game_cmps::{Damage, Game, Hp},
    player::{
        player_cmps::{Player, Stamina},
        PLAYER_SIZE,
    },
    world::MAP_SIZE,
};

use super::{
    powerups_cmps::{
        DamagePowerUp, DamagePowerUpDurationDisplay, HpPowerUp, PowerUpDisplay, StaminaPowerUp,
    },
    powerups_res::{DamageBoostDuration, PowerUpSpawnTime},
    DMG_BOOST, HP_BOOST,
};

pub fn spawn_powerups(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_timer: ResMut<PowerUpSpawnTime>,
    time: Res<Time>,
) {
    spawn_timer.0.tick(time.delta());
    let mut rng = rand::thread_rng();

    let map_bounds = MAP_SIZE / 2.0;
    let x = rng.gen_range(-map_bounds..=map_bounds);
    let z = rng.gen_range(-map_bounds..=map_bounds);

    let mut powerup = |color: Color| -> PbrBundle {
        PbrBundle {
            material: materials.add(StandardMaterial {
                emissive: color.into(),
                ..default()
            }),
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                height: 0.2,
                radius: 0.1,
                ..default()
            })),
            transform: Transform::from_xyz(x, 0.3, z),
            ..default()
        }
    };

    if spawn_timer.0.finished() {
        let random_powerup = rng.gen_range(1..=3);

        match random_powerup {
            1 => {
                cmds.spawn((
                    powerup(Color::GREEN),
                    StaminaPowerUp,
                    Game,
                    Name::new("Stamina PowerUp"),
                ));
            }
            2 => {
                cmds.spawn((
                    powerup(Color::RED),
                    HpPowerUp,
                    Game,
                    Name::new("Health PowerUp"),
                ));
            }
            3 => {
                cmds.spawn((
                    powerup(Color::YELLOW),
                    DamagePowerUp,
                    Game,
                    Name::new("Damage PowerUp"),
                ));
            }
            _ => (),
        }
    }
}

pub fn collect_stamina_powerup(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut player_q: Query<(&mut Stamina, &Transform), With<Player>>,
    powerup_q: Query<(Entity, &Transform), With<StaminaPowerUp>>,
) {
    for (powerup_ent, powerup_trans) in powerup_q.iter() {
        for (mut stamina, player_trans) in player_q.iter_mut() {
            let distance = powerup_trans.translation.distance(player_trans.translation);

            // collect powerup and despawn
            if distance < PLAYER_SIZE {
                stamina.value = stamina.max;
                cmds.entity(powerup_ent).despawn_recursive();

                // spawn txt
                let txt = powerup_txt(&assets, "Full Stamina!".to_string());
                cmds.spawn((
                    txt,
                    Name::new("Full Stamina Text"),
                    PowerUpDisplay::default(),
                    Game,
                ));
            }
        }
    }
}

pub fn collect_dmg_powerup(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut player_q: Query<(&Transform, &mut Damage), With<Player>>,
    powerup_q: Query<(Entity, &Transform), With<DamagePowerUp>>,
    mut duration_res: ResMut<DamageBoostDuration>,
) {
    for (powerup_ent, powerup_trans) in powerup_q.iter() {
        for (player_trans, mut dmg) in player_q.iter_mut() {
            let distance = powerup_trans.translation.distance(player_trans.translation);

            // collect powerup and despawn
            if distance < PLAYER_SIZE {
                duration_res.0.reset();
                duration_res.0.unpause();
                cmds.entity(powerup_ent).despawn_recursive();

                dmg.value = dmg.max + DMG_BOOST;

                // spawn txt
                let txt = powerup_txt(&assets, "x2 Damage!".to_string());
                cmds.spawn((
                    txt,
                    Name::new("x2 Damage Text"),
                    PowerUpDisplay::default(),
                    Game,
                ));
            }
        }
    }
}

pub fn collect_hp_powerup(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut player_q: Query<(&Transform, &mut Hp), With<Player>>,
    powerup_q: Query<(Entity, &Transform), With<HpPowerUp>>,
) {
    for (powerup_ent, powerup_trans) in powerup_q.iter() {
        for (player_trans, mut hp) in player_q.iter_mut() {
            let distance = powerup_trans.translation.distance(player_trans.translation);

            // collect powerup and despawn
            if distance < PLAYER_SIZE {
                if hp.value + HP_BOOST > hp.max {
                    hp.value = hp.max;
                } else {
                    hp.value += HP_BOOST;
                }

                cmds.entity(powerup_ent).despawn_recursive();

                // spawn txt
                let txt = powerup_txt(&assets, format!("+{} health!", HP_BOOST));
                cmds.spawn((
                    txt,
                    Name::new("HP PowerUp Text"),
                    PowerUpDisplay::default(),
                    Game,
                ));
            }
        }
    }
}

pub fn spawn_dmg_powerup_duration_display(mut cmds: Commands, assets: Res<AssetServer>) {
    let txt = TextBundle {
        text: Text::from_section(
            "",
            TextStyle {
                font: assets.load("fonts/PermanentMarker-Regular.ttf"),
                font_size: 25.0,
                color: Color::WHITE.into(),
            },
        ),
        style: Style {
            display: Display::None,
            position_type: PositionType::Absolute,
            position: UiRect::new(
                Val::Percent(1.2),
                Val::Undefined,
                Val::Percent(13.0),
                Val::Undefined,
            ),
            ..default()
        },
        ..default()
    };

    cmds.spawn((
        txt,
        Name::new("2X Damage Text"),
        DamagePowerUpDurationDisplay,
        Game,
    ));
}

pub fn update_dmg_powerup_duration_display(
    duration: Res<DamageBoostDuration>,
    mut txt_q: Query<(&mut Text, &mut Style), With<DamagePowerUpDurationDisplay>>,
) {
    if let Ok((mut txt, mut style)) = txt_q.get_single_mut() {
        if duration.0.percent() > 0.0 {
            // un-hide txt
            style.display = Display::Flex;

            // update time
            let time = duration.0.elapsed();
            let time_left = duration.0.duration().saturating_sub(time).as_secs();
            txt.sections[0].value = format!("X2 Damage: {}", time_left);
        } else {
            // hide txt
            style.display = Display::None;
        }
    }
}

pub fn tick_dmg_duration_timer(
    mut player_q: Query<&mut Damage, With<Player>>,
    time: Res<Time>,
    mut duration: ResMut<DamageBoostDuration>,
) {
    duration.0.tick(time.delta());

    if duration.0.finished() {
        if let Ok(mut dmg) = player_q.get_single_mut() {
            dmg.value = dmg.max;
        }

        duration.0.reset();
        duration.0.pause();
    }
}

pub fn despawn_powerup_display(
    mut cmds: Commands,
    time: Res<Time>,
    mut display_q: Query<(Entity, &mut PowerUpDisplay), With<PowerUpDisplay>>,
) {
    for (ent, mut display) in display_q.iter_mut() {
        display.duration.tick(time.delta());

        if display.duration.finished() {
            cmds.entity(ent).despawn_recursive();
        }
    }
}

fn powerup_txt(assets: &Res<AssetServer>, txt: String) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            txt,
            TextStyle {
                font: assets.load("fonts/PermanentMarker-Regular.ttf"),
                font_size: 25.0,
                color: Color::WHITE.into(),
            },
        ),
        style: Style {
            align_self: AlignSelf::Center,
            position: UiRect::left(Val::Percent(55.0)),
            ..default()
        },
        ..default()
    }
}
