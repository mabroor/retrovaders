// Shield/bunker entities with pixel-based erosion

use macroquad::prelude::*;
use crate::entities::Entity;
use crate::game::config::*;

/// A destructible shield/bunker
#[derive(Debug, Clone)]
pub struct Shield {
    pub x: f32,
    pub y: f32,
    /// Pixel data for erosion (true = solid, false = destroyed)
    pub pixels: Vec<Vec<bool>>,
}

impl Shield {
    pub fn new(x: f32, y: f32) -> Self {
        // Create full shield pixel data
        let width = SHIELD_WIDTH as usize;
        let height = SHIELD_HEIGHT as usize;

        // Classic shield shape - arc top, notch at bottom
        let mut pixels = vec![vec![false; width]; height];

        for (row, pixel_row) in pixels.iter_mut().enumerate() {
            for (col, pixel) in pixel_row.iter_mut().enumerate() {
                // Calculate if this pixel should be solid
                *pixel = Self::is_shield_pixel(row, col, width, height);
            }
        }

        Self { x, y, pixels }
    }

    /// Determine if a position is part of the classic shield shape
    fn is_shield_pixel(row: usize, col: usize, width: usize, height: usize) -> bool {
        // Top curved section (first 4 rows)
        if row < 4 {
            let center = width / 2;
            let dist_from_center = (col as i32 - center as i32).unsigned_abs() as usize;
            let curve_offset = 4 - row; // More curve at top
            return dist_from_center < center - curve_offset;
        }

        // Middle section (rows 4 to height-5) - full width
        if row < height - 5 {
            return true;
        }

        // Bottom section with notch (last 5 rows)
        let notch_width = 6;
        let notch_start = (width - notch_width) / 2;
        let notch_end = notch_start + notch_width;

        // If in notch area, not solid
        if col >= notch_start && col < notch_end {
            return false;
        }

        true
    }

    /// Apply damage at a point, eroding nearby pixels
    pub fn damage_at(&mut self, hit_x: f32, hit_y: f32, from_above: bool) {
        // Convert world coordinates to local pixel coordinates
        let local_x = (hit_x - self.x) as i32;
        let local_y = (hit_y - self.y) as i32;

        let radius = SHIELD_DAMAGE_RADIUS as i32;
        let width = SHIELD_WIDTH as i32;
        let height = SHIELD_HEIGHT as i32;

        // Damage in a circular-ish pattern
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                // Bias erosion in the direction of the bullet
                let adjusted_dy = if from_above {
                    dy + 1  // Shift damage pattern down for bullets from above
                } else {
                    dy - 1  // Shift damage pattern up for bullets from below
                };

                let px = local_x + dx;
                let py = local_y + adjusted_dy;

                if px >= 0 && px < width && py >= 0 && py < height {
                    // Random-ish erosion based on distance
                    let dist_sq = dx * dx + dy * dy;
                    if dist_sq <= radius * radius {
                        self.pixels[py as usize][px as usize] = false;
                    }
                }
            }
        }
    }

    /// Check if any solid pixel overlaps with the given rectangle
    pub fn check_hit(&self, rect: &Rect) -> Option<(f32, f32)> {
        let local_left = (rect.x - self.x).max(0.0) as usize;
        let local_top = (rect.y - self.y).max(0.0) as usize;
        let local_right = ((rect.x + rect.w - self.x) as usize).min(SHIELD_WIDTH as usize);
        let local_bottom = ((rect.y + rect.h - self.y) as usize).min(SHIELD_HEIGHT as usize);

        for row in local_top..local_bottom {
            for col in local_left..local_right {
                if row < self.pixels.len() && col < self.pixels[row].len() && self.pixels[row][col] {
                    // Return world position of hit pixel
                    return Some((self.x + col as f32, self.y + row as f32));
                }
            }
        }

        None
    }

    /// Count remaining solid pixels
    pub fn solid_count(&self) -> usize {
        self.pixels
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&p| p)
            .count()
    }

    /// Check if shield is completely destroyed
    pub fn is_destroyed(&self) -> bool {
        self.solid_count() == 0
    }
}

impl Entity for Shield {
    fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    fn set_position(&mut self, pos: Vec2) {
        self.x = pos.x;
        self.y = pos.y;
    }

    fn hitbox(&self) -> Rect {
        Rect::new(self.x, self.y, SHIELD_WIDTH, SHIELD_HEIGHT)
    }

    fn is_alive(&self) -> bool {
        !self.is_destroyed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_shield() {
        let shield = Shield::new(32.0, 192.0);
        assert_eq!(shield.x, 32.0);
        assert_eq!(shield.y, 192.0);
        assert!(shield.solid_count() > 0);
    }

    #[test]
    fn test_shield_shape() {
        let shield = Shield::new(0.0, 0.0);
        // Check that notch at bottom exists
        let width = SHIELD_WIDTH as usize;
        let height = SHIELD_HEIGHT as usize;
        let notch_center = width / 2;

        // Bottom row, center should be empty (notch)
        assert!(!shield.pixels[height - 1][notch_center]);

        // Top rows should have curved edges (corners empty)
        assert!(!shield.pixels[0][0]);
        assert!(!shield.pixels[0][width - 1]);
    }

    #[test]
    fn test_damage() {
        let mut shield = Shield::new(0.0, 0.0);
        let initial_count = shield.solid_count();

        // Damage center
        shield.damage_at(SHIELD_WIDTH / 2.0, SHIELD_HEIGHT / 2.0, true);

        assert!(shield.solid_count() < initial_count);
    }

    #[test]
    fn test_check_hit() {
        let shield = Shield::new(0.0, 0.0);

        // Create a rect overlapping the shield's solid area
        let rect = Rect::new(SHIELD_WIDTH / 2.0 - 1.0, SHIELD_HEIGHT / 2.0, 2.0, 2.0);
        let hit = shield.check_hit(&rect);

        assert!(hit.is_some());
    }

    #[test]
    fn test_check_hit_miss() {
        let shield = Shield::new(100.0, 100.0);

        // Create a rect not overlapping
        let rect = Rect::new(0.0, 0.0, 5.0, 5.0);
        let hit = shield.check_hit(&rect);

        assert!(hit.is_none());
    }

    #[test]
    fn test_is_destroyed() {
        let mut shield = Shield::new(0.0, 0.0);
        assert!(!shield.is_destroyed());

        // Clear all pixels
        for row in &mut shield.pixels {
            for pixel in row {
                *pixel = false;
            }
        }

        assert!(shield.is_destroyed());
    }
}
