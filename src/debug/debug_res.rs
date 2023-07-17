use bevy::prelude::*;

#[derive(Resource, PartialEq)]
pub struct EnableDebugMode(pub bool);

impl EnableDebugMode {
    pub fn new(enabled: bool) -> EnableDebugMode {
        Self(enabled)
    }
}

#[derive(Resource, PartialEq)]
pub struct DebugProps {
    pub lock_cursor: bool,
}

impl Default for DebugProps {
    fn default() -> Self {
        Self { lock_cursor: true }
    }
}