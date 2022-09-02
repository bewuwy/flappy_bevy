use bevy::prelude::*;
// #[cfg(not(target_arch="aarch64-linux-android"))]
// #[cfg(not(target_arch="armv7-linux-androideabi"))]
// use bevy_kira_audio::prelude::*;

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
            hit_sound: false,
            lose_sound: false,
            animation: PlayerAnimation::Idle,
        });
}

fn player_system(
    mut commands: Commands,

    mut query: Query<(&mut Player, &mut Transform)>,
    mut pipes_query: Query<&mut PipeParent>,

    (time, touches): ( // pkv, audio): (
        Res<Time>,
        Res<Touches>, // Res<Input<KeyCode>>,
        // ResMut<PkvStore>,
        // Res<Audio>,
    ),
    (mut game_controller, pipes_handler): ( // player_handler): (
        ResMut<GameController>,
        Res<PipesHandler>,
        // Res<PlayerHandler>,
    ),
) {
    const MIN_ROTATION: f32 = -0.4;
    const MAX_ROTATION: f32 = 0.4;
    const ROTATION_SPEED: f32 = 3.0;

    let delta_time: f32 = time.delta().as_secs_f32();

    // get the player
    let (mut player, mut transform) = query.single_mut();

    // input processing
    // for finger in touches.iter() {
        if touches.any_just_pressed()
            && (game_controller.is_game_running() || game_controller.game_state == GameState::Waiting)
        {
            player.delta_y = JUMP_FORCE;
            game_controller.game_state = GameState::Started;

            // // play the jump sound
            // #[cfg(not(target_arch="aarch64-linux-android"))]
            // #[cfg(not(target_arch="armv7-linux-androideabi"))]    
            // audio
            //     .play(player_handler.jump_sound.clone())
            //     .with_volume(game_controller.settings.effects_vol_level * 0.5);

            // jump animation
            player.animation = PlayerAnimation::Jump;
        } else if touches.any_just_released() {
            // stop the jump animation
            player.animation = PlayerAnimation::Fall;
        }
    // }

    // if keyboard_input.just_pressed(KeyCode::P) {
    //     // game_controller.game_state = GameState::Paused;
    //     game_controller.score += 10;
    // }

    if game_controller.is_game_running() {
        // apply gravity
        player.delta_y -= GRAVITY * delta_time;
    } else if game_controller.game_state == GameState::Waiting {
        // idle animation
        player.animation = PlayerAnimation::Idle;
    }

    // player animation
    if !game_controller.is_game_paused() {
        match player.animation {
            PlayerAnimation::Idle => {
                if transform.translation.y > PLAYER_START_Y - 20.0 {
                    player.delta_y -= GRAVITY * delta_time / 4.0;
                } else {
                    player.delta_y += GRAVITY * delta_time / 2.0;
                }
            }
            PlayerAnimation::Jump => {
                let rotation = MAX_ROTATION - transform.rotation.z;
                transform.rotate_z(rotation);
            }
            PlayerAnimation::Death => {
                if !player.hit_sound {
                    // #[cfg(not(target_arch="aarch64-linux-android"))]
                    // #[cfg(not(target_arch="armv7-linux-androideabi"))]                
                    // audio
                    //     .play(player_handler.hit_sound.clone())
                    //     .with_volume(game_controller.settings.effects_vol_level);
                    player.hit_sound = true;
                }
                if game_controller.is_game_finished(&transform) && !player.lose_sound {
                    // #[cfg(not(target_arch="aarch64-linux-android"))]
                    // #[cfg(not(target_arch="armv7-linux-androideabi"))]                
                    // audio
                    //     .play(player_handler.lose_sound.clone())
                    //     .with_volume(2.0 * game_controller.settings.effects_vol_level);
                    player.lose_sound = true;
                }

                player.delta_y -= GRAVITY * 2.0 * delta_time;

                if transform.rotation.z > MIN_ROTATION * 1.4 {
                    transform.rotate_z(-ROTATION_SPEED * 1.5 * delta_time);
                }
            }
            PlayerAnimation::Fall => {
                // rotation animation
                if transform.rotation.z > MIN_ROTATION {
                    transform.rotate_z(-ROTATION_SPEED * delta_time);
                }
            }
        }
    }

    if !game_controller.is_game_paused() {
        transform.translation.y += player.delta_y;
    }

    // check if player off screen
    if transform.translation.y < -SCREEN_Y_BOUNDARY || transform.translation.y > SCREEN_Y_BOUNDARY {
        player.dead = true;
    }

    // check if player dead
    if player.dead {
        if game_controller.game_state != GameState::Restart {
            game_controller.game_state = GameState::Finished;
        }
        player.animation = PlayerAnimation::Death;

        game_controller.update_highscore();

        if // keyboard_input.just_pressed(KeyCode::Space)
            // || keyboard_input.just_pressed(KeyCode::Escape) ||
            game_controller.game_state == GameState::Restart
        {
            // reset game
            game_controller.reset_game(
                &mut commands,
                &mut player,
                &mut transform,
                &mut pipes_query,
                &pipes_handler,
            );
        }
    }
}

pub struct PlayerHandler {
    texture: Handle<Image>,
    // jump_sound: Handle<AudioSource>,
    // hit_sound: Handle<AudioSource>,
    // lose_sound: Handle<AudioSource>,
}

impl FromWorld for PlayerHandler {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        PlayerHandler {
            texture: asset_server.load("sprites/bird.png"),
            // jump_sound: asset_server.load("sounds/jump.wav"),
            // hit_sound: asset_server.load("sounds/hit.wav"),
            // lose_sound: asset_server.load("sounds/lose.wav"),
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub delta_y: f32,
    pub dead: bool,
    hit_sound: bool,
    lose_sound: bool,
    animation: PlayerAnimation,
}

enum PlayerAnimation {
    Fall,
    Jump,
    Idle,
    Death,
}

impl Player {
    pub fn die(&mut self, player_transform: &mut Transform) {
        self.delta_y = 0.0;
        player_transform.translation.y = PLAYER_START_Y;
        player_transform.rotation.z = 0.0;
        player_transform.rotation.w = 1.0;

        self.dead = false;
        self.hit_sound = false;
        self.lose_sound = false;
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
