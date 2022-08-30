use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::*;

pub struct GameController {
    pub game_state: GameState,
    before_pause: GameState,

    pub score: u32,
    pub player_stats: PlayerStatistics,
    pub settings: GameSettings,
}

impl GameController {
    pub fn update_highscore(&mut self, mut pkv: ResMut<PkvStore>) {
        if self.score > self.player_stats.high_score {
            self.player_stats.high_score = self.score;

            // Save the high score
            self.save_player_stats(&mut pkv);
        }
    }

    pub fn reset_game(
        &mut self,
        commands: &mut Commands,
        player: &mut Player,
        player_transform: &mut Transform,
        pipes_query: &mut Query<&mut PipeParent>,
        pipes_handler: &PipesHandler,
    ) {
        self.game_state = GameState::Waiting;

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

    pub fn is_game_waiting(&self) -> bool {
        self.game_state == GameState::Waiting
    }

    pub fn is_game_running(&self) -> bool {
        self.game_state == GameState::Started
    }

    pub fn has_game_started(&self) -> bool {
        self.game_state == GameState::Started
            || (self.game_state == GameState::Paused && self.before_pause == GameState::Started)
    }

    pub fn is_game_paused(&self) -> bool {
        self.game_state == GameState::Paused
    }

    pub fn is_game_finished(&self, player_transform: &Transform) -> bool {
        self.game_state == GameState::Finished
            && player_transform.translation.y < -SCREEN_Y_BOUNDARY
    }

    pub fn pause_game(&mut self) {
        self.before_pause = self.game_state;
        self.game_state = GameState::Paused;
    }

    pub fn resume_game(&mut self) {
        if self.before_pause == GameState::Finished {
            self.game_state = GameState::Restart;
        } else {
            self.game_state = self.before_pause;
        }
    }

    pub fn save_player_stats(&mut self, pkv: &mut PkvStore) {
        pkv.set(PLAYER_STATS_KEY, &self.player_stats)
            .expect("Failed to save high score");
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

        Self {
            game_state: GameState::Waiting,
            before_pause: GameState::Waiting,
            score: 0,
            player_stats,
            settings,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Waiting,
    Started,
    Paused,
    Finished,
    Restart,
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
