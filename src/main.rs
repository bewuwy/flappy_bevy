#![windows_subsystem = "windows"]

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

// use bevy_framepace;
use bevy_kira_audio::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

mod clouds;
mod options;
mod pipes;
mod player;
mod ui;

use clouds::*;
use options::*;
use pipes::*;
use player::*;
use ui::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: GAME_NAME.to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(
            BACKGROUND_COLOR[0] / 255.0,
            BACKGROUND_COLOR[1] / 255.0,
            BACKGROUND_COLOR[2] / 255.0,
        )))
        .add_plugins(DefaultPlugins)
        // FPS
        .add_plugin(bevy_framepace::FramepacePlugin)
        .insert_resource(bevy_framepace::FramepaceSettings::default().with_warnings(false))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Game systems
        .add_startup_system(setup)
        .add_plugin(PipesPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CloudsPlugin)
        // UI
        .add_plugin(UIPlugin)
        // Saving plugin
        .insert_resource(PkvStore::new("bewuwy", GAME_NAME))
        // Audio
        .add_plugin(AudioPlugin)
        .add_startup_system(start_background_audio)
        .run();
}

fn setup(mut commands: Commands, pkv: ResMut<PkvStore>) {
    // Load saved data
    let stats: PlayerStatistics = pkv
        .get::<PlayerStatistics>(PLAYER_STATS_KEY)
        .unwrap_or(PlayerStatistics { high_score: 0 });

    // Add a 2D Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Spawn the game controller
    commands.spawn().insert(GameController {
        started: false,
        score: 0,
        player_stats: stats,
    });
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("sounds/bg.wav"))
        .with_volume(0.5)
        .looped();
}

#[derive(Component)]
pub struct GameController {
    started: bool,
    score: u32,
    player_stats: PlayerStatistics,
}

impl GameController {
    pub fn reset_game(
        &mut self,
        commands: &mut Commands,
        player: &mut Player,
        player_transform: &mut Transform,
        pipes_query: &mut Query<&mut PipeParent>,
        pipes_handler: &PipesHandler,
        mut pkv: ResMut<PkvStore>,
    ) {
        self.started = false;

        if self.score > self.player_stats.high_score {
            self.player_stats.high_score = self.score;

            // Save the high score
            pkv.set(PLAYER_STATS_KEY, &self.player_stats)
                .expect("Failed to save high score");
        }

        self.score = 0;

        player.die(player_transform);

        let mut i = 0.0;
        for mut pipe in pipes_query.iter_mut() {
            pipe.reset(
                commands,
                pipes_handler,
                PIPES_START_X + i * PIPES_GAP_BETWEEN,
            );

            i += 1.0;
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PlayerStatistics {
    high_score: u32,
}
