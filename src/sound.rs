use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("sounds/bg.wav"))
        .with_volume(0.5)
        .looped();
}

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system(start_background_audio);
    }
}
