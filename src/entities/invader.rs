// Invader entities and grid management

use macroquad::prelude::*;
use crate::entities::Entity;
use crate::game::config::*;

/// The three types of invaders with different point values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvaderType {
    Squid,   // Top row, 30 points
    Crab,    // Middle rows, 20 points
    Octopus, // Bottom rows, 10 points
}

impl InvaderType {
    pub fn points(&self) -> u32 {
        match self {
            InvaderType::Squid => SCORE_SQUID,
            InvaderType::Crab => SCORE_CRAB,
            InvaderType::Octopus => SCORE_OCTOPUS,
        }
    }

    pub fn for_row(row: usize) -> Self {
        match row {
            0 => InvaderType::Squid,
            1 | 2 => InvaderType::Crab,
            _ => InvaderType::Octopus,
        }
    }
}

/// Individual invader entity
#[derive(Debug, Clone)]
pub struct Invader {
    pub x: f32,
    pub y: f32,
    pub invader_type: InvaderType,
    pub alive: bool,
    pub animation_frame: u8,
}

impl Invader {
    pub fn new(x: f32, y: f32, invader_type: InvaderType) -> Self {
        Self {
            x,
            y,
            invader_type,
            alive: true,
            animation_frame: 0,
        }
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

    pub fn toggle_animation(&mut self) {
        self.animation_frame = 1 - self.animation_frame;
    }

    pub fn points(&self) -> u32 {
        self.invader_type.points()
    }

    /// Get the bullet spawn position (centered below invader)
    pub fn bullet_spawn_position(&self) -> Vec2 {
        Vec2::new(
            self.x + INVADER_WIDTH / 2.0 - INVADER_BULLET_WIDTH / 2.0,
            self.y + INVADER_HEIGHT,
        )
    }
}

impl Entity for Invader {
    fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    fn set_position(&mut self, pos: Vec2) {
        self.x = pos.x;
        self.y = pos.y;
    }

    fn hitbox(&self) -> Rect {
        Rect::new(self.x, self.y, INVADER_WIDTH, INVADER_HEIGHT)
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

/// Movement direction for the invader grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveDirection {
    Left,
    Right,
}

impl MoveDirection {
    pub fn reverse(&self) -> Self {
        match self {
            MoveDirection::Left => MoveDirection::Right,
            MoveDirection::Right => MoveDirection::Left,
        }
    }
}

/// The 5x11 grid of invaders
#[derive(Debug, Clone)]
pub struct InvaderGrid {
    pub invaders: Vec<Vec<Invader>>,
    pub direction: MoveDirection,
    pub move_timer: f32,
    pub move_delay: f32,
    pub needs_drop: bool,
    pub wave: u32,
    pub base_move_delay: f32,
}

impl Default for InvaderGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl InvaderGrid {
    pub fn new() -> Self {
        Self::new_at_wave(1)
    }

    pub fn new_at_wave(wave: u32) -> Self {
        let mut invaders = Vec::with_capacity(INVADER_ROWS);

        // Calculate starting Y position based on wave
        let wave_offset = ((wave.min(MAX_DIFFICULTY_WAVE) - 1) as f32) * WAVE_START_Y_INCREASE;
        let start_y = INVADER_START_Y + wave_offset;

        for row in 0..INVADER_ROWS {
            let mut row_invaders = Vec::with_capacity(INVADER_COLS);
            let invader_type = InvaderType::for_row(row);

            for col in 0..INVADER_COLS {
                let x = INVADER_START_X + (col as f32) * INVADER_SPACING_X;
                let y = start_y + (row as f32) * INVADER_SPACING_Y;
                row_invaders.push(Invader::new(x, y, invader_type));
            }
            invaders.push(row_invaders);
        }

        // Calculate base delay with wave speed multiplier
        let speed_multiplier = 1.0 + ((wave.min(MAX_DIFFICULTY_WAVE) - 1) as f32) * WAVE_SPEED_MULTIPLIER_INCREASE;
        let base_move_delay = BASE_INVADER_MOVE_DELAY / speed_multiplier;

        Self {
            invaders,
            direction: MoveDirection::Right,
            move_timer: 0.0,
            move_delay: base_move_delay,
            needs_drop: false,
            wave,
            base_move_delay,
        }
    }

    pub fn alive_count(&self) -> usize {
        self.invaders
            .iter()
            .flat_map(|row| row.iter())
            .filter(|inv| inv.alive)
            .count()
    }

    /// Calculate move delay based on remaining invaders
    pub fn calculate_move_delay(&self) -> f32 {
        let alive = self.alive_count() as f32;
        let ratio = alive / INVADER_TOTAL as f32;
        (self.base_move_delay * ratio).max(MIN_INVADER_MOVE_DELAY)
    }

    /// Update move delay based on current alive count
    pub fn update_move_delay(&mut self) {
        self.move_delay = self.calculate_move_delay();
    }

