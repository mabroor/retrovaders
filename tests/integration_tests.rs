// Integration tests for RetroVaders

use retrovaders::game::config::*;
use retrovaders::game::state::{GameSession, GameState};
use retrovaders::game::world::World;
use retrovaders::entities::{Bullet, InvaderType};

/// Test: Game starts in attract mode
#[test]
fn test_game_starts_in_attract() {
    let world = World::new();
    assert_eq!(world.session.state, GameState::Attract);
}

/// Test: Starting a game transitions to playing state
#[test]
fn test_start_game_transition() {
    let mut world = World::new();
    world.start_game();
    assert_eq!(world.session.state, GameState::Playing);
    assert_eq!(world.session.lives, PLAYER_LIVES);
    assert_eq!(world.session.score, 0);
    assert_eq!(world.session.wave, 1);
}

/// Test: Full grid has 55 invaders
#[test]
fn test_invader_count() {
    let world = World::new();
    assert_eq!(world.invader_grid.alive_count(), INVADER_TOTAL);
}

/// Test: Invader types are correct per row
#[test]
fn test_invader_types() {
    let world = World::new();

    // Row 0 = Squid
    assert_eq!(world.invader_grid.invaders[0][0].invader_type, InvaderType::Squid);

    // Row 1-2 = Crab
    assert_eq!(world.invader_grid.invaders[1][0].invader_type, InvaderType::Crab);
    assert_eq!(world.invader_grid.invaders[2][0].invader_type, InvaderType::Crab);

    // Row 3-4 = Octopus
    assert_eq!(world.invader_grid.invaders[3][0].invader_type, InvaderType::Octopus);
    assert_eq!(world.invader_grid.invaders[4][0].invader_type, InvaderType::Octopus);
}

/// Test: Scoring is correct for each invader type
#[test]
fn test_invader_scoring() {
    assert_eq!(InvaderType::Squid.points(), 30);
    assert_eq!(InvaderType::Crab.points(), 20);
    assert_eq!(InvaderType::Octopus.points(), 10);
}

/// Test: Player starts at correct position
#[test]
fn test_player_start_position() {
    let world = World::new();
    assert_eq!(world.player.x, PLAYER_START_X);
    assert_eq!(world.player.y, PLAYER_START_Y);
}

/// Test: Shields are created at correct positions
#[test]
fn test_shield_positions() {
    let world = World::new();
    assert_eq!(world.shields.len(), NUM_SHIELDS);

    for (i, shield) in world.shields.iter().enumerate() {
        assert_eq!(shield.x, SHIELD_POSITIONS[i].0);
        assert_eq!(shield.y, SHIELD_POSITIONS[i].1);
    }
}

/// Test: Losing all lives triggers game over
#[test]
fn test_game_over_on_zero_lives() {
    let mut session = GameSession::new();
    session.start_game();
    session.lives = 1;
    session.lose_life();
    assert_eq!(session.state, GameState::GameOver);
}

/// Test: Wave complete increments wave number
#[test]
fn test_wave_complete() {
    let mut session = GameSession::new();
    session.start_game();
    assert_eq!(session.wave, 1);
    session.complete_wave();
    assert_eq!(session.wave, 2);
}

/// Test: Extra life awarded at score threshold
#[test]
fn test_extra_life_award() {
    let mut session = GameSession::new();
    session.start_game();
    let initial_lives = session.lives;

    session.add_score(EXTRA_LIFE_SCORE);

    assert_eq!(session.lives, initial_lives + 1);
    assert!(session.extra_life_awarded);
}

/// Test: Extra life only awarded once
#[test]
fn test_extra_life_once() {
    let mut session = GameSession::new();
    session.start_game();

    session.add_score(EXTRA_LIFE_SCORE);
    let lives_after_first = session.lives;

    // Score more points
    session.add_score(EXTRA_LIFE_SCORE);

    // Lives should not increase again
    assert_eq!(session.lives, lives_after_first);
}

/// Test: Bullet speeds are correct
#[test]
fn test_bullet_speeds() {
    let player_bullet = Bullet::player_bullet(100.0, 100.0);
    let invader_bullet = Bullet::invader_bullet(100.0, 100.0);

    assert_eq!(player_bullet.height(), PLAYER_BULLET_HEIGHT);
    assert_eq!(invader_bullet.height(), INVADER_BULLET_HEIGHT);
}

/// Test: Invader speed increases as invaders are eliminated
#[test]
fn test_speed_acceleration() {
    let mut world = World::new();
    world.start_game();

    let initial_delay = world.invader_grid.calculate_move_delay();

    // Kill half the invaders
    for row in 0..INVADER_ROWS {
        for col in 0..INVADER_COLS / 2 {
            world.invader_grid.kill_at(row, col);
        }
    }

    let new_delay = world.invader_grid.calculate_move_delay();
    assert!(new_delay < initial_delay);
}

/// Test: Wave cleared when all invaders killed
#[test]
fn test_wave_clear() {
    let mut world = World::new();
    world.start_game();

    // Kill all invaders
    for row in 0..INVADER_ROWS {
        for col in 0..INVADER_COLS {
            world.invader_grid.kill_at(row, col);
        }
    }

    assert!(world.is_wave_complete());
    assert_eq!(world.invader_grid.alive_count(), 0);
}

/// Test: High score preserved across games
#[test]
fn test_high_score_preserved() {
    let mut session = GameSession::new();
    session.start_game();
    session.add_score(5000);

    let high_score = session.high_score;
    assert_eq!(high_score, 5000);

    // Simulate new game
    let mut new_session = GameSession::new();
    new_session.high_score = high_score;
    new_session.start_game();

    assert_eq!(new_session.high_score, 5000);
    assert_eq!(new_session.score, 0);
}

/// Test: Pause and unpause work correctly
#[test]
fn test_pause_unpause() {
    let mut session = GameSession::new();
    session.start_game();

    session.pause();
    assert_eq!(session.state, GameState::Paused);

    session.unpause();
    assert_eq!(session.state, GameState::Playing);
}

/// Test: Can't pause when not playing
#[test]
fn test_pause_only_when_playing() {
    let mut session = GameSession::new();
    // In attract mode
    session.pause();
    assert_eq!(session.state, GameState::Attract);
}
