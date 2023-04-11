use bevy::prelude::*;

use crate::game;

#[derive(Resource)]
pub struct LightTimer(pub Timer);

impl Default for LightTimer {
    fn default() -> Self {
        LightTimer(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct Colors(pub Vec<Color>);

impl Default for Colors {
    fn default() -> Self {
        let mut colors = Vec::new();
        colors.push(Color::BLUE);
        colors.push(Color::RED);
        colors.push(Color::YELLOW);
        colors.push(Color::PURPLE);
        colors.push(Color::PINK);
        colors.push(Color::ORANGE);
        colors.push(Color::TEAL);
        colors.push(Color::AZURE);
        colors.push(Color::VIOLET);

        game::world::world_res::Colors(colors)
    }
}
