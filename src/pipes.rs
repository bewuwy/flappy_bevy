use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::prelude::*;

use crate::*;

const PIPES_SPEED: f32 = 250.0;

fn pipes_setup(mut commands: Commands, pipes_handler: Res<PipesHandler>) {
    const PIPES_NUMBER: u32 = 5;

    // Spawn pipes
    for i in 0..PIPES_NUMBER {
        spawn_pipe(
            &mut commands,
            &pipes_handler,
            PIPES_START_X + i as f32 * PIPES_GAP_BETWEEN,
        );
    }
}

#[allow(clippy::type_complexity)]
pub fn pipes_system(
    mut commands: Commands,

    (mut player_query, mut pipes_query, mut block_query): (
        Query<(&mut Player, &mut Transform)>,
        Query<&mut PipeParent>,
        Query<(&mut PipeBlock, &mut Transform), Without<Player>>,
    ),

    mut game_controller: ResMut<GameController>,
    time: Res<Time>,
    audio: Res<Audio>,
    pipes_handler: Res<PipesHandler>,
) {
    let delta_time: f32 = time.delta().as_secs_f32();

    // get the player and atlas handle
    let (mut player, player_transform) = player_query.single_mut();

    if game_controller.is_game_running() {
        // update pipe blocks
        for (_, mut transform) in block_query.iter_mut() {
            transform.translation.x -= PIPES_SPEED * delta_time;
        }

        // update pipes
        for mut pipe in pipes_query.iter_mut() {
            pipe.x -= PIPES_SPEED * delta_time;

            // check if pipe off screen
            if pipe.x < -SCREEN_X_BOUNDARY {
                pipe.reset(
                    &mut commands,
                    &pipes_handler,
                    SCREEN_X_BOUNDARY + SPRITE_SIZE,
                );
            }

            // check if player gained point
            if !pipe.passed_score
                && pipe.x - (pipe.width_sprites * SPRITE_SIZE / 2.0)
                    < player_transform.translation.x - SPRITE_SIZE
            {
                pipe.passed_score = true;
                game_controller.score += 1;

                // play the score sound if high score passed
                if game_controller.score == game_controller.player_stats.high_score + 1 {
                    audio
                        .play(pipes_handler.score_sound.clone())
                        .with_volume(0.5);
                }
            }

            // check if player touches bottom pipe
            if pipe.x - (pipe.width_sprites * SPRITE_SIZE / 2.0) < player_transform.translation.x
                && pipe.x + (pipe.width_sprites * SPRITE_SIZE / 2.0)
                    > player_transform.translation.x
                && (PIPE_FLOOR_Y_SPR + pipe.height_sprites as i32) as f32 * SPRITE_SIZE
                    >= player_transform.translation.y
            {
                player.dead = true;
            }

            // check if player touches top pipe
            if pipe.x - (pipe.width_sprites * SPRITE_SIZE / 2.0) < player_transform.translation.x
                && pipe.x + (pipe.width_sprites * SPRITE_SIZE / 2.0)
                    > player_transform.translation.x
                && (PIPE_FLOOR_Y_SPR + pipe.height_sprites as i32 + pipe.y_gap_sprites as i32)
                    as f32
                    * SPRITE_SIZE
                    <= player_transform.translation.y
            {
                player.dead = true;
            }
        }
    }
}

pub struct PipesHandler {
    texture_body: Handle<Image>,
    texture_end: Handle<Image>,
    score_sound: Handle<AudioSource>,
}

impl FromWorld for PipesHandler {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        PipesHandler {
            texture_body: asset_server.load("sprites/pipe/body.png"),
            texture_end: asset_server.load("sprites/pipe/end.png"),
            score_sound: asset_server.load("sounds/score.wav"),
        }
    }
}

#[derive(Component)]
pub struct PipeParent {
    pub x: f32,
    height_sprites: u32,
    width_sprites: f32, // has to be f32, because of 1.5
    y_gap_sprites: u32,
    // pub passed: bool,
    passed_score: bool, // give score in the middle of the pipe
    blocks: Vec<Entity>,
}

