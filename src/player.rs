use bevy::prelude::*;

use crate::*;

static JUMP_FORCE: f32 = 10.0;
static GRAVITY: f32 = 30.0;

fn player_setup(mut commands: Commands, player_handler: Res<PlayerHandler>) {
    // Spawn the player
    let texture = &player_handler.texture;
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: texture.clone(),
            transform: Transform::from_translation(Vec3::new(PLAYER_X, PLAYER_START_Y, Z_PLAYER)),
            sprite: Sprite {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            delta_y: 0.0,
            dead: false,
        });
}

pub fn player_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
    mut controller_query: Query<&mut GameController>,
    mut pipes_query: Query<&mut PipeParent>,
    mut commands: Commands,
    pipes_handler: Res<PipesHandler>,
    pkv: ResMut<PkvStore>,
    time: Res<Time>,
) {
    const MIN_ROTATION: f32 = -0.4;
    const MAX_ROTATION: f32 = 0.7;
    const ROTATION_SPEED: f32 = 3.0;

    let delta_time: f32 = time.delta().as_secs_f32();

    // get the player
    let (mut player, mut transform) = query.single_mut();

    // get the game controller
    let mut game_controller = controller_query.single_mut();

    // input processing
    if keyboard_input.just_pressed(KeyCode::Space) {
        player.delta_y = JUMP_FORCE;
        game_controller.started = true;

        let rotation = MAX_ROTATION - transform.rotation.z;
        transform.rotate_z(rotation);
    }

    // physics
    transform.translation.y += player.delta_y;

    if game_controller.started {
        player.delta_y -= GRAVITY * delta_time;

        if transform.rotation.z > MIN_ROTATION {
            transform.rotate_z(-ROTATION_SPEED * delta_time);
        }
    } else {
        // idle animation
        if transform.translation.y > PLAYER_START_Y - 20.0 {
            player.delta_y -= GRAVITY * delta_time / 4.0;
        } else {
            player.delta_y += GRAVITY * delta_time / 2.0;
        }
    }

    // check if player off screen
    if transform.translation.y < -SCREEN_Y_BOUNDARY || transform.translation.y > SCREEN_Y_BOUNDARY {
        player.dead = true;
    }

    // check if player dead
    if player.dead {
        // reset game
        game_controller.reset_game(
            &mut commands,
            &mut player,
            &mut transform,
            &mut pipes_query,
            &pipes_handler,
            pkv,
        );
    }
}

// #[derive(Default)]
struct PlayerHandler {
    texture: Handle<Image>,
}

impl FromWorld for PlayerHandler {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        PlayerHandler {
            texture: asset_server.load("sprites/bird.png"),
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub delta_y: f32,
    pub dead: bool,
}

impl Player {
    pub fn die(&mut self, player_transform: &mut Transform) {
        self.delta_y = 0.0;
        player_transform.translation.y = PLAYER_START_Y;
        player_transform.rotation.z = 0.0;
        player_transform.rotation.w = 1.0;

        self.dead = false;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerHandler>()
            .add_startup_system(player_setup)
            .add_system(player_system);
    }
}
