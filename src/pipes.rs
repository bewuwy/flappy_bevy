use bevy::prelude::*;
use rand::prelude::*;

use crate::*;


#[derive(Component)]
pub struct PipeParent {
    pub x: f32,
    height: f32,
    width: f32,
    y_gap: f32,
    pub passed: bool,
    passed_score: bool, // give score in the middle of the pipe
    blocks: Vec<Entity>,
}

#[derive(Component)]
pub struct PipeBlock;

impl PipeParent {
    pub fn reset(&mut self, commands: &mut Commands, atlas_handle: &Handle<TextureAtlas>, x: f32) {
        self.passed = false;
        self.passed_score = false;

        let mut rng = thread_rng();

        let new_height = rng.gen_range(PIPE_HEIGHT_RANGE[0]..=PIPE_HEIGHT_RANGE[1]);
        // let new_x = SCREEN_X_BOUNDARY + SPRITE_SIZE;

        self.x = x;
        self.height = new_height;

        // despawn old blocks
        for block in self.blocks.iter_mut() {
            commands.entity(*block).despawn();
        }
        self.blocks.clear();

        // spawn new blocks
        self.spawn_blocks(commands, atlas_handle);
    }

    fn spawn_blocks(&mut self, commands: &mut Commands, atlas_handle: &Handle<TextureAtlas>) {
        // spawn bottom pipe
        for i in 0..(self.height/SPRITE_SIZE).ceil() as u32 {
            for j in 0..PIPE_WIDTH {
    
                let block_x = self.x + j as f32 * 0.5 * SPRITE_SIZE;
                self.width = block_x - self.x + SPRITE_SIZE;
    
                self.blocks.push(commands.spawn().insert_bundle(SpriteSheetBundle {
                    texture_atlas: atlas_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(block_x, PIPE_FLOOR_Y + (i as f32 * SPRITE_SIZE), 0.0)),
                    sprite: TextureAtlasSprite::new(1),
                    ..Default::default()
                })
                .insert(PipeBlock).id());
            }
        }

        // spawn top pipe
        for i in 0..((-PIPE_FLOOR_Y - PIPE_FLOOR_Y - self.height - self.y_gap)/SPRITE_SIZE).ceil() as u32 {
            for j in 0..PIPE_WIDTH {
    
                let block_x = self.x + j as f32 * 0.5 * SPRITE_SIZE;
    
                self.blocks.push(commands.spawn().insert_bundle(SpriteSheetBundle {
                    texture_atlas: atlas_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(block_x, -PIPE_FLOOR_Y - (i as f32 * SPRITE_SIZE), 0.0)),
                    sprite: TextureAtlasSprite::new(1),
                    ..Default::default()
                })
                .insert(PipeBlock).id());
            }
        }
    }
}


pub fn spawn_pipe(mut commands: &mut Commands, atlas_handle: &Handle<TextureAtlas>, x: f32) {
    let mut rng = thread_rng();
    let height = rng.gen_range(200..=600) as f32;

    let blocks: Vec<Entity> = Vec::new();

    let mut pipe = PipeParent {
        x,
        height,
        width: 0.0,
        y_gap: PIPE_Y_GAP,
        passed: false,
        passed_score: false,
        blocks,
    };

    // spawn pipe blocks
    pipe.spawn_blocks(&mut commands, &atlas_handle);

    // spawn pipe parent
    commands.spawn().insert(pipe);
}


pub fn pipe_system (
    mut player_query: Query<(&mut Player, &mut Transform, &Handle<TextureAtlas>)>,
    mut pipes_query: Query<&mut PipeParent>,
    mut block_query: Query<(&mut PipeBlock, &mut Transform), Without<Player>>,
    mut controller_query: Query<&mut GameController>,
    mut commands: Commands,
) {
    // get the game controller
    let mut game_controller = controller_query.single_mut();

    // get the player and atlas handle
    let (mut player, player_transform, atlas_handle,) = player_query.single_mut();

    if game_controller.started {
        // update pipe blocks
        for (_, mut transform) in block_query.iter_mut() {
            transform.translation.x -= PIPE_SPEED;
        }

        // update pipes
        for mut pipe in pipes_query.iter_mut() {
            pipe.x -= PIPE_SPEED;

            // check if pipe off screen
            if pipe.x < -SCREEN_X_BOUNDARY {
                pipe.reset(&mut commands, atlas_handle, SCREEN_X_BOUNDARY+SPRITE_SIZE);
            }

            // check if player gained point
            if !pipe.passed_score && pipe.x - (pipe.width / 2.0) < player_transform.translation.x - SPRITE_SIZE {
                pipe.passed_score = true;
                game_controller.score += 1;
            }

            // check if player passed pipe
            if !pipe.passed && pipe.x + (pipe.width / 2.0) < player_transform.translation.x - SPRITE_SIZE {
                pipe.passed = true;
            }

            // check if player touches bottom pipe
            if pipe.x - (pipe.width / 2.0) < player_transform.translation.x
                && pipe.x + (pipe.width / 2.0) > player_transform.translation.x
                && PIPE_FLOOR_Y + pipe.height >= player_transform.translation.y
                {
                
                    player.dead = true;
            }

            // check if player touches top pipe
            if pipe.x - (pipe.width / 2.0) < player_transform.translation.x
                && pipe.x + (pipe.width / 2.0) > player_transform.translation.x
                && PIPE_FLOOR_Y + pipe.height + pipe.y_gap <= player_transform.translation.y
                {

                    player.dead = true;
            }

        }

    }
}
