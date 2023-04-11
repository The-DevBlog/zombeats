use bevy::{prelude::*, time::Stopwatch};

#[derive(Resource, Default)]
pub struct GameTime(pub Stopwatch);
