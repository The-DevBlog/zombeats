use bevy::{prelude::*, render::render_resource::Face};
use bevy_rapier3d::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

use crate::game::game_cmps::Game;

use super::{
    world_cmps::DiscoLight,
    world_res::{Colors, LightTimer},
    MAP_SIZE, WALL_HEIGHT,
};

pub fn spawn_floor(mut cmds: Commands, assets: Res<AssetServer>) {
    let floor = (
        SceneBundle {
            scene: assets.load("models/floor.gltf#Scene0"),
            ..default()
        },
        Collider::cuboid(MAP_SIZE / 2.0, 0.0, MAP_SIZE / 2.0),
        Game,
        Name::new("Floor"),
    );

    cmds.spawn(floor);
}

pub fn spawn_walls(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut wall = |x_pos: f32, z_pos: f32, y_rotation: f32, face: Face, name: &str| {
        (
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Quad {
                    size: Vec2::new(MAP_SIZE, WALL_HEIGHT),
                    ..default()
                })),
                material: materials
                    .add(StandardMaterial {
                        base_color: Color::CRIMSON.into(),
                        cull_mode: Some(face),
                        ..default()
                    })
                    .clone(),
                transform: Transform {
                    translation: Vec3::new(x_pos / 2.0, WALL_HEIGHT / 2.0, z_pos / 2.0),
                    rotation: Quat::from_rotation_y(y_rotation),
                    ..default()
                },
                ..default()
            },
            Collider::cuboid(MAP_SIZE / 2.0, WALL_HEIGHT / 2.0, 0.0),
            Game,
            Name::new(name.to_string()),
        )
    };

    cmds.spawn(wall(0.0, MAP_SIZE, 0.0, Face::Front, "North Wall"));
    cmds.spawn(wall(0.0, -MAP_SIZE, 0.0, Face::Back, "South Wall"));
    cmds.spawn(wall(MAP_SIZE, 0.0, PI / 2.0, Face::Front, "East Wall"));
    cmds.spawn(wall(-MAP_SIZE, 0.0, PI / 2.0, Face::Back, "West Wall"));
}

pub fn spawn_disco_light(mut cmds: Commands) {
    cmds.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 5000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(-5.0, 5.0, -4.5),
            ..default()
        },
        DiscoLight,
        Game,
        Name::new("Point Light"),
    ));
}

pub fn spawn_tables(mut cmds: Commands, assets: Res<AssetServer>) {
    let table =
        |x: f32, z: f32, asset_server: &AssetServer| -> (SceneBundle, Collider, Game, Name) {
            (
                SceneBundle {
                    scene: asset_server.load("models/Table.gltf#Scene0"),
                    transform: Transform {
                        translation: Vec3::new(x, 0.35, z),
                        ..default()
                    },
                    ..default()
                },
                Collider::cylinder(0.32, 0.7),
                Game,
                Name::new("Table"),
            )
        };

    let chair = |x: f32, z: f32, asset_server: &AssetServer| -> (SceneBundle, Collider, Name) {
        (
            SceneBundle {
                scene: asset_server.load("models/Chair.gltf#Scene0"),
                transform: Transform {
                    translation: Vec3::new(x, 0.0, z),
                    ..default()
                },
                ..default()
            },
            Collider::cuboid(0.15, 0.15, 0.15),
            Name::new("Chair"),
        )
    };

    cmds.spawn(table(-0.7, 4.1, &assets))
        .with_children(|parent| {
            parent.spawn(chair(0.4, 0.0, &assets));
            parent.spawn(chair(0.0, 0.4, &assets));
        });
    cmds.spawn(table(-2.7, 8.0, &assets))
        .with_children(|parent| {
            parent.spawn(chair(-0.4, 0.0, &assets));
            parent.spawn(chair(0.0, 0.4, &assets));
        });

    cmds.spawn(table(-4.3, 4.6, &assets))
        .with_children(|parent| {
            parent.spawn(chair(0.4, 0.0, &assets));
        });

    cmds.spawn(table(-7.1, 7.7, &assets))
        .with_children(|parent| {
            parent.spawn(chair(0.4, 0.4, &assets));
        });
}

pub fn spawn_bar_table(mut cmds: Commands, assets: Res<AssetServer>) {
    let light = |pos: Vec3| -> (PointLightBundle, Name) {
        (
            PointLightBundle {
                point_light: PointLight {
                    color: Color::YELLOW_GREEN,
                    intensity: 250.0,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Name::new("Bar Table Light"),
        )
    };

    let bar_table = (
        SceneBundle {
            scene: assets.load("models/BarTable.gltf#Scene0"),
            transform: Transform::from_xyz(8.8, 0.28, 7.0),
            ..default()
        },
        Collider::cuboid(0.5, 0.25, 3.1),
        Game,
        Name::new("Bar Table"),
    );

    cmds.spawn(bar_table).with_children(|parent| {
        parent.spawn(light(Vec3::new(0.0, 3.2, 2.0)));
        parent.spawn(light(Vec3::new(0.0, 3.2, 0.0)));
        parent.spawn(light(Vec3::new(0.0, 3.2, -2.0)));
    });
}

pub fn change_light_clr(
    mut light_q: Query<&mut PointLight, With<DiscoLight>>,
    mut light_timer: ResMut<LightTimer>,
    time: Res<Time>,
    colors: Res<Colors>,
) {
    light_timer.0.tick(time.delta());
    if let Ok(mut light) = light_q.get_single_mut() {
        if light_timer.0.finished() {
            let rng = rand::thread_rng().gen_range(0..colors.0.len());
            light.color = colors.0[rng].into();
        }
    }
}
