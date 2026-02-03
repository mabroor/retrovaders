// Game state management - tracks current game phase and transitions

use crate::game::config::*;

/// Main game states following original arcade flow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameState {
    /// Demo/title screen
    #[default]
    Attract,
    /// Active gameplay
    Playing,
    /// Game paused
    Paused,
    /// Player was hit, showing explosion
    PlayerDeath,
    /// All invaders eliminated
    WaveComplete,
    /// No lives remaining or invasion reached bottom
    GameOver,
}


/// Playing substates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayingState {
    #[default]
    Normal,
    Respawning,
}


/// Tracks game session data
#[derive(Debug, Clone)]
pub struct GameSession {
    pub state: GameState,
    pub playing_state: PlayingState,
    pub score: u32,
    pub high_score: u32,
    pub lives: u32,
    pub wave: u32,
    pub state_timer: f32,
    pub extra_life_awarded: bool,
}

impl Default for GameSession {
    fn default() -> Self {
        Self::new()
    }
}

impl GameSession {
    pub fn new() -> Self {
        Self {
            state: GameState::Attract,
            playing_state: PlayingState::Normal,
            score: 0,
            high_score: 0,
            lives: PLAYER_LIVES,
            wave: 1,
            state_timer: 0.0,
            extra_life_awarded: false,
        }
    }

    pub fn start_game(&mut self) {
        self.state = GameState::Playing;
        self.playing_state = PlayingState::Normal;
        self.score = 0;
        self.lives = PLAYER_LIVES;
        self.wave = 1;
        self.state_timer = 0.0;
        self.extra_life_awarded = false;
    }

    pub fn add_score(&mut self, points: u32) {
        self.score += points;
        if self.score > self.high_score {
            self.high_score = self.score;
        }
        // Award extra life at threshold
        if !self.extra_life_awarded && self.score >= EXTRA_LIFE_SCORE {
            self.lives += 1;
            self.extra_life_awarded = true;
        }
    }

    pub fn lose_life(&mut self) {
        if self.lives > 0 {
            self.lives -= 1;
        }
        if self.lives == 0 {
            self.state = GameState::GameOver;
        } else {
            self.state = GameState::PlayerDeath;
            self.state_timer = 0.0;
        }
    }

    pub fn complete_wave(&mut self) {
        self.state = GameState::WaveComplete;
        self.state_timer = 0.0;
        self.wave += 1;
    }

    pub fn pause(&mut self) {
        if self.state == GameState::Playing {
            self.state = GameState::Paused;
        }
    }

    pub fn unpause(&mut self) {
        if self.state == GameState::Paused {
            self.state = GameState::Playing;
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.state_timer += delta;

        match self.state {
            GameState::PlayerDeath => {
                if self.state_timer >= PLAYER_RESPAWN_DELAY {
                    self.state = GameState::Playing;
                    self.playing_state = PlayingState::Respawning;
                    self.state_timer = 0.0;
                }
            }
            GameState::WaveComplete => {
                if self.state_timer >= 2.0 {
                    self.state = GameState::Playing;
                    self.playing_state = PlayingState::Normal;
                    self.state_timer = 0.0;
                }
            }
            GameState::Playing if self.playing_state == PlayingState::Respawning => {
                if self.state_timer >= PLAYER_INVINCIBILITY_TIME {
                    self.playing_state = PlayingState::Normal;
                    self.state_timer = 0.0;
                }
            }
            _ => {}
        }
    }

    pub fn is_playing(&self) -> bool {
        self.state == GameState::Playing
    }

    pub fn can_player_be_hit(&self) -> bool {
        self.state == GameState::Playing && self.playing_state == PlayingState::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_session() {
        let session = GameSession::new();
        assert_eq!(session.state, GameState::Attract);
        assert_eq!(session.score, 0);
        assert_eq!(session.lives, PLAYER_LIVES);
        assert_eq!(session.wave, 1);
    }

    #[test]
    fn test_start_game() {
        let mut session = GameSession::new();
        session.start_game();
        assert_eq!(session.state, GameState::Playing);
        assert_eq!(session.playing_state, PlayingState::Normal);
    }

    #[test]
    fn test_add_score() {
        let mut session = GameSession::new();
        session.add_score(100);
        assert_eq!(session.score, 100);
        assert_eq!(session.high_score, 100);
    }

    #[test]
    fn test_extra_life() {
        let mut session = GameSession::new();
        session.start_game();
        let initial_lives = session.lives;
        session.add_score(EXTRA_LIFE_SCORE);
        assert_eq!(session.lives, initial_lives + 1);
        assert!(session.extra_life_awarded);
    }

    #[test]
    fn test_lose_life() {
        let mut session = GameSession::new();
        session.start_game();
        session.lose_life();
        assert_eq!(session.lives, PLAYER_LIVES - 1);
        assert_eq!(session.state, GameState::PlayerDeath);
    }

    #[test]
    fn test_game_over() {
        let mut session = GameSession::new();
        session.start_game();
        session.lives = 1;
        session.lose_life();
        assert_eq!(session.lives, 0);
        assert_eq!(session.state, GameState::GameOver);
    }

    #[test]
    fn test_wave_complete() {
        let mut session = GameSession::new();
        session.start_game();
        session.complete_wave();
        assert_eq!(session.state, GameState::WaveComplete);
        assert_eq!(session.wave, 2);
    }

    #[test]
    fn test_pause_unpause() {
        let mut session = GameSession::new();
        session.start_game();
        session.pause();
        assert_eq!(session.state, GameState::Paused);
        session.unpause();
        assert_eq!(session.state, GameState::Playing);
    }
}
