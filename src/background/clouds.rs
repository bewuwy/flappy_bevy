use bevy::prelude::*;
use rand::prelude::*;

use crate::{game_controller, options::*};

const CLOUDS_SPEED: f32 = 14.0;

fn clouds_setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    clouds_handler: Res<CloudsHandler>,
) {
    // settings
    const CLOUDS_NUMBER: usize = 8;

    // Spawn clouds
    for i in 0..CLOUDS_NUMBER {
        spawn_cloud(
            &mut commands,
            &clouds_handler,
            CLOUDS_START_X + i as f32 * CLOUDS_GAP_BETWEEN,
        );
    }
}

fn clouds_system(
    mut query: Query<&mut CloudParent>,
    mut block_query: Query<(&mut CloudBlock, &mut Transform)>,
    mut commands: Commands,
    clouds_manager: Res<CloudsHandler>,
    game_controller: Res<game_controller::GameController>,
    time: Res<Time>,
) {
    if !game_controller.is_game_paused() {
        let delta_time: f32 = time.delta().as_secs_f32();

        for mut cloud in query.iter_mut() {
            cloud.x += CLOUDS_SPEED * delta_time;

            if cloud.x > SCREEN_X_BOUNDARY + cloud.width_sprites as f32 * SPRITE_SIZE {
                cloud.reset(&mut commands, &clouds_manager);
            }
        }

        for (_, mut transform) in block_query.iter_mut() {
            transform.translation.x += CLOUDS_SPEED * delta_time;
        }
    }
}

// #[derive(Default)]
struct CloudsHandler {
    texture_start: Handle<Image>,
    texture_end: Handle<Image>,
}

impl FromWorld for CloudsHandler {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        CloudsHandler {
            texture_start: asset_server.load("sprites/cloud/start.png"),
            texture_end: asset_server.load("sprites/cloud/end.png"),
        }
    }
}

#[derive(Component)]
struct CloudParent {
    x: f32,
    y: f32,
    width_sprites: u32,
    blocks: Vec<Entity>,
}

#[derive(Component)]
struct CloudBlock;

impl CloudParent {
    fn reset(&mut self, commands: &mut Commands, clouds_handler: &CloudsHandler) {
        for block in self.blocks.iter() {
            commands.entity(*block).despawn();
        }
        self.blocks.clear();

        self.x = -SCREEN_X_BOUNDARY - SPRITE_SIZE * self.width_sprites as f32;
        let mut rng = thread_rng();
        self.y = rng.gen_range(CLOUDS_Y_RANGE[0]..=CLOUDS_Y_RANGE[1]);

        self.spawn_blocks(commands, clouds_handler);
    }

    fn spawn_blocks(&mut self, commands: &mut Commands, clouds_handler: &CloudsHandler) {
        for i in 0..self.width_sprites {
            let block = {
                let texture = {
                    if i == 0 {
                        &clouds_handler.texture_start
                    } else {
                        &clouds_handler.texture_end
                    }
                };

                commands
                    .spawn()
                    .insert_bundle(SpriteBundle {
                        texture: texture.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            self.x + i as f32 * SPRITE_SIZE,
                            self.y,
                            Z_BACKGROUND,
                        )),
                        sprite: Sprite {
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(CloudBlock)
                    .id()
            };

            self.blocks.push(block);
        }
    }
}

fn spawn_cloud(commands: &mut Commands, clouds_handler: &CloudsHandler, x: f32) {
    const CLOUD_WIDTH: usize = 2;

    let mut rng = thread_rng();
    let y = rng.gen_range(CLOUDS_Y_RANGE[0]..=CLOUDS_Y_RANGE[1]);

    let mut cloud = CloudParent {
        x,
        y,
        width_sprites: CLOUD_WIDTH as u32,
        blocks: Vec::new(),
    };

    cloud.spawn_blocks(commands, clouds_handler);

    commands.spawn().insert(cloud);
}

pub struct CloudsPlugin;

impl Plugin for CloudsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CloudsHandler>()
            .add_startup_system(clouds_setup)
            .add_system(clouds_system);
    }
}
