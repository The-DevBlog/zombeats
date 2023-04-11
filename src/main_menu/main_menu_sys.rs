use bevy::prelude::*;

use crate::{gamepad::gamepad_rcs::MyGamepad, AppState};

use super::{
    main_menu_cmps::{MainMenu, MainMenuCamera, PlayBtn},
    PLAY_BTN_COLOR, PLAY_BTN_COLOR_HOVER,
};

pub fn spawn_menu(mut cmds: Commands, assets: Res<AssetServer>) {
    let img_container = (
        ImageBundle {
            image: assets.load("imgs/main_menu_background.png").into(),
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::all(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        },
        MainMenu,
        Name::new("Main Menu Image"),
    );

    let play_btn = (
        ButtonBundle {
            background_color: PLAY_BTN_COLOR.into(),
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position: UiRect::top(Val::Percent(30.0)),
                size: Size::new(Val::Px(150.0), Val::Px(75.0)),
                ..default()
            },
            ..default()
        },
        PlayBtn,
        Name::new("Play Button"),
    );

    let play_txt = (
        TextBundle::from_section(
            "Play - ",
            TextStyle {
                color: Color::WHITE,
                font: assets.load("fonts/PermanentMarker-Regular.ttf"),
                font_size: 40.0,
                ..default()
            },
        ),
        Name::new("Play Text"),
    );

    let play_btn_img = (
        ImageBundle {
            image: assets.load("imgs/a_button.png").into(),
            style: Style {
                size: Size::all(Val::Px(35.0)),
                ..default()
            },
            ..default()
        },
        Name::new("Play Button Image"),
    );

    let title_txt = (
        TextBundle {
            text: Text::from_section(
                "ZOMBEATS",
                TextStyle {
                    color: Color::RED.into(),
                    font: assets.load("fonts/PermanentMarker-Regular.ttf"),
                    font_size: 125.0,
                },
            ),
            style: Style {
                align_self: AlignSelf::Start,
                position_type: PositionType::Absolute,
                position: UiRect::top(Val::Percent(5.0)),
                ..default()
            },
            ..default()
        },
        Name::new("Zombeats Text"),
    );

    cmds.spawn((Camera3dBundle::default(), MainMenuCamera));
    cmds.spawn(img_container).with_children(|parent| {
        parent.spawn(title_txt);
        parent.spawn(play_btn).with_children(|parent| {
            parent.spawn(play_txt);
            parent.spawn(play_btn_img);
        });
    });
}

pub fn despawn_menu(
    mut cmds: Commands,
    main_menu_cam_q: Query<Entity, With<MainMenuCamera>>,
    main_menu_q: Query<Entity, With<MainMenu>>,
) {
    if let Ok(camera) = main_menu_cam_q.get_single() {
        cmds.entity(camera).despawn_recursive();
    }

    if let Ok(menu) = main_menu_q.get_single() {
        cmds.entity(menu).despawn_recursive();
    }
}

pub fn select_play_gamepad(
    btns: Res<Input<GamepadButton>>,
    cur_app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, GamepadButtonType::South)))
        .unwrap_or(false);

    if gamepad_input {
        if cur_app_state.0 != AppState::Game {
            next_app_state.set(AppState::Game);
        }
    }
}

pub fn select_play_mouse(
    mut interact_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayBtn>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_clr) in &mut interact_q {
        match *interaction {
            Interaction::Clicked => next_app_state.set(AppState::Game),
            Interaction::Hovered => *background_clr = PLAY_BTN_COLOR_HOVER.into(),
            Interaction::None => *background_clr = PLAY_BTN_COLOR.into(),
        }
    }
}