    /// Check if any alive invader has reached a screen edge
    pub fn at_edge(&self) -> bool {
        for row in &self.invaders {
            for invader in row {
                if !invader.alive {
                    continue;
                }
                match self.direction {
                    MoveDirection::Right => {
                        if invader.x + INVADER_WIDTH + INVADER_STEP_PIXELS >= GAME_WIDTH as f32 - 8.0 {
                            return true;
                        }
                    }
                    MoveDirection::Left => {
                        if invader.x - INVADER_STEP_PIXELS <= 8.0 {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Move all invaders one step
    pub fn move_step(&mut self) {
        if self.needs_drop {
            // Drop down
            for row in &mut self.invaders {
                for invader in row {
                    if invader.alive {
                        invader.y += INVADER_DROP_PIXELS;
                        invader.toggle_animation();
                    }
                }
            }
            self.direction = self.direction.reverse();
            self.needs_drop = false;
        } else {
            // Check if we'll hit edge
            if self.at_edge() {
                self.needs_drop = true;
            }

            // Move horizontally
            let dx = match self.direction {
                MoveDirection::Right => INVADER_STEP_PIXELS,
                MoveDirection::Left => -INVADER_STEP_PIXELS,
            };

            for row in &mut self.invaders {
                for invader in row {
                    if invader.alive {
                        invader.x += dx;
                        invader.toggle_animation();
                    }
                }
            }
        }

        // Update move delay based on remaining invaders
        self.update_move_delay();
    }

    /// Update the grid, returns true if it's time to move
    pub fn update(&mut self, delta: f32) -> bool {
        self.move_timer += delta;
        if self.move_timer >= self.move_delay {
            self.move_timer = 0.0;
            true
        } else {
            false
        }
    }

    /// Check if invaders have reached the bottom (game over condition)
    pub fn has_reached_bottom(&self) -> bool {
        for row in &self.invaders {
            for invader in row {
                if invader.alive && invader.y + INVADER_HEIGHT >= ZONE_SHIELD_Y_START {
                    return true;
                }
            }
        }
        false
    }

    /// Get the bottom-most alive invaders per column (for shooting)
    pub fn bottom_invaders(&self) -> Vec<&Invader> {
        let mut result = Vec::new();

        for col in 0..INVADER_COLS {
            // Search from bottom row upward
            for row in (0..INVADER_ROWS).rev() {
                if self.invaders[row][col].alive {
                    result.push(&self.invaders[row][col]);
                    break;
                }
            }
        }

        result
    }

    /// Kill an invader at the given position, returns points if hit
    pub fn kill_at(&mut self, row: usize, col: usize) -> Option<u32> {
        if row < INVADER_ROWS && col < INVADER_COLS && self.invaders[row][col].alive {
            let points = self.invaders[row][col].points();
            self.invaders[row][col].kill();
            self.update_move_delay();
            Some(points)
        } else {
            None
        }
    }

    /// Find invader at position
    pub fn find_invader_at(&self, rect: &Rect) -> Option<(usize, usize)> {
        for (row_idx, row) in self.invaders.iter().enumerate() {
            for (col_idx, invader) in row.iter().enumerate() {
                if invader.alive {
                    let hitbox = invader.hitbox();
                    if crate::entities::check_collision(&hitbox, rect) {
                        return Some((row_idx, col_idx));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invader_type_points() {
        assert_eq!(InvaderType::Squid.points(), SCORE_SQUID);
        assert_eq!(InvaderType::Crab.points(), SCORE_CRAB);
        assert_eq!(InvaderType::Octopus.points(), SCORE_OCTOPUS);
    }

    #[test]
    fn test_invader_type_for_row() {
        assert_eq!(InvaderType::for_row(0), InvaderType::Squid);
        assert_eq!(InvaderType::for_row(1), InvaderType::Crab);
        assert_eq!(InvaderType::for_row(2), InvaderType::Crab);
        assert_eq!(InvaderType::for_row(3), InvaderType::Octopus);
        assert_eq!(InvaderType::for_row(4), InvaderType::Octopus);
    }

    #[test]
    fn test_grid_creation() {
        let grid = InvaderGrid::new();
        assert_eq!(grid.invaders.len(), INVADER_ROWS);
        assert_eq!(grid.invaders[0].len(), INVADER_COLS);
        assert_eq!(grid.alive_count(), INVADER_TOTAL);
    }

    #[test]
    fn test_grid_initial_direction() {
        let grid = InvaderGrid::new();
        assert_eq!(grid.direction, MoveDirection::Right);
    }

    #[test]
    fn test_direction_reverse() {
        assert_eq!(MoveDirection::Left.reverse(), MoveDirection::Right);
        assert_eq!(MoveDirection::Right.reverse(), MoveDirection::Left);
    }

    #[test]
    fn test_kill_invader() {
        let mut grid = InvaderGrid::new();
        let points = grid.kill_at(0, 0);
        assert_eq!(points, Some(SCORE_SQUID));
        assert_eq!(grid.alive_count(), INVADER_TOTAL - 1);
    }

    #[test]
    fn test_speed_acceleration() {
        let mut grid = InvaderGrid::new();
        let initial_delay = grid.calculate_move_delay();

        // Kill half the invaders
        for row in 0..INVADER_ROWS {
            for col in 0..INVADER_COLS / 2 {
                grid.kill_at(row, col);
            }
        }

        let new_delay = grid.calculate_move_delay();
        assert!(new_delay < initial_delay);
    }

    #[test]
    fn test_bottom_invaders() {
        let grid = InvaderGrid::new();
        let bottom = grid.bottom_invaders();
        assert_eq!(bottom.len(), INVADER_COLS);
        // All should be from bottom row (row 4)
        for inv in bottom {
            assert_eq!(inv.invader_type, InvaderType::Octopus);
        }
    }

    #[test]
    fn test_animation_toggle() {
        let mut invader = Invader::new(0.0, 0.0, InvaderType::Squid);
        assert_eq!(invader.animation_frame, 0);
        invader.toggle_animation();
        assert_eq!(invader.animation_frame, 1);
        invader.toggle_animation();
        assert_eq!(invader.animation_frame, 0);
    }
}
