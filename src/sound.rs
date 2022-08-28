use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;

use crate::GameController;

fn start_background_audio(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    game_controller: Res<GameController>,
) {
    audio
        .play(asset_server.load("sounds/bg.wav"))
        .fade_in(AudioTween::new(
            Duration::from_secs(1),
            AudioEasing::OutPowi(2),
        ))
        .with_volume(game_controller.settings.vol_level as f64)
        .looped();
}

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system(start_background_audio);
    }
}
