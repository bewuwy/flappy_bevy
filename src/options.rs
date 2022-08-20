pub static GAME_NAME: &str = "Flappy Bevy";

// window settings
    pub static BACKGROUND_COLOR: [f32; 3] =  [87.0, 169.0, 241.0]; // 124.0, 182.0, 222.0 // 109.0, 174.0, 219.0

// Z-index
    pub static Z_PLAYER: f32 = 10.0;
    pub static Z_PIPE: f32 = 5.0;
    pub static Z_BACKGROUND: f32 = 2.0;
    // pub static Z_UI: f32 = 20.0;

// spritesheet settings
    pub static SPRITE_SIZE: f32 = 64.0;

    pub static SCREEN_X_BOUNDARY: f32 = 800.0;
    pub static SCREEN_Y_BOUNDARY: f32 = 500.0;

// assets settings

// gameplay settings
    // pipes
        pub static PIPES_START_X: f32 = 280.0;
        pub static PIPES_GAP_BETWEEN: f32 = 350.0;

        pub static PIPE_HEIGHT_RANGE_SPR: [u32; 2] = [3, 9];
        pub static PIPE_Y_GAP_SPR: u32 = 4;
        pub static PIPE_FLOOR_Y_SPR: i32 = -8; // -500

        pub static PIPE_WIDTH: u32 = 2;

        pub static PIPE_SPEED: f32 = 5.0;

    // player
        pub static PLAYER_X: f32 = -64.0;
        pub static PLAYER_START_Y: f32 = -200.0;

    // clouds
        pub static CLOUDS_START_X: f32 = -SCREEN_X_BOUNDARY - SPRITE_SIZE * 3 as f32;
        pub static CLOUDS_GAP_BETWEEN: f32 = 250.0;
        pub static CLOUDS_SPEED: f32 = 0.7;
        pub static CLOUDS_Y_RANGE: [f32; 2] = [-150.0, 300.0];

// data storage
    pub static PLAYER_STATS_KEY: &str = "player_stats";
