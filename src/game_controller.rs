use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::*;

pub struct GameController {
    pub started: bool,
    pub score: u32,
    pub player_stats: PlayerStatistics,
    pub settings: GameSettings,
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

        let settings: GameSettings =
            pkv.get::<GameSettings>(GAME_SETTINGS_KEY)
                .unwrap_or(GameSettings {
                    ..Default::default()
                });

        GameController {
            started: false,
            score: 0,
            player_stats,
            settings,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerStatistics {
    pub high_score: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GameSettings {
    pub vol_level: f64,
    pub show_fps: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            vol_level: 0.5,
            show_fps: false,
        }
    }
}
