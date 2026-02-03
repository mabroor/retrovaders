// Scoring system - tracks scores and high scores with persistence

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const HIGH_SCORE_FILE: &str = "high_scores.json";
const MAX_HIGH_SCORES: usize = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighScoreEntry {
    pub rank: usize,
    pub name: String,
    pub score: u32,
    pub wave_reached: u32,
    pub date: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HighScores {
    pub entries: Vec<HighScoreEntry>,
}

impl HighScores {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Get the path to the high scores file
    fn get_path() -> PathBuf {
        // Try to get user's data directory, fall back to current directory
        if let Some(data_dir) = dirs::data_local_dir() {
            let game_dir = data_dir.join("retrovaders");
            if fs::create_dir_all(&game_dir).is_ok() {
                return game_dir.join(HIGH_SCORE_FILE);
            }
        }
        PathBuf::from(HIGH_SCORE_FILE)
    }

    /// Load high scores from file
    pub fn load() -> Self {
        let path = Self::get_path();
        if let Ok(data) = fs::read_to_string(&path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::new()
        }
    }

    /// Save high scores to file
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::get_path();
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)
    }

    /// Get the top score
    pub fn top_score(&self) -> u32 {
        self.entries.first().map(|e| e.score).unwrap_or(0)
    }

    /// Check if a score qualifies as a high score
    pub fn is_high_score(&self, score: u32) -> bool {
        if score == 0 {
            return false;
        }
        if self.entries.len() < MAX_HIGH_SCORES {
            return true;
        }
        self.entries.last().map(|e| score > e.score).unwrap_or(true)
    }

    /// Add a new high score entry
    pub fn add_score(&mut self, name: String, score: u32, wave_reached: u32) {
        let date = get_current_date();

        let entry = HighScoreEntry {
            rank: 0,  // Will be updated
            name,
            score,
            wave_reached,
            date,
        };

        // Insert in sorted position (highest first)
        let pos = self.entries
            .iter()
            .position(|e| score > e.score)
            .unwrap_or(self.entries.len());

        self.entries.insert(pos, entry);

        // Trim to max entries
        self.entries.truncate(MAX_HIGH_SCORES);

        // Update ranks
        for (i, entry) in self.entries.iter_mut().enumerate() {
            entry.rank = i + 1;
        }

        // Auto-save after adding
        let _ = self.save();
    }

    /// Get a formatted string of all high scores for display
    pub fn format_for_display(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|e| format!("{:2}. {:3} {:>7} W{}", e.rank, e.name, e.score, e.wave_reached))
            .collect()
    }
}

/// Get current date as string (without external chrono dependency)
fn get_current_date() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let secs = duration.as_secs();

    // Simple date calculation (approximate)
    let days = secs / 86400;
    let years = days / 365;
    let year = 1970 + years;

    let remaining_days = days % 365;
    let month = (remaining_days / 30).min(11) + 1;
    let day = (remaining_days % 30) + 1;

    format!("{year:04}-{month:02}-{day:02}")
}

/// Calculate score for invader type
pub fn score_for_invader(invader_type: crate::entities::InvaderType) -> u32 {
    invader_type.points()
}

/// Score manager for the current game session
#[derive(Debug, Default)]
pub struct ScoreManager {
    pub high_scores: HighScores,
}

impl ScoreManager {
    pub fn new() -> Self {
        Self {
            high_scores: HighScores::load(),
        }
    }

    pub fn get_top_score(&self) -> u32 {
        self.high_scores.top_score()
    }

    pub fn check_and_add_score(&mut self, name: &str, score: u32, wave: u32) -> bool {
        if self.high_scores.is_high_score(score) {
            self.high_scores.add_score(name.to_string(), score, wave);
            true
        } else {
            false
        }
    }

    pub fn reload(&mut self) {
        self.high_scores = HighScores::load();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::InvaderType;
    use crate::game::config::*;

    #[test]
    fn test_score_for_invader() {
        assert_eq!(score_for_invader(InvaderType::Squid), SCORE_SQUID);
        assert_eq!(score_for_invader(InvaderType::Crab), SCORE_CRAB);
        assert_eq!(score_for_invader(InvaderType::Octopus), SCORE_OCTOPUS);
    }

    #[test]
    fn test_high_scores_empty() {
        let scores = HighScores::new();
        assert_eq!(scores.top_score(), 0);
        assert!(scores.is_high_score(100));
        assert!(!scores.is_high_score(0));
    }

    #[test]
    fn test_add_high_score() {
        let mut scores = HighScores::new();
        scores.add_score("AAA".to_string(), 1000, 1);

        assert_eq!(scores.entries.len(), 1);
        assert_eq!(scores.entries[0].score, 1000);
        assert_eq!(scores.entries[0].rank, 1);
    }

    #[test]
    fn test_high_score_ordering() {
        let mut scores = HighScores::new();
        scores.add_score("BBB".to_string(), 500, 1);
        scores.add_score("AAA".to_string(), 1000, 2);
        scores.add_score("CCC".to_string(), 750, 1);

        assert_eq!(scores.entries[0].name, "AAA");
        assert_eq!(scores.entries[1].name, "CCC");
        assert_eq!(scores.entries[2].name, "BBB");
    }

    #[test]
    fn test_high_score_limit() {
        let mut scores = HighScores::new();

        for i in 0..15 {
            scores.add_score(format!("P{:02}", i), i * 100, 1);
        }

        assert_eq!(scores.entries.len(), MAX_HIGH_SCORES);
    }

    #[test]
    fn test_is_high_score() {
        let mut scores = HighScores::new();

        // Fill with scores
        for i in 1..=10 {
            scores.add_score(format!("P{}", i), i * 100, 1);
        }

        // Score higher than lowest should qualify
        assert!(scores.is_high_score(150));

        // Score lower than lowest should not qualify
        assert!(!scores.is_high_score(50));
    }

    #[test]
    fn test_format_for_display() {
        let mut scores = HighScores::new();
        scores.add_score("ABC".to_string(), 5000, 3);
        scores.add_score("XYZ".to_string(), 3000, 2);

        let display = scores.format_for_display();
        assert_eq!(display.len(), 2);
        assert!(display[0].contains("ABC"));
        assert!(display[0].contains("5000"));
    }

    #[test]
    fn test_current_date() {
        let date = get_current_date();
        assert!(date.len() == 10);  // YYYY-MM-DD format
        assert!(date.contains('-'));
    }
}