#[derive(Component)]
pub struct PipeBlock;

impl PipeParent {
    pub fn reset(&mut self, commands: &mut Commands, pipes_handler: &PipesHandler, x: f32) {
        // self.passed = false;
        self.passed_score = false;

        let mut rng = thread_rng();

        let new_height = rng.gen_range(PIPE_HEIGHT_RANGE_SPR[0]..=PIPE_HEIGHT_RANGE_SPR[1]);
        // let new_x = SCREEN_X_BOUNDARY + SPRITE_SIZE;

        self.x = x;
        self.height_sprites = new_height;

        // despawn old blocks
        for block in self.blocks.iter_mut() {
            commands.entity(*block).despawn();
        }
        self.blocks.clear();

        // spawn new blocks
        self.spawn_blocks(commands, pipes_handler);
    }

    fn spawn_blocks(&mut self, commands: &mut Commands, pipes_handler: &PipesHandler) {
        // spawn bottom pipe
        for i in 0..self.height_sprites {
            for j in 0..PIPE_WIDTH {
                let block_x = self.x + j as f32 * 0.5 * SPRITE_SIZE;
                self.width_sprites = (block_x - self.x + SPRITE_SIZE) / SPRITE_SIZE;

                let flip_x = j + 1 == PIPE_WIDTH;
                let texture = if i + 1 == self.height_sprites {
                    &pipes_handler.texture_end
                } else {
                    &pipes_handler.texture_body
                };
                let sprite = Sprite {
                    flip_x,
                    ..Default::default()
                };

                self.blocks.push(
                    commands
                        .spawn()
                        .insert_bundle(SpriteBundle {
                            texture: texture.clone(),
                            transform: Transform::from_translation(Vec3::new(
                                block_x,
                                (PIPE_FLOOR_Y_SPR + i as i32) as f32 * SPRITE_SIZE,
                                Z_PIPE,
                            )),
                            sprite,
                            ..Default::default()
                        })
                        .insert(PipeBlock)
                        .id(),
                );
            }
        }

        // spawn top pipe
        let top_blocks =
            ((-PIPE_FLOOR_Y_SPR) * 2) as u32 - self.height_sprites - self.y_gap_sprites;
        for i in 0..top_blocks {
            for j in 0..PIPE_WIDTH {
                let block_x = self.x + j as f32 * 0.5 * SPRITE_SIZE;

                let flip_x = j + 1 == PIPE_WIDTH;
                let texture = if i + 1 == top_blocks {
                    &pipes_handler.texture_end
                } else {
                    &pipes_handler.texture_body
                };
                let sprite = Sprite {
                    flip_x,
                    flip_y: true,
                    ..Default::default()
                };

                self.blocks.push(
                    commands
                        .spawn()
                        .insert_bundle(SpriteBundle {
                            texture: texture.clone(),
                            transform: Transform::from_translation(Vec3::new(
                                block_x,
                                (-PIPE_FLOOR_Y_SPR - i as i32) as f32 * SPRITE_SIZE,
                                Z_PIPE,
                            )),
                            sprite,
                            ..Default::default()
                        })
                        .insert(PipeBlock)
                        .id(),
                );
            }
        }
    }
}

pub fn spawn_pipe(commands: &mut Commands, pipes_handler: &PipesHandler, x: f32) {
    let mut rng = thread_rng();
    let height = rng.gen_range(PIPE_HEIGHT_RANGE_SPR[0]..=PIPE_HEIGHT_RANGE_SPR[1]);

    let blocks: Vec<Entity> = Vec::new();

    let mut pipe = PipeParent {
        x,
        height_sprites: height,
        width_sprites: 0.0,
        y_gap_sprites: PIPE_Y_GAP_SPR,
        passed_score: false,
        blocks,
    };

    // spawn pipe blocks
    pipe.spawn_blocks(commands, pipes_handler);

    // spawn pipe parent
    commands.spawn().insert(pipe);
}

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PipesHandler>()
            .add_startup_system(pipes_setup)
            .add_system(pipes_system);
    }
}
