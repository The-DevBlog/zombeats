use bevy::{prelude::*, render::render_resource::Face};
use bevy_rapier3d::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

use crate::game::game_cmps::Game;

use super::{
    world_res::{Colors, LightTimer},
    MAP_SIZE, WALL_HEIGHT,
};

pub fn spawn_ground(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_txtr = assets.load("textures/floor.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(floor_txtr.clone()),
        ..default()
    });

    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: MAP_SIZE,
                ..default()
            })),
            material: material_handle,
            ..default()
        },
        Game,
        Collider::cuboid(MAP_SIZE / 2.0, 0.0, MAP_SIZE / 2.0),
        Name::new("Ground"),
    ));
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

pub fn spawn_light(mut cmds: Commands) {
    cmds.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::GREEN.into(),
                intensity: 5000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Game,
        Name::new("Point Light"),
    ));
}

pub fn change_light_clr(
    mut light_q: Query<&mut PointLight, With<PointLight>>,
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
