use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::playstyle::{PlayStyle, StyleCharacteristics};
use chess_trainer::ExerciseDifficulty;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum SkillLevel {
    Beginner,       // 0-1000 rating equivalent
    Intermediate,   // 1000-1500
    Advanced,       // 1500-2000
    Expert,         // 2000+
}

impl SkillLevel {
    pub fn from_rating(rating: u32) -> Self {
        match rating {
            0..=1000 => SkillLevel::Beginner,
            1001..=1500 => SkillLevel::Intermediate,
            1501..=2000 => SkillLevel::Advanced,
            _ => SkillLevel::Expert,
        }
    }

    pub fn to_difficulty(&self) -> ExerciseDifficulty {
        match self {
            SkillLevel::Beginner => ExerciseDifficulty::Beginner,
            SkillLevel::Intermediate => ExerciseDifficulty::Intermediate,
            SkillLevel::Advanced => ExerciseDifficulty::Advanced,
            SkillLevel::Expert => ExerciseDifficulty::Expert,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub user_id: u64,
    pub skill_level: SkillLevel,
    pub estimated_rating: u32,
    pub play_style: PlayStyle,
    pub style_characteristics: StyleCharacteristics,
    pub games_played: u32,
    pub exercises_completed: u32,
    pub weaknesses: Vec<String>,
    pub strengths: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PlayerProfile {
    pub fn new(user_id: u64) -> Self {
        Self {
            user_id,
            skill_level: SkillLevel::Beginner,
            estimated_rating: 800,
            play_style: PlayStyle::Balanced,
            style_characteristics: StyleCharacteristics {
                aggression_score: 0.5,
                tactical_score: 0.5,
                positional_score: 0.5,
                risk_taking_score: 0.5,
                accuracy_score: 0.5,
                primary_style: PlayStyle::Balanced,
            },
            games_played: 0,
            exercises_completed: 0,
            weaknesses: Vec::new(),
            strengths: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn update_style(&mut self, characteristics: StyleCharacteristics) {
        self.style_characteristics = characteristics.clone();
        self.play_style = characteristics.primary_style;
        self.updated_at = Utc::now();
    }

    pub fn update_weaknesses(&mut self, weaknesses: Vec<String>) {
        self.weaknesses = weaknesses;
        self.updated_at = Utc::now();
    }

    pub fn update_strengths(&mut self, strengths: Vec<String>) {
        self.strengths = strengths;
        self.updated_at = Utc::now();
    }

    pub fn increment_games_played(&mut self) {
        self.games_played += 1;
        self.updated_at = Utc::now();
    }

    pub fn increment_exercises_completed(&mut self, count: u32) {
        self.exercises_completed += count;
        self.updated_at = Utc::now();
    }

    pub fn update_rating(&mut self, new_rating: u32) {
        self.estimated_rating = new_rating;
        self.skill_level = SkillLevel::from_rating(new_rating);
        self.updated_at = Utc::now();
    }

    pub fn get_recommended_difficulty(&self) -> ExerciseDifficulty {
        self.skill_level.to_difficulty()
    }

    pub fn summary(&self) -> String {
        format!(
            "Player Profile Summary\n\
             Skill Level: {:?}\n\
             Estimated Rating: {}\n\
             Play Style: {:?}\n\
             Games Played: {}\n\
             Exercises Completed: {}\n\
             Weaknesses: {}\n\
             Strengths: {}",
            self.skill_level,
            self.estimated_rating,
            self.play_style,
            self.games_played,
            self.exercises_completed,
            self.weaknesses.join(", "),
            self.strengths.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_level_from_rating() {
        assert_eq!(SkillLevel::from_rating(800), SkillLevel::Beginner);
        assert_eq!(SkillLevel::from_rating(1200), SkillLevel::Intermediate);
        assert_eq!(SkillLevel::from_rating(1800), SkillLevel::Advanced);
        assert_eq!(SkillLevel::from_rating(2200), SkillLevel::Expert);
    }

    #[test]
    fn test_player_profile_creation() {
        let profile = PlayerProfile::new(1);
        assert_eq!(profile.user_id, 1);
        assert_eq!(profile.skill_level, SkillLevel::Beginner);
        assert_eq!(profile.games_played, 0);
    }

    #[test]
    fn test_update_rating() {
        let mut profile = PlayerProfile::new(1);
        profile.update_rating(1600);

        assert_eq!(profile.estimated_rating, 1600);
        assert_eq!(profile.skill_level, SkillLevel::Advanced);
    }
}
