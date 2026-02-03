// RetroVaders - Space Invaders (1978) Recreation
// Main entry point and game loop

// Allow dead code for features not yet fully implemented
#![allow(dead_code)]
#![allow(unused_imports)]

use macroquad::prelude::*;

mod game;
mod entities;
mod systems;
mod rendering;
mod audio;
mod input;

use game::config::*;
use game::state::GameState;
use game::world::World;
use input::InputHandler;
use audio::AudioManager;
use rendering::crt::CrtSettings;
use systems::{collision, movement, shooting, scoring::ScoreManager};

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

struct Game {
    world: World,
    input: InputHandler,
    audio: AudioManager,
    crt: CrtSettings,
    score_manager: ScoreManager,
}

impl Game {
    fn new() -> Self {
        let score_manager = ScoreManager::new();
        let mut world = World::new();

        // Load high score
        world.session.high_score = score_manager.get_top_score();

        Self {
            world,
            input: InputHandler::new(),
            audio: AudioManager::new(),
            crt: CrtSettings::default(),
            score_manager,
        }
    }

    fn update(&mut self, delta: f32) {
        let input_state = self.input.poll();

        // Toggle CRT effects with F1
        if is_key_pressed(KeyCode::F1) {
            self.crt.toggle();
        }

        match self.world.session.state {
            GameState::Attract => {
                if input_state.start || self.input.any_key_pressed() {
                    self.world.start_game();
                }
            }

            GameState::Playing => {
                // Handle pause
                if input_state.pause {
                    self.world.session.pause();
                    return;
                }

                // Movement
                movement::update_player_movement(&mut self.world, &input_state, delta);
                let invaders_moved = movement::update_invader_movement(&mut self.world, delta);
                movement::update_bullets(&mut self.world, delta);
                movement::update_ufo(&mut self.world, delta);

                // Play march audio when invaders move
                if invaders_moved {
                    self.audio.update_march_tempo(self.world.remaining_invaders());
                    self.audio.play_march_step();
                }

                // Shooting
                shooting::handle_player_shooting(&mut self.world, &input_state);
                shooting::handle_invader_shooting(&mut self.world);
                shooting::try_spawn_ufo(&mut self.world, delta);

                // Collisions
                let collision_result = collision::process_collisions(&mut self.world);

                // Handle collision results
                for (_row, _col, points) in collision_result.invaders_killed {
                    self.world.session.add_score(points);
                    self.audio.play(audio::sounds::SoundId::InvaderKilled);
                }

                if let Some(points) = collision_result.ufo_hit {
                    self.world.session.add_score(points);
                    self.audio.play(audio::sounds::SoundId::UfoHit);
                }

                if collision_result.player_hit {
                    self.world.player.kill();
                    self.world.session.lose_life();
                    self.audio.play(audio::sounds::SoundId::PlayerExplosion);
                }

                // Check game over conditions
                collision::check_invasion_game_over(&mut self.world);

                // Check wave complete
                if self.world.is_wave_complete() {
                    self.world.session.complete_wave();
                }
            }

            GameState::Paused => {
                if input_state.pause {
                    self.world.session.unpause();
                }
            }

            GameState::PlayerDeath => {
                self.world.session.update(delta);
            }

            GameState::WaveComplete => {
                self.world.update(delta);
            }

            GameState::GameOver => {
                // Save high score if applicable
                if self.world.session.score > 0 {
                    self.score_manager.check_and_add_score(
                        "PLR",
                        self.world.session.score,
                        self.world.session.wave,
                    );
                }

                if input_state.start || self.input.any_key_pressed() {
                    let high_score = self.score_manager.get_top_score();
                    self.world = World::new();
                    self.world.session.high_score = high_score;
                }
            }
        }

        // Handle quit
        if input_state.quit {
            std::process::exit(0);
        }
    }

    fn render(&self) {
        match self.world.session.state {
            GameState::Attract => {
                rendering::render_attract_screen();
                self.render_crt_hint();
            }

            GameState::Playing | GameState::PlayerDeath | GameState::WaveComplete => {
                rendering::render_world(&self.world);

                // Apply CRT effects
                if self.crt.enabled {
                    rendering::crt::apply_color_zones();
                    rendering::crt::draw_scanlines(&self.crt);
                    rendering::crt::draw_vignette();
                }
            }

            GameState::Paused => {
                rendering::render_world(&self.world);
                rendering::render_paused();
            }

            GameState::GameOver => {
                rendering::render_game_over(&self.world);
            }
        }
    }

    fn render_crt_hint(&self) {
        let scale = SCALE as f32;
        let status = if self.crt.enabled { "ON" } else { "OFF" };
        draw_text(
            &format!("F1: CRT Effects [{status}]"),
            10.0 * scale,
            10.0 * scale,
            10.0 * scale / 3.0,
            Color::new(0.5, 0.5, 0.5, 1.0),
        );
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    // Load assets
    game.audio.load_all().await.ok();

    loop {
        let delta = get_frame_time().min(FRAME_TIME * 2.0); // Cap delta to prevent spiral

        game.update(delta);
        game.render();

        next_frame().await;
    }
}
