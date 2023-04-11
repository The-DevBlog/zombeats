use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MusicController(pub Handle<AudioSink>);
