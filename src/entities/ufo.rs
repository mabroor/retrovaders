// UFO/Mystery ship entity

use macroquad::prelude::*;
use crate::entities::Entity;
use crate::game::config::*;

/// Direction the UFO travels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UfoDirection {
    Left,
    Right,
}

/// The mystery UFO that appears at the top of the screen
#[derive(Debug, Clone)]
pub struct Ufo {
    pub x: f32,
    pub y: f32,
    pub direction: UfoDirection,
    pub alive: bool,
    /// Shot count for score table lookup
    shot_index: usize,
}

// Score table from original arcade (cycles through)
const UFO_SCORE_TABLE: [u32; 15] = [
    100, 50, 50, 100, 150, 100, 100, 50, 300, 100, 100, 100, 50, 150, 100,
];

impl Ufo {
    pub fn new(direction: UfoDirection, shot_index: usize) -> Self {
        let x = match direction {
            UfoDirection::Left => GAME_WIDTH as f32,
            UfoDirection::Right => -UFO_WIDTH,
        };

        Self {
            x,
            y: UFO_Y_POSITION,
            direction,
            alive: true,
            shot_index,
        }
    }

    pub fn update(&mut self, delta: f32) {
        match self.direction {
            UfoDirection::Left => {
                self.x -= UFO_SPEED * delta;
                if self.x + UFO_WIDTH < 0.0 {
                    self.alive = false;
                }
            }
            UfoDirection::Right => {
                self.x += UFO_SPEED * delta;
                if self.x > GAME_WIDTH as f32 {
                    self.alive = false;
                }
            }
        }
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

    /// Get the score for hitting this UFO
    pub fn points(&self) -> u32 {
        UFO_SCORE_TABLE[self.shot_index % UFO_SCORE_TABLE.len()]
    }
}

impl Entity for Ufo {
    fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    fn set_position(&mut self, pos: Vec2) {
        self.x = pos.x;
        self.y = pos.y;
    }

    fn hitbox(&self) -> Rect {
        Rect::new(self.x, self.y, UFO_WIDTH, UFO_HEIGHT)
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ufo_left() {
        let ufo = Ufo::new(UfoDirection::Left, 0);
        assert_eq!(ufo.x, GAME_WIDTH as f32);
        assert_eq!(ufo.y, UFO_Y_POSITION);
        assert_eq!(ufo.direction, UfoDirection::Left);
    }

    #[test]
    fn test_new_ufo_right() {
        let ufo = Ufo::new(UfoDirection::Right, 0);
        assert_eq!(ufo.x, -UFO_WIDTH);
        assert_eq!(ufo.direction, UfoDirection::Right);
    }

    #[test]
    fn test_ufo_moves_left() {
        let mut ufo = Ufo::new(UfoDirection::Left, 0);
        let start_x = ufo.x;
        ufo.update(0.1);
        assert!(ufo.x < start_x);
    }

    #[test]
    fn test_ufo_moves_right() {
        let mut ufo = Ufo::new(UfoDirection::Right, 0);
        let start_x = ufo.x;
        ufo.update(0.1);
        assert!(ufo.x > start_x);
    }

    #[test]
    fn test_ufo_despawns() {
        let mut ufo = Ufo::new(UfoDirection::Left, 0);
        ufo.x = -UFO_WIDTH - 1.0;
        ufo.update(0.1);
        assert!(!ufo.alive);
    }

    #[test]
    fn test_ufo_points() {
        // Test score table cycling
        let ufo0 = Ufo::new(UfoDirection::Left, 0);
        assert_eq!(ufo0.points(), 100);

        let ufo8 = Ufo::new(UfoDirection::Left, 8);
        assert_eq!(ufo8.points(), 300);  // The 300 point entry

        // Test cycling
        let ufo15 = Ufo::new(UfoDirection::Left, 15);
        assert_eq!(ufo15.points(), UFO_SCORE_TABLE[0]);  // Cycles back
    }
}
