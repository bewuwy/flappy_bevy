use bevy::prelude::*;
use rand::prelude::*;

use crate::options::*;

fn hills_setup(mut commands: Commands, hills_handler: Res<HillsHandler>) {
    const HILL_WIDTH: f32 = 128.0;
    const HILL_HEIGHT: f32 = 128.0;

    // Spawn hills
    for i in 0..(2.0 * SCREEN_X_BOUNDARY / HILL_WIDTH).ceil() as usize {
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                texture: hills_handler
                    .textures
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone(),
                transform: Transform::from_translation(Vec3::new(
                    -SCREEN_X_BOUNDARY + i as f32 * HILL_WIDTH,
                    -SCREEN_Y_BOUNDARY + (HILL_HEIGHT / 2.0),
                    Z_BACKGROUND,
                )),
                sprite: Sprite {
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Hill);
    }
}

fn hills_system(mut hills_query: Query<(&Hill, &mut Transform)>, time: Res<Time>) {
    let delta_time = time.delta().as_secs_f32();

    for (_, mut transform) in hills_query.iter_mut() {
        transform.translation.x -= PIPES_SPEED * delta_time * 0.05;

        if transform.translation.x < -SCREEN_X_BOUNDARY {
            transform.translation.x = SCREEN_X_BOUNDARY;
        }
    }
}

#[derive(Component)]
struct Hill;

struct HillsHandler {
    textures: Vec<Handle<Image>>,
}

impl FromWorld for HillsHandler {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        HillsHandler {
            textures: vec![
                asset_server.load("sprites/hill/1.png"),
                asset_server.load("sprites/hill/2.png"),
                asset_server.load("sprites/hill/3.png"),
            ],
        }
    }
}

pub struct HillsPlugin;

impl Plugin for HillsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HillsHandler>()
            .add_startup_system(hills_setup)
            .add_system(hills_system);
    }
}
