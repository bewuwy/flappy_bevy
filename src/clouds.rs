use bevy::prelude::*;

use rand::prelude::*;

use crate::*;

// TODO: fix atlas_handle


fn clouds_system(
    mut query: Query<&mut CloudParent>,
    mut block_query: Query<(&mut CloudBlock, &mut Transform)>,
    mut commands: Commands,
    atlas_handle_query: Query<&Handle<TextureAtlas>, With<Player>>,
) {
    // get atlas handle
    let atlas_handle = atlas_handle_query.single();
    
    for mut cloud in query.iter_mut() {
        cloud.x += CLOUDS_SPEED;

        if cloud.x > SCREEN_X_BOUNDARY + cloud.width_sprites as f32 * SPRITE_SIZE {
            cloud.reset(&mut commands, &atlas_handle);
        }
    }

    for (_, mut transform) in block_query.iter_mut() {
        transform.translation.x += CLOUDS_SPEED;
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
    fn reset(&mut self, mut commands: &mut Commands, atlas_handle: &Handle<TextureAtlas>) {
        for block in self.blocks.iter() {
            commands.entity(*block).despawn();
        }
        self.blocks.clear();

        self.x = -SCREEN_X_BOUNDARY - SPRITE_SIZE * self.width_sprites as f32;
        let mut rng = thread_rng();
        self.y = rng.gen_range(CLOUDS_Y_RANGE[0]..=CLOUDS_Y_RANGE[1]);

        self.spawn_blocks(&mut commands, &atlas_handle);
    }

    fn spawn_blocks(&mut self, commands: &mut Commands, atlas_handle: &Handle<TextureAtlas>) {
        for i in 0..self.width_sprites {
            let block = { 
                let index = {
                    if i == 0 {
                        CLOUD_SPRITE_START_INDEX      
                    } else {
                        CLOUD_SPRITE_END_INDEX
                    }
                };
    
                commands.spawn()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: atlas_handle.clone(),
                        transform: Transform::from_translation(Vec3::new(self.x + i as f32 * SPRITE_SIZE, self.y, Z_BACKGROUND)),
                        sprite: TextureAtlasSprite::new(index),
                        ..Default::default()
                    })
                    .insert(CloudBlock).id()
            };
    
            self.blocks.push(block);
        }
    }
}

pub fn spawn_cloud(mut commands: &mut Commands, atlas_handle: &Handle<TextureAtlas>, x: f32) {
    const CLOUD_WIDTH: usize = 2;

    let mut rng = thread_rng();
    let y = rng.gen_range(CLOUDS_Y_RANGE[0]..=CLOUDS_Y_RANGE[1]);

    let mut cloud = CloudParent {
        x, y, width_sprites: CLOUD_WIDTH as u32, blocks: Vec::new(),
    };

    cloud.spawn_blocks(&mut commands, &atlas_handle);

    commands.spawn().insert(cloud);
}

pub struct CloudsPlugin;

impl Plugin for CloudsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(clouds_system);
    }
}
