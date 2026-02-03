// World - contains all game entities and manages their lifecycle

use crate::entities::*;
use crate::game::config::*;
use crate::game::state::{GameSession, GameState, PlayingState};

/// The game world containing all entities
pub struct World {
    pub session: GameSession,
    pub player: Player,
    pub invader_grid: InvaderGrid,
    pub player_bullets: Vec<Bullet>,
    pub invader_bullets: Vec<Bullet>,
    pub shields: Vec<Shield>,
    pub ufo: Option<Ufo>,
    pub ufo_spawn_timer: f32,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            session: GameSession::new(),
            player: Player::new(),
            invader_grid: InvaderGrid::new(),
            player_bullets: Vec::new(),
            invader_bullets: Vec::new(),
            shields: Self::create_shields(),
            ufo: None,
            ufo_spawn_timer: 0.0,
        }
    }

    fn create_shields() -> Vec<Shield> {
        SHIELD_POSITIONS
            .iter()
            .map(|&(x, y)| Shield::new(x, y))
            .collect()
    }

    pub fn start_game(&mut self) {
        self.session.start_game();
        self.reset_wave();
    }

    pub fn reset_wave(&mut self) {
        self.player = Player::new();
        self.invader_grid = InvaderGrid::new_at_wave(self.session.wave);
        self.player_bullets.clear();
        self.invader_bullets.clear();
        self.shields = Self::create_shields();
        self.ufo = None;
        self.ufo_spawn_timer = 0.0;
    }

    pub fn respawn_player(&mut self) {
        self.player = Player::new();
        self.player_bullets.clear();
    }

    pub fn update(&mut self, delta: f32) {
        self.session.update(delta);

        match self.session.state {
            GameState::Playing => {
                if self.session.playing_state == PlayingState::Respawning {
                    // Respawn player but allow invaders to continue
                    self.respawn_player();
                    self.session.playing_state = PlayingState::Normal;
                }
            }
            GameState::WaveComplete => {
                if self.session.state_timer >= 2.0 {
                    self.reset_wave();
                }
            }
            _ => {}
        }
    }

    pub fn remaining_invaders(&self) -> usize {
        self.invader_grid.alive_count()
    }

    pub fn is_wave_complete(&self) -> bool {
        self.invader_grid.alive_count() == 0
    }

    pub fn has_invasion_reached_bottom(&self) -> bool {
        self.invader_grid.has_reached_bottom()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_world() {
        let world = World::new();
        assert_eq!(world.session.state, GameState::Attract);
        assert_eq!(world.shields.len(), NUM_SHIELDS);
        assert_eq!(world.invader_grid.alive_count(), INVADER_TOTAL);
    }

    #[test]
    fn test_start_game() {
        let mut world = World::new();
        world.start_game();
        assert_eq!(world.session.state, GameState::Playing);
    }

    #[test]
    fn test_remaining_invaders() {
        let world = World::new();
        assert_eq!(world.remaining_invaders(), INVADER_TOTAL);
    }
}
