#![windows_subsystem = "windows"]

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

// use bevy_framepace;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

mod clouds;
mod options;
mod pipes;
mod player;
mod sound;
mod ui;
mod window;

use clouds::*;
use options::*;
use pipes::*;
use player::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: GAME_NAME.to_string(),
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(
            BACKGROUND_COLOR[0] / 255.0,
            BACKGROUND_COLOR[1] / 255.0,
            BACKGROUND_COLOR[2] / 255.0,
        )))
        .add_plugins(DefaultPlugins)
        // PKV data storage
        .insert_resource(PkvStore::new("bewuwy", GAME_NAME))
        // FPS
        .add_plugin(bevy_framepace::FramepacePlugin)
        .insert_resource(bevy_framepace::FramepaceSettings::default().with_warnings(false))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Game systems
        .init_resource::<GameController>()
        .add_startup_system(setup)
        .add_plugin(PipesPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CloudsPlugin)
        // UI
        .add_plugin(ui::UIPlugin)
        // Audio
        .add_plugin(sound::SoundPlugin)
        // Window
        .add_plugin(window::WindowIconPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // Add a 2D Camera
    commands.spawn_bundle(Camera2dBundle::default());
}

// #[derive(Default)]
pub struct GameController {
    //TODO: change to resource
    started: bool,
    score: u32,
    player_stats: PlayerStatistics,
    settings: GameSettings,
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

impl FromWorld for GameController {
    fn from_world(world: &mut World) -> Self {
        let pkv = world.get_resource::<PkvStore>().unwrap();

        // Load saved data
        let player_stats: PlayerStatistics = pkv
            .get::<PlayerStatistics>(PLAYER_STATS_KEY)
            .unwrap_or(PlayerStatistics { high_score: 0 });

        let settings: GameSettings = pkv
            .get::<GameSettings>(GAME_SETTINGS_KEY)
            .unwrap_or(GameSettings { vol_level: 0.5 });

        GameController {
            started: false,
            score: 0,
            player_stats,
            settings,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PlayerStatistics {
    high_score: u32,
}

#[derive(Serialize, Deserialize)]
struct GameSettings {
    vol_level: f32,
}
