use bevy::prelude::*;

use super::music_res::MusicController;

pub fn setup_music(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
    music_res: Res<MusicController>,
) {
    // reset music if it is already playing
    if let Some(sink) = audio_sinks.get(&music_res.0) {
        sink.stop();
    }

    let music = assets.load(r"audio\music\tvs_story.ogg");
    let handle = audio_sinks.get_handle(audio.play_with_settings(music, PlaybackSettings::LOOP));

    cmds.insert_resource(MusicController(handle));
}
