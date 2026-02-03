// Rendering module - sprite drawing and visual effects

pub mod sprites;
pub mod animation;
pub mod crt;

use macroquad::prelude::*;
use crate::game::world::World;
use crate::game::config::*;
use crate::entities::*;

/// Render the entire game world
pub fn render_world(world: &World) {
    // Clear with black background
    clear_background(BLACK);

    // Render game elements
    render_shields(world);
    render_invaders(world);
    render_player(world);
    render_bullets(world);
    render_ufo(world);
    render_hud(world);
}

fn render_shields(world: &World) {
    for shield in &world.shields {
        let color = Color::new(0.0, 1.0, 0.0, 1.0);  // Green

        for (row_idx, row) in shield.pixels.iter().enumerate() {
            for (col_idx, &solid) in row.iter().enumerate() {
                if solid {
                    let x = shield.x + col_idx as f32;
                    let y = shield.y + row_idx as f32;
                    draw_rectangle(x * SCALE as f32, y * SCALE as f32,
                                 SCALE as f32, SCALE as f32, color);
                }
            }
        }
    }
}

fn render_invaders(world: &World) {
    for row in &world.invader_grid.invaders {
        for invader in row {
            if !invader.alive {
                continue;
            }

            // Color based on type (or use sprites when loaded)
            let color = match invader.invader_type {
                InvaderType::Squid => Color::new(1.0, 1.0, 1.0, 1.0),
                InvaderType::Crab => Color::new(0.8, 0.8, 1.0, 1.0),
                InvaderType::Octopus => Color::new(0.6, 0.6, 1.0, 1.0),
            };

            // Draw as rectangle for now (sprites will replace this)
            draw_rectangle(
                invader.x * SCALE as f32,
                invader.y * SCALE as f32,
                INVADER_WIDTH * SCALE as f32,
                INVADER_HEIGHT * SCALE as f32,
                color,
            );
        }
    }
}

fn render_player(world: &World) {
    if !world.player.alive {
        return;
    }

    // Blink when invincible
    if world.player.invincible {
        let blink = (world.session.state_timer * 10.0) as i32 % 2 == 0;
        if !blink {
            return;
        }
    }

    let color = Color::new(0.0, 1.0, 0.0, 1.0);  // Green

    draw_rectangle(
        world.player.x * SCALE as f32,
        world.player.y * SCALE as f32,
        PLAYER_WIDTH * SCALE as f32,
        PLAYER_HEIGHT * SCALE as f32,
        color,
    );
}

fn render_bullets(world: &World) {
    // Player bullets - white
    for bullet in &world.player_bullets {
        if !bullet.alive {
            continue;
        }

        draw_rectangle(
            bullet.x * SCALE as f32,
            bullet.y * SCALE as f32,
            bullet.width() * SCALE as f32,
            bullet.height() * SCALE as f32,
            WHITE,
        );
    }

    // Invader bullets - white
    for bullet in &world.invader_bullets {
        if !bullet.alive {
            continue;
        }

        draw_rectangle(
            bullet.x * SCALE as f32,
            bullet.y * SCALE as f32,
            bullet.width() * SCALE as f32,
            bullet.height() * SCALE as f32,
            WHITE,
        );
    }
}

fn render_ufo(world: &World) {
    if let Some(ref ufo) = world.ufo {
        if !ufo.alive {
            return;
        }

        let color = Color::new(1.0, 0.0, 0.0, 1.0);  // Red

        draw_rectangle(
            ufo.x * SCALE as f32,
            ufo.y * SCALE as f32,
            UFO_WIDTH * SCALE as f32,
            UFO_HEIGHT * SCALE as f32,
            color,
        );
    }
}

fn render_hud(world: &World) {
    let scale = SCALE as f32;

    // Score
    draw_text(
        &format!("SCORE: {:05}", world.session.score),
        8.0 * scale,
        250.0 * scale,
        20.0 * scale / 3.0,
        WHITE,
    );

    // High score
    draw_text(
        &format!("HI: {:05}", world.session.high_score),
        100.0 * scale,
        250.0 * scale,
        20.0 * scale / 3.0,
        WHITE,
    );

    // Lives
    draw_text(
        &format!("LIVES: {}", world.session.lives),
        170.0 * scale,
        250.0 * scale,
        20.0 * scale / 3.0,
        WHITE,
    );
}

/// Render attract mode / title screen
pub fn render_attract_screen() {
    clear_background(BLACK);

    let scale = SCALE as f32;

    draw_text(
        "RETROVADERS",
        50.0 * scale,
        80.0 * scale,
        30.0 * scale / 3.0,
        WHITE,
    );

    draw_text(
        "SPACE INVADERS (1978)",
        35.0 * scale,
        100.0 * scale,
        15.0 * scale / 3.0,
        Color::new(0.6, 0.6, 0.6, 1.0),
    );

    draw_text(
        "PRESS SPACE TO START",
        40.0 * scale,
        160.0 * scale,
        15.0 * scale / 3.0,
        GREEN,
    );

    // Score table
    draw_text(
        "SCORE ADVANCE TABLE",
        50.0 * scale,
        190.0 * scale,
        12.0 * scale / 3.0,
        WHITE,
    );

    draw_text(
        "= 30 POINTS",
        100.0 * scale,
        205.0 * scale,
        10.0 * scale / 3.0,
        WHITE,
    );

    draw_text(
        "= 20 POINTS",
        100.0 * scale,
        215.0 * scale,
        10.0 * scale / 3.0,
        WHITE,
    );

    draw_text(
        "= 10 POINTS",
        100.0 * scale,
        225.0 * scale,
        10.0 * scale / 3.0,
        WHITE,
    );

    draw_text(
        "= ? MYSTERY",
        100.0 * scale,
        235.0 * scale,
        10.0 * scale / 3.0,
        RED,
    );
}

/// Render game over screen
pub fn render_game_over(world: &World) {
    render_world(world);

    let scale = SCALE as f32;

    // Semi-transparent overlay
    draw_rectangle(
        0.0,
        0.0,
        WINDOW_WIDTH as f32,
        WINDOW_HEIGHT as f32,
        Color::new(0.0, 0.0, 0.0, 0.7),
    );

    draw_text(
        "GAME OVER",
        60.0 * scale,
        120.0 * scale,
        30.0 * scale / 3.0,
        RED,
    );

    draw_text(
        &format!("FINAL SCORE: {:05}", world.session.score),
        45.0 * scale,
        150.0 * scale,
        15.0 * scale / 3.0,
        WHITE,
    );

    draw_text(
        &format!("WAVE REACHED: {}", world.session.wave),
        55.0 * scale,
        165.0 * scale,
        15.0 * scale / 3.0,
        WHITE,
    );

    draw_text(
        "PRESS SPACE TO CONTINUE",
        30.0 * scale,
        200.0 * scale,
        12.0 * scale / 3.0,
        GREEN,
    );
}

/// Render paused overlay
pub fn render_paused() {
    let scale = SCALE as f32;

    draw_rectangle(
        0.0,
        0.0,
        WINDOW_WIDTH as f32,
        WINDOW_HEIGHT as f32,
        Color::new(0.0, 0.0, 0.0, 0.5),
    );

    draw_text(
        "PAUSED",
        80.0 * scale,
        130.0 * scale,
        30.0 * scale / 3.0,
        WHITE,
    );

    draw_text(
        "PRESS P TO RESUME",
        50.0 * scale,
        160.0 * scale,
        12.0 * scale / 3.0,
        GREEN,
    );
}
