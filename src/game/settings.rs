// Game settings - configurable options saved to file

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const SETTINGS_FILE: &str = "settings.json";

/// Input key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSettings {
    pub move_left: Vec<String>,
    pub move_right: Vec<String>,
    pub fire: Vec<String>,
    pub pause: Vec<String>,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            move_left: vec!["Left".to_string(), "A".to_string()],
            move_right: vec!["Right".to_string(), "D".to_string()],
            fire: vec!["Space".to_string(), "W".to_string(), "Up".to_string()],
            pause: vec!["Escape".to_string(), "P".to_string()],
        }
    }
}

/// Audio settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub muted: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 0.8,
            music_volume: 0.7,
            sfx_volume: 0.9,
            muted: false,
        }
    }
}

/// Display settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub crt_enabled: bool,
    pub scanline_weight: f32,
    pub fullscreen: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            crt_enabled: true,
            scanline_weight: 0.15,
            fullscreen: false,
        }
    }
}

/// Gameplay settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplaySettings {
    pub starting_lives: u32,
    pub difficulty: DifficultyLevel,
}

impl Default for GameplaySettings {
    fn default() -> Self {
        Self {
            starting_lives: 3,
            difficulty: DifficultyLevel::Normal,
        }
    }
}

/// Difficulty presets
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DifficultyLevel {
    Easy,
    Normal,
    Hard,
}

impl DifficultyLevel {
    pub fn invader_fire_multiplier(&self) -> f32 {
        match self {
            DifficultyLevel::Easy => 0.5,
            DifficultyLevel::Normal => 1.0,
            DifficultyLevel::Hard => 1.5,
        }
    }

    pub fn speed_multiplier(&self) -> f32 {
        match self {
            DifficultyLevel::Easy => 0.8,
            DifficultyLevel::Normal => 1.0,
            DifficultyLevel::Hard => 1.2,
        }
    }
}

/// All game settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    pub input: InputSettings,
    pub audio: AudioConfig,
    pub display: DisplaySettings,
    pub gameplay: GameplaySettings,
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the path to the settings file
    fn get_path() -> PathBuf {
        if let Some(data_dir) = dirs::data_local_dir() {
            let game_dir = data_dir.join("retrovaders");
            if fs::create_dir_all(&game_dir).is_ok() {
                return game_dir.join(SETTINGS_FILE);
            }
        }
        PathBuf::from(SETTINGS_FILE)
    }

    /// Load settings from file, or create default if not found
    pub fn load() -> Self {
        let path = Self::get_path();
        if let Ok(data) = fs::read_to_string(&path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            let settings = Self::default();
            let _ = settings.save();  // Create file with defaults
            settings
        }
    }

    /// Save settings to file
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::get_path();
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert!(settings.display.crt_enabled);
        assert_eq!(settings.gameplay.starting_lives, 3);
        assert_eq!(settings.gameplay.difficulty, DifficultyLevel::Normal);
    }

    #[test]
    fn test_difficulty_multipliers() {
        assert!(DifficultyLevel::Easy.invader_fire_multiplier() < 1.0);
        assert_eq!(DifficultyLevel::Normal.invader_fire_multiplier(), 1.0);
        assert!(DifficultyLevel::Hard.invader_fire_multiplier() > 1.0);
    }

    #[test]
    fn test_input_settings() {
        let input = InputSettings::default();
        assert!(!input.move_left.is_empty());
        assert!(!input.fire.is_empty());
    }

    #[test]
    fn test_settings_serialize() {
        let settings = Settings::default();
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("crt_enabled"));
        assert!(json.contains("starting_lives"));
    }
}
