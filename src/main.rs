use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics};
use bevy_framepace;

mod player;
mod pipes;
mod ui;
mod options;

use player::*;
use pipes::*;
use ui::*;
use options::*;


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Flappy Bevy".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        
        // FPS
        .add_plugin(bevy_framepace::FramepacePlugin)
        .insert_resource(bevy_framepace::FramepaceSettings::default().with_warnings(false))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        
        // Game systems
        .add_startup_system(setup)
        .add_system(player_system)
        .add_system(pipe_system)

        // UI
        .add_plugin(UIPlugin)

        .run();
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    // Setup the sprite sheet
    let texture_handle = asset_server.load("spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 2, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Add a 2D Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Spawn the player
    commands.spawn().insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, PLAYER_START_Y, 1.0)),
        sprite: TextureAtlasSprite::new(0),
        ..Default::default()
    })
    .insert(Player { delta_y: 0.0, dead: false });

    // Spawn pipes
    const PIPE_NUMBER: u32 = 5;

    for i in 0..PIPE_NUMBER {
        spawn_pipe(&mut commands, &texture_atlas_handle, PIPES_START_X + i as f32 * PIPES_GAP_BETWEEN);
    }
    
    // Spawn the game controller
    commands.spawn().insert(GameController{ started: false, score: 0 });
}


#[derive(Component)]
pub struct GameController {
    started: bool,
    score: u32,
}

impl GameController {
    pub fn reset_game(
        &mut self, 
        mut commands: &mut Commands, 
        atlas_handle: &Handle<TextureAtlas>, 
        player: &mut Player, 
        mut player_transform: &mut Transform, 
        pipes_query: &mut Query<&mut PipeParent>
    ) {

        self.started = false;
        self.score = 0;

        player.die(&mut player_transform);

        let mut i = 0.0;
        for mut pipe in pipes_query.iter_mut() {
            pipe.reset(&mut commands, &atlas_handle, PIPES_START_X + i * PIPES_GAP_BETWEEN); // TODO: this is not working

            i += 1.0;
        }
    }
}
