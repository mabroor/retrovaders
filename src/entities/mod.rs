// Entities module - game objects

mod player;
mod invader;
mod bullet;
mod shield;
mod ufo;

pub use player::Player;
pub use invader::{Invader, InvaderGrid, InvaderType};
pub use bullet::{Bullet, BulletOwner};
pub use shield::Shield;
pub use ufo::{Ufo, UfoDirection};

use macroquad::prelude::*;

/// Common trait for entities with position and hitbox
pub trait Entity {
    fn position(&self) -> Vec2;
    fn set_position(&mut self, pos: Vec2);
    fn hitbox(&self) -> Rect;
    fn is_alive(&self) -> bool;
}

/// Basic rectangle collision check
pub fn check_collision(a: &Rect, b: &Rect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_overlap() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(5.0, 5.0, 10.0, 10.0);
        assert!(check_collision(&a, &b));
    }

    #[test]
    fn test_collision_no_overlap() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(20.0, 20.0, 10.0, 10.0);
        assert!(!check_collision(&a, &b));
    }

    #[test]
    fn test_collision_touching() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(10.0, 0.0, 10.0, 10.0);
        assert!(!check_collision(&a, &b)); // Just touching, not overlapping
    }
}
