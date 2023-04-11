use bevy::{
    input::gamepad::{GamepadConnection::*, GamepadConnectionEvent},
    prelude::*,
};

use super::gamepad_rcs::*;

pub fn connections(
    mut cmds: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadConnectionEvent>,
) {
    for ev in gamepad_evr.iter() {
        match &ev.connection {
            Connected(_info) => {
                // if no gamepad is setup yet, use this one
                if my_gamepad.is_none() {
                    cmds.insert_resource(MyGamepad::default());
                }
                // println!("Gamepad connected. ID: {}, name: {}", gp.id, _info.name);
            }
            Disconnected => {
                cmds.remove_resource::<MyGamepad>();
                // println!("Gamepad disconnected: ID: {}", gp.id);
            }
        }
    }
}
