// Game configuration constants - matches original 1978 arcade specifications

// Display (original arcade was 224x256, portrait mode)
pub const GAME_WIDTH: u32 = 224;
pub const GAME_HEIGHT: u32 = 256;
pub const SCALE: u32 = 3;
pub const WINDOW_WIDTH: u32 = GAME_WIDTH * SCALE;
pub const WINDOW_HEIGHT: u32 = GAME_HEIGHT * SCALE;
pub const WINDOW_TITLE: &str = "RetroVaders - Space Invaders (1978)";

// Timing
pub const TARGET_FPS: u32 = 60;
pub const FRAME_TIME: f32 = 1.0 / TARGET_FPS as f32;
pub const BASE_INVADER_MOVE_DELAY: f32 = 0.8;
pub const MIN_INVADER_MOVE_DELAY: f32 = 0.05;

// Invader Grid
pub const INVADER_ROWS: usize = 5;
pub const INVADER_COLS: usize = 11;
pub const INVADER_TOTAL: usize = INVADER_ROWS * INVADER_COLS;
pub const INVADER_SPACING_X: f32 = 16.0;
pub const INVADER_SPACING_Y: f32 = 16.0;
pub const INVADER_START_X: f32 = 26.0;
pub const INVADER_START_Y: f32 = 64.0;
pub const INVADER_STEP_PIXELS: f32 = 2.0;
pub const INVADER_DROP_PIXELS: f32 = 8.0;

// Invader dimensions
pub const INVADER_WIDTH: f32 = 12.0;
pub const INVADER_HEIGHT: f32 = 8.0;

// Scoring (original arcade values)
pub const SCORE_SQUID: u32 = 30;
pub const SCORE_CRAB: u32 = 20;
pub const SCORE_OCTOPUS: u32 = 10;
pub const SCORE_UFO_MIN: u32 = 50;
pub const SCORE_UFO_MAX: u32 = 300;

// Player
pub const PLAYER_WIDTH: f32 = 13.0;
pub const PLAYER_HEIGHT: f32 = 8.0;
pub const PLAYER_START_X: f32 = 112.0;
pub const PLAYER_START_Y: f32 = 216.0;
pub const PLAYER_SPEED: f32 = 120.0;
pub const PLAYER_MIN_X: f32 = 8.0;
pub const PLAYER_MAX_X: f32 = 203.0;
pub const PLAYER_LIVES: u32 = 3;
pub const PLAYER_RESPAWN_DELAY: f32 = 2.0;
pub const PLAYER_INVINCIBILITY_TIME: f32 = 1.0;
pub const EXTRA_LIFE_SCORE: u32 = 1500;

// Bullets
pub const PLAYER_BULLET_SPEED: f32 = 400.0;
pub const PLAYER_BULLET_WIDTH: f32 = 1.0;
pub const PLAYER_BULLET_HEIGHT: f32 = 4.0;
pub const INVADER_BULLET_SPEED: f32 = 120.0;
pub const INVADER_BULLET_WIDTH: f32 = 3.0;
pub const INVADER_BULLET_HEIGHT: f32 = 7.0;
pub const MAX_PLAYER_BULLETS: usize = 1;
pub const MAX_INVADER_BULLETS: usize = 3;
pub const INVADER_FIRE_PROBABILITY: f32 = 0.003;

// Shields
pub const NUM_SHIELDS: usize = 4;
pub const SHIELD_WIDTH: f32 = 22.0;
pub const SHIELD_HEIGHT: f32 = 16.0;
pub const SHIELD_DAMAGE_RADIUS: f32 = 3.0;
pub const SHIELD_POSITIONS: [(f32, f32); NUM_SHIELDS] = [
    (32.0, 192.0),
    (78.0, 192.0),
    (124.0, 192.0),
    (170.0, 192.0),
];

// UFO
pub const UFO_WIDTH: f32 = 16.0;
pub const UFO_HEIGHT: f32 = 7.0;
pub const UFO_SPEED: f32 = 50.0;
pub const UFO_Y_POSITION: f32 = 24.0;
pub const UFO_SPAWN_PROBABILITY: f32 = 0.0005;
pub const UFO_MIN_SPAWN_INTERVAL: f32 = 25.0;

// Color zones (original gel overlay Y ranges)
pub const ZONE_UFO_Y_START: f32 = 0.0;
pub const ZONE_UFO_Y_END: f32 = 32.0;
pub const ZONE_PLAY_Y_START: f32 = 32.0;
pub const ZONE_PLAY_Y_END: f32 = 184.0;
pub const ZONE_SHIELD_Y_START: f32 = 184.0;
pub const ZONE_SHIELD_Y_END: f32 = 240.0;
pub const ZONE_HUD_Y_START: f32 = 240.0;
pub const ZONE_HUD_Y_END: f32 = 256.0;

// Wave progression
pub const WAVE_START_Y_INCREASE: f32 = 8.0;
pub const WAVE_SPEED_MULTIPLIER_INCREASE: f32 = 0.05;
pub const WAVE_FIRE_PROBABILITY_INCREASE: f32 = 0.0002;
pub const MAX_DIFFICULTY_WAVE: u32 = 10;
