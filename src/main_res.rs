use bevy::prelude::*;

#[derive(Resource, PartialEq)]
pub struct IsDebugMode(pub bool);

impl IsDebugMode {
    pub fn new(enabled: bool) -> IsDebugMode {
        Self(enabled)
    }
}
