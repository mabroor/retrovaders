// Bullet entities for player and invader projectiles

use macroquad::prelude::*;
use crate::entities::Entity;
use crate::game::config::*;

/// Owner of a bullet
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulletOwner {
    Player,
    Invader,
}

/// Bullet entity
#[derive(Debug, Clone)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub owner: BulletOwner,
    pub alive: bool,
}

impl Bullet {
    pub fn new(x: f32, y: f32, owner: BulletOwner) -> Self {
        Self {
            x,
            y,
            owner,
            alive: true,
        }
    }

    pub fn player_bullet(x: f32, y: f32) -> Self {
        Self::new(x, y, BulletOwner::Player)
    }

    pub fn invader_bullet(x: f32, y: f32) -> Self {
        Self::new(x, y, BulletOwner::Invader)
    }

    pub fn update(&mut self, delta: f32) {
        match self.owner {
            BulletOwner::Player => {
                self.y -= PLAYER_BULLET_SPEED * delta;
                if self.y + self.height() < 0.0 {
                    self.alive = false;
                }
            }
            BulletOwner::Invader => {
                self.y += INVADER_BULLET_SPEED * delta;
                if self.y > GAME_HEIGHT as f32 {
                    self.alive = false;
                }
            }
        }
    }

    pub fn width(&self) -> f32 {
        match self.owner {
            BulletOwner::Player => PLAYER_BULLET_WIDTH,
            BulletOwner::Invader => INVADER_BULLET_WIDTH,
        }
    }

    pub fn height(&self) -> f32 {
        match self.owner {
            BulletOwner::Player => PLAYER_BULLET_HEIGHT,
            BulletOwner::Invader => INVADER_BULLET_HEIGHT,
        }
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }
}

impl Entity for Bullet {
    fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    fn set_position(&mut self, pos: Vec2) {
        self.x = pos.x;
        self.y = pos.y;
    }

    fn hitbox(&self) -> Rect {
        Rect::new(self.x, self.y, self.width(), self.height())
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_bullet() {
        let bullet = Bullet::player_bullet(100.0, 200.0);
        assert_eq!(bullet.owner, BulletOwner::Player);
        assert_eq!(bullet.width(), PLAYER_BULLET_WIDTH);
        assert_eq!(bullet.height(), PLAYER_BULLET_HEIGHT);
    }

    #[test]
    fn test_invader_bullet() {
        let bullet = Bullet::invader_bullet(100.0, 50.0);
        assert_eq!(bullet.owner, BulletOwner::Invader);
        assert_eq!(bullet.width(), INVADER_BULLET_WIDTH);
        assert_eq!(bullet.height(), INVADER_BULLET_HEIGHT);
    }

    #[test]
    fn test_player_bullet_moves_up() {
        let mut bullet = Bullet::player_bullet(100.0, 200.0);
        let start_y = bullet.y;
        bullet.update(0.1);
        assert!(bullet.y < start_y);
    }

    #[test]
    fn test_invader_bullet_moves_down() {
        let mut bullet = Bullet::invader_bullet(100.0, 50.0);
        let start_y = bullet.y;
        bullet.update(0.1);
        assert!(bullet.y > start_y);
    }

    #[test]
    fn test_player_bullet_despawns_at_top() {
        let mut bullet = Bullet::player_bullet(100.0, 5.0);
        bullet.update(0.1);
        assert!(!bullet.alive);
    }

    #[test]
    fn test_invader_bullet_despawns_at_bottom() {
        let mut bullet = Bullet::invader_bullet(100.0, GAME_HEIGHT as f32 - 5.0);
        bullet.update(0.1);
        assert!(!bullet.alive);
    }

    #[test]
    fn test_hitbox() {
        let bullet = Bullet::player_bullet(100.0, 200.0);
        let hitbox = bullet.hitbox();
        assert_eq!(hitbox.x, 100.0);
        assert_eq!(hitbox.y, 200.0);
        assert_eq!(hitbox.w, PLAYER_BULLET_WIDTH);
        assert_eq!(hitbox.h, PLAYER_BULLET_HEIGHT);
    }
}
