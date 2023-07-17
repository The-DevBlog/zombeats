use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use super::debug_res::DebugProps;

/// unlock cursor when space key is pressed.
pub fn unlock_cursor(
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
    mut debug_props: ResMut<DebugProps>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        debug_props.lock_cursor = !debug_props.lock_cursor;
    }

    if !debug_props.lock_cursor {
        let mut window = window_q.get_single_mut().unwrap();
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

pub fn unlock_cursor_condition() -> impl Fn(Res<DebugProps>) -> bool {
    move |debug_props: Res<DebugProps>| debug_props.lock_cursor
}
