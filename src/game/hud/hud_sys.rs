use bevy::prelude::*;

use super::hud_cmps::*;
use crate::game::{
    game_cmps::{Game, Hp},
    game_res::GameTime,
    player::{player_cmps::*, player_res::KillCount},
};

pub fn spawn_kill_count(mut cmds: Commands, assets: Res<AssetServer>) {
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
            position_type: PositionType::Absolute,
            left: Val::Percent(1.2),
            top: Val::Percent(7.5),
            ..default()
        },
        ..default()
    };

    cmds.spawn((txt, KillCountTxt, Game));
}

pub fn update_kill_count(kills: Res<KillCount>, mut txt_q: Query<&mut Text, With<KillCountTxt>>) {
    if let Ok(mut txt) = txt_q.get_single_mut() {
        txt.sections[0].value = format!("Kills: {}", kills.0);
    }
}

pub fn spawn_health_bar(mut cmds: Commands, assets: Res<AssetServer>) {
    let container = create_container(
        Color::BLACK,
        UiRect::left(Val::Percent(1.0)),
        Val::Percent(10.0),
        Val::Percent(4.0),
    );

    let fill = create_container(
        Color::RED,
        UiRect::new(Val::Percent(0.75), Val::Auto, Val::Percent(1.5), Val::Auto),
        Val::Percent(99.0),
        Val::Percent(99.0),
    );

    let txt = create_txt(&assets);

    cmds.spawn((container, HealthBarContainer, Name::new("Health Bar"), Game))
        .with_children(|parent| {
            parent.spawn(fill);
            parent.spawn((txt, HealthBarTxt, Name::new("Health Bar Text")));
        });
}

pub fn spawn_stamina_bar(mut cmds: Commands, assets: Res<AssetServer>) {
    let container = create_container(
        Color::BLACK,
        UiRect::left(Val::Percent(89.0)),
        Val::Percent(10.0),
        Val::Percent(4.0),
    );

    let fill = create_container(
        Color::DARK_GREEN,
        UiRect::new(Val::Auto, Val::Percent(0.75), Val::Percent(1.5), Val::Auto),
        Val::Percent(99.0),
        Val::Percent(99.0),
    );

    let txt = create_txt(&assets);

    cmds.spawn((
        container,
        StaminaBarContainer,
        Name::new("Stamina Bar"),
        Game,
    ))
    .with_children(|parent| {
        parent.spawn(fill);
        parent.spawn((txt, StaminaBarTxt, Name::new("Stamina Bar Text")));
    });
}

pub fn update_stamina_bar(
    mut stamina_q: Query<&mut Text, With<StaminaBarTxt>>,
    player_q: Query<&Stamina, With<Player>>,
) {
    if let Ok(mut txt) = stamina_q.get_single_mut() {
        if let Ok(stamina) = player_q.get_single() {
            txt.sections[0].value = stamina.value.round().to_string();
        }
    }
}

pub fn update_health_bar(
    mut stamina_q: Query<&mut Text, With<HealthBarTxt>>,
    player_q: Query<&Hp, With<Player>>,
) {
    if let Ok(mut txt) = stamina_q.get_single_mut() {
        if let Ok(hp) = player_q.get_single() {
            txt.sections[0].value = hp.value.round().to_string();
        }
    }
}

pub fn spawn_time_display(mut cmds: Commands, assets: Res<AssetServer>) {
    let container = (
        NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: Val::Percent(46.5),
                ..default()
            },
            ..default()
        },
        Game,
        GameTimeDisplay,
        Name::new("Game Time Display"),
    );

    let txt = (
        TextBundle::from_section(
            "0:00",
            TextStyle {
                color: Color::WHITE,
                font: assets.load("fonts/PermanentMarker-Regular.ttf"),
                font_size: 40.0,
            },
        ),
        GameTimeDisplayTxt,
        Name::new("Game Time Display Text"),
    );

    cmds.spawn(container).with_children(|parent| {
        parent.spawn(txt);
    });
}

pub fn update_game_time_display(
    mut time_display_q: Query<&mut Text, With<GameTimeDisplayTxt>>,
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
) {
    game_time.0.tick(time.delta());

    if let Ok(mut txt) = time_display_q.get_single_mut() {
        let time = game_time.0.elapsed().as_secs_f32();
        txt.sections[0].value = format!("{:.2}", time);
    }
}

/// reset the game time to zero whenever game starts
pub fn reset_game_time(mut game_time: ResMut<GameTime>) {
    game_time.0.reset();
}

fn create_txt(assets: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_section(
        "",
        TextStyle {
            color: Color::WHITE,
            font: assets.load("fonts/PermanentMarker-Regular.ttf"),
            font_size: 25.0,
        },
    )
}

fn create_container(color: Color, position: UiRect, width: Val, height: Val) -> NodeBundle {
    NodeBundle {
        background_color: color.into(),
        style: Style {
            align_self: AlignSelf::FlexStart,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(20.0)),
            left: position.left,
            right: position.right,
            top: position.top,
            bottom: position.bottom,
            position_type: PositionType::Absolute,
            width,
            height,
            ..default()
        },
        ..default()
    }
}
