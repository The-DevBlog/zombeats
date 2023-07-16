use bevy::prelude::*;

use super::music_cmps::Music;

pub fn play_music(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((
        AudioBundle {
            source: assets.load(r"audio\music\tvs_story.ogg"),
            ..default()
        },
        Music,
        Name::new("Music"),
    ));
}

pub fn stop_music(mut cmds: Commands, music_q: Query<(&AudioSink, Entity), With<Music>>) {
    for (sink, ent) in music_q.iter() {
        // stop any existing music entities
        sink.stop();

        // despawn existing music entities
        cmds.entity(ent).despawn_recursive();
    }
}
