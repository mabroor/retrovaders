// Audio playback and management

use crate::game::config::*;

/// Sound effect identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundId {
    March1,
    March2,
    March3,
    March4,
    PlayerShoot,
    InvaderKilled,
    PlayerExplosion,
    Ufo,
    UfoHit,
}

/// Audio settings
#[derive(Debug, Clone)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub muted: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.8,
            music_volume: 0.7,
            sfx_volume: 0.9,
            muted: false,
        }
    }
}

/// Manages audio playback
pub struct AudioManager {
    pub settings: AudioSettings,
    march_note: usize,
    march_interval: f32,
    march_timer: f32,
    loaded: bool,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            settings: AudioSettings::default(),
            march_note: 0,
            march_interval: BASE_INVADER_MOVE_DELAY,
            march_timer: 0.0,
            loaded: false,
        }
    }

    /// Load all audio files
    pub async fn load_all(&mut self) -> Result<(), String> {
        // Audio loading will be implemented when assets are created
        self.loaded = true;
        Ok(())
    }

    /// Check if audio is loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    /// Play a sound effect
    pub fn play(&self, _sound: SoundId) {
        if self.settings.muted {
            // Nothing to do when muted
        }

        // Sound playback will be implemented with actual sound files
        // For now, this is a placeholder
    }

    /// Update the march tempo based on remaining invaders
    pub fn update_march_tempo(&mut self, remaining_invaders: usize) {
        let ratio = remaining_invaders as f32 / INVADER_TOTAL as f32;
        self.march_interval = (BASE_INVADER_MOVE_DELAY * ratio).max(MIN_INVADER_MOVE_DELAY);
    }

    /// Called when invaders move - plays the next march note
    pub fn play_march_step(&mut self) {
        if self.settings.muted {
            return;
        }

        let sound = match self.march_note {
            0 => SoundId::March1,
            1 => SoundId::March2,
            2 => SoundId::March3,
            _ => SoundId::March4,
        };

        self.play(sound);
        self.march_note = (self.march_note + 1) % 4;
    }

    /// Toggle mute
    pub fn toggle_mute(&mut self) {
        self.settings.muted = !self.settings.muted;
    }

    /// Set master volume
    pub fn set_master_volume(&mut self, volume: f32) {
        self.settings.master_volume = volume.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_manager_new() {
        let manager = AudioManager::new();
        assert!(!manager.settings.muted);
        assert_eq!(manager.march_note, 0);
    }

    #[test]
    fn test_toggle_mute() {
        let mut manager = AudioManager::new();
        assert!(!manager.settings.muted);

        manager.toggle_mute();
        assert!(manager.settings.muted);

        manager.toggle_mute();
        assert!(!manager.settings.muted);
    }

    #[test]
    fn test_march_tempo() {
        let mut manager = AudioManager::new();

        // Full grid
        manager.update_march_tempo(INVADER_TOTAL);
        let full_interval = manager.march_interval;

        // Half grid
        manager.update_march_tempo(INVADER_TOTAL / 2);
        let half_interval = manager.march_interval;

        assert!(half_interval < full_interval);
    }

    #[test]
    fn test_march_step_cycles() {
        let mut manager = AudioManager::new();

        assert_eq!(manager.march_note, 0);
        manager.play_march_step();
        assert_eq!(manager.march_note, 1);
        manager.play_march_step();
        assert_eq!(manager.march_note, 2);
        manager.play_march_step();
        assert_eq!(manager.march_note, 3);
        manager.play_march_step();
        assert_eq!(manager.march_note, 0);  // Cycles back
    }

    #[test]
    fn test_set_volume() {
        let mut manager = AudioManager::new();

        manager.set_master_volume(0.5);
        assert_eq!(manager.settings.master_volume, 0.5);

        // Clamp to bounds
        manager.set_master_volume(1.5);
        assert_eq!(manager.settings.master_volume, 1.0);

        manager.set_master_volume(-0.5);
        assert_eq!(manager.settings.master_volume, 0.0);
    }
}
