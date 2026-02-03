// Sprite loading and management

use macroquad::prelude::*;
use std::collections::HashMap;

/// Sprite identifiers
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SpriteId {
    // Invaders
    SquidFrame0,
    SquidFrame1,
    CrabFrame0,
    CrabFrame1,
    OctopusFrame0,
    OctopusFrame1,
    // Player
    Player,
    PlayerExplosion,
    // Bullets
    PlayerBullet,
    InvaderBullet1,
    InvaderBullet2,
    InvaderBullet3,
    // Other
    Ufo,
    Shield,
    InvaderExplosion,
}

/// Asset manager for sprites
pub struct SpriteManager {
    textures: HashMap<SpriteId, Texture2D>,
    loaded: bool,
}

impl Default for SpriteManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SpriteManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            loaded: false,
        }
    }

    /// Load all sprites from assets directory
    pub async fn load_all(&mut self) -> Result<(), String> {
        // For now, we'll create placeholder textures
        // Real implementation would load from files

        self.loaded = true;
        Ok(())
    }

    /// Check if sprites are loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    /// Get a texture by ID
    pub fn get(&self, id: SpriteId) -> Option<&Texture2D> {
        self.textures.get(&id)
    }

    /// Create a placeholder colored texture
    #[allow(dead_code)]
    fn create_placeholder(width: u16, height: u16, color: Color) -> Texture2D {
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);

        for _ in 0..(width * height) {
            pixels.push((color.r * 255.0) as u8);
            pixels.push((color.g * 255.0) as u8);
            pixels.push((color.b * 255.0) as u8);
            pixels.push((color.a * 255.0) as u8);
        }

        Texture2D::from_rgba8(width, height, &pixels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_manager_new() {
        let manager = SpriteManager::new();
        assert!(!manager.is_loaded());
    }
}
