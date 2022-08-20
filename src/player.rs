use bevy::prelude::*;

use crate::*;


static JUMP_FORCE: f32 = 8.0;
static GRAVITY: f32 = 0.4;


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


pub fn player_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform, &mut Handle<TextureAtlas>)>,
    mut controller_query: Query<&mut GameController>,
    mut pipes_query: Query<&mut PipeParent>,
    mut commands: Commands,
    pkv: ResMut<PkvStore>,
) {
    const MIN_ROTATION : f32 = -0.4;
    const MAX_ROTATION : f32 = 0.4;
    const ROTATION_SPEED : f32 = 0.05;


    // get the player
    let (mut player, mut transform, atlas_handle) = query.single_mut();

    // get the game controller
    let mut game_controller = controller_query.single_mut();

    // input processing
    if keyboard_input.pressed(KeyCode::Space) {
        player.delta_y = JUMP_FORCE;
        game_controller.started = true;

        let rotation = MAX_ROTATION - transform.rotation.z;
        transform.rotate_z(rotation);
    }

    // physics
    transform.translation.y += player.delta_y;

    if game_controller.started {
        player.delta_y -= GRAVITY;

        if transform.rotation.z > MIN_ROTATION {
            transform.rotate_z(-ROTATION_SPEED);
        }
    }
    else {
        // idle animation
        if transform.translation.y > PLAYER_START_Y - 20.0 {
            player.delta_y -= GRAVITY/4.0;
        }
        else {
            player.delta_y += GRAVITY/2.0;
        }
    }

    // check if player off screen
    if transform.translation.y < -SCREEN_Y_BOUNDARY || transform.translation.y > SCREEN_Y_BOUNDARY {

        player.dead = true;
    }

    // check if player dead
    if player.dead {

        // reset game
        game_controller.reset_game(&mut commands, &atlas_handle, &mut player, &mut transform, &mut pipes_query, pkv);
    }

}
