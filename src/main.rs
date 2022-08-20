use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics};

use bevy_framepace;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

mod player;
mod pipes;
mod clouds;
mod ui;
mod options;

use player::*;
use pipes::*;
use clouds::*;
use ui::*;
use options::*;


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: GAME_NAME.to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(BACKGROUND_COLOR[0]/255.0, BACKGROUND_COLOR[1]/255.0, BACKGROUND_COLOR[2]/255.0)))
        .add_plugins(DefaultPlugins)
        
        // FPS
        .add_plugin(bevy_framepace::FramepacePlugin)
        .insert_resource(bevy_framepace::FramepaceSettings::default().with_warnings(false))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        
        // Game systems
        .add_startup_system(setup)
        .add_system(player_system)
        .add_system(pipe_system)
        .add_plugin(CloudsPlugin)

        // UI
        .add_plugin(UIPlugin)

        // Saving plugin
        .insert_resource(PkvStore::new("bewuwy", GAME_NAME))

        .run();
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    pkv: ResMut<PkvStore>,
) {

    // Setup the sprite sheet
    let texture_handle = asset_server.load("spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle, 
        Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 
        SPRITESHEET_COLS, SPRITESHEET_ROWS,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Load saved data
    let stats: PlayerStatistics = pkv.get::<PlayerStatistics>(PLAYER_STATS_KEY)
    .unwrap_or_else(|_| PlayerStatistics {
        high_score: 0,
    });

    // Add a 2D Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Spawn the player
    commands.spawn().insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(Vec3::new(PLAYER_X, PLAYER_START_Y, Z_PLAYER)),
        sprite: TextureAtlasSprite::new(0),
        ..Default::default()
    })
    .insert(Player { delta_y: 0.0, dead: false });

    // Spawn pipes
    for i in 0..PIPES_NUMBER {
        spawn_pipe(&mut commands, &texture_atlas_handle, PIPES_START_X + i as f32 * PIPES_GAP_BETWEEN);
    }

    // Spawn clouds
    for i in 0..CLOUDS_NUMBER {
        spawn_cloud(&mut commands, &texture_atlas_handle, CLOUDS_START_X + i as f32 * CLOUDS_GAP_BETWEEN);
    }
    
    // Spawn the game controller
    commands.spawn().insert(GameController{ started: false, score: 0, player_stats: stats });
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
        mut commands: &mut Commands, 
        atlas_handle: &Handle<TextureAtlas>, 
        player: &mut Player, 
        mut player_transform: &mut Transform, 
        pipes_query: &mut Query<&mut PipeParent>,
        mut pkv: ResMut<PkvStore>,
    ) {

        self.started = false;

        if self.score > self.player_stats.high_score {
            self.player_stats.high_score = self.score;

            // Save the high score
            pkv.set(PLAYER_STATS_KEY, &self.player_stats).expect("Failed to save high score");
        }

        self.score = 0;

        player.die(&mut player_transform);

        let mut i = 0.0;
        for mut pipe in pipes_query.iter_mut() {
            pipe.reset(&mut commands, &atlas_handle, PIPES_START_X + i * PIPES_GAP_BETWEEN); // TODO: this is not working

            i += 1.0;
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PlayerStatistics {
    high_score: u32,
}
