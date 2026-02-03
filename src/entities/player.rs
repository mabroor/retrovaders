// Player cannon entity

use macroquad::prelude::*;
use crate::entities::Entity;
use crate::game::config::*;

#[derive(Debug, Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub alive: bool,
    pub invincible: bool,
    pub invincibility_timer: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: PLAYER_START_X,
            y: PLAYER_START_Y,
            alive: true,
            invincible: false,
            invincibility_timer: 0.0,
        }
    }

    pub fn at(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            alive: true,
            invincible: false,
            invincibility_timer: 0.0,
        }
    }

    pub fn move_left(&mut self, delta: f32) {
        self.x = (self.x - PLAYER_SPEED * delta).max(PLAYER_MIN_X);
    }

    pub fn move_right(&mut self, delta: f32) {
        self.x = (self.x + PLAYER_SPEED * delta).min(PLAYER_MAX_X);
    }

    pub fn update(&mut self, delta: f32) {
        if self.invincible {
            self.invincibility_timer += delta;
            if self.invincibility_timer >= PLAYER_INVINCIBILITY_TIME {
                self.invincible = false;
                self.invincibility_timer = 0.0;
            }
        }
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

    pub fn respawn(&mut self) {
        self.x = PLAYER_START_X;
        self.y = PLAYER_START_Y;
        self.alive = true;
        self.invincible = true;
        self.invincibility_timer = 0.0;
    }

    pub fn can_be_hit(&self) -> bool {
        self.alive && !self.invincible
    }

    /// Get the bullet spawn position (centered above player)
    pub fn bullet_spawn_position(&self) -> Vec2 {
        Vec2::new(
            self.x + PLAYER_WIDTH / 2.0 - PLAYER_BULLET_WIDTH / 2.0,
            self.y - PLAYER_BULLET_HEIGHT,
        )
    }
}

impl Entity for Player {
    fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    fn set_position(&mut self, pos: Vec2) {
        self.x = pos.x.clamp(PLAYER_MIN_X, PLAYER_MAX_X);
        self.y = pos.y;
    }

    fn hitbox(&self) -> Rect {
        Rect::new(self.x, self.y, PLAYER_WIDTH, PLAYER_HEIGHT)
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_player() {
        let player = Player::new();
        assert_eq!(player.x, PLAYER_START_X);
        assert_eq!(player.y, PLAYER_START_Y);
        assert!(player.alive);
    }

    #[test]
    fn test_move_left() {
        let mut player = Player::new();
        let start_x = player.x;
        player.move_left(0.1);
        assert!(player.x < start_x);
    }

    #[test]
    fn test_move_right() {
        let mut player = Player::new();
        let start_x = player.x;
        player.move_right(0.1);
        assert!(player.x > start_x);
    }

    #[test]
    fn test_left_boundary() {
        let mut player = Player::at(PLAYER_MIN_X, PLAYER_START_Y);
        player.move_left(1.0);
        assert_eq!(player.x, PLAYER_MIN_X);
    }

    #[test]
    fn test_right_boundary() {
        let mut player = Player::at(PLAYER_MAX_X, PLAYER_START_Y);
        player.move_right(1.0);
        assert_eq!(player.x, PLAYER_MAX_X);
    }

    #[test]
    fn test_kill_and_respawn() {
        let mut player = Player::new();
        player.kill();
        assert!(!player.alive);
        player.respawn();
        assert!(player.alive);
        assert!(player.invincible);
    }

    #[test]
    fn test_invincibility() {
        let mut player = Player::new();
        player.respawn();
        assert!(player.invincible);
        assert!(!player.can_be_hit());

        // Update past invincibility time
        player.update(PLAYER_INVINCIBILITY_TIME + 0.1);
        assert!(!player.invincible);
        assert!(player.can_be_hit());
    }

    #[test]
    fn test_hitbox() {
        let player = Player::new();
        let hitbox = player.hitbox();
        assert_eq!(hitbox.x, PLAYER_START_X);
        assert_eq!(hitbox.y, PLAYER_START_Y);
        assert_eq!(hitbox.w, PLAYER_WIDTH);
        assert_eq!(hitbox.h, PLAYER_HEIGHT);
    }
}
