// Scoring system - tracks scores and high scores

use serde::{Deserialize, Serialize};
use std::fs;

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

    pub fn load() -> Self {
        if let Ok(data) = fs::read_to_string(HIGH_SCORE_FILE) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::new()
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(HIGH_SCORE_FILE, data)
    }

    pub fn top_score(&self) -> u32 {
        self.entries.first().map(|e| e.score).unwrap_or(0)
    }

    pub fn is_high_score(&self, score: u32) -> bool {
        if self.entries.len() < MAX_HIGH_SCORES {
            return score > 0;
        }
        self.entries.last().map(|e| score > e.score).unwrap_or(true)
    }

    pub fn add_score(&mut self, name: String, score: u32, wave_reached: u32) {
        let date = chrono_lite_date();

        let entry = HighScoreEntry {
            rank: 0,  // Will be updated
            name,
            score,
            wave_reached,
            date,
        };

        // Insert in sorted position
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
    }
}

/// Simple date string without external crate
fn chrono_lite_date() -> String {
    // In a real impl, would use chrono crate
    // For now, return placeholder
    "2026-02-03".to_string()
}

/// Calculate score for invader type
pub fn score_for_invader(invader_type: crate::entities::InvaderType) -> u32 {
    invader_type.points()
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
}
