use chess_core::ChessGame;
use chess_engine::GameAnalyzer;
use chess_trainer::{TrainingSession, ExerciseDifficulty};
use crate::playstyle::{PlayStyleAnalyzer, StyleCharacteristics};
use crate::profile::PlayerProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRecommendation {
    pub weaknesses_identified: Vec<String>,
    pub strengths_identified: Vec<String>,
    pub recommended_exercises: usize,
    pub recommended_difficulty: ExerciseDifficulty,
    pub personalized_message: String,
    pub focus_areas: Vec<String>,
}

pub struct LearningAgent {
    profile: PlayerProfile,
    game_history: Vec<ChessGame>,
}

impl LearningAgent {
    pub fn new(user_id: u64) -> Self {
        Self {
            profile: PlayerProfile::new(user_id),
            game_history: Vec::new(),
        }
    }

    pub fn from_profile(profile: PlayerProfile) -> Self {
        Self {
            profile,
            game_history: Vec::new(),
        }
    }

    pub fn get_profile(&self) -> &PlayerProfile {
        &self.profile
    }

    pub fn get_profile_mut(&mut self) -> &mut PlayerProfile {
        &mut self.profile
    }

    pub fn analyze_game(&mut self, game: ChessGame) -> AgentRecommendation {
        // Add game to history
        self.game_history.push(game.clone());
        self.profile.increment_games_played();

        // Analyze the game
        let analyses = GameAnalyzer::analyze_game(&game);
        let weaknesses = GameAnalyzer::identify_weaknesses(&analyses);

        // Analyze play style
        let style_chars = PlayStyleAnalyzer::analyze_game(&game);
        self.profile.update_style(style_chars.clone());

        // Update profile with weaknesses
        self.profile.update_weaknesses(weaknesses.clone());

        // Identify strengths
        let strengths = self.identify_strengths(&analyses);
        self.profile.update_strengths(strengths.clone());

        // Generate recommendations
        self.generate_recommendations(weaknesses, strengths)
    }

    pub fn analyze_multiple_games(&mut self, games: Vec<ChessGame>) -> AgentRecommendation {
        // Add games to history
        for game in &games {
            self.game_history.push(game.clone());
            self.profile.increment_games_played();
        }

        // Analyze all games
        let mut all_weaknesses = Vec::new();
        let mut all_strengths = Vec::new();

        for game in &games {
            let analyses = GameAnalyzer::analyze_game(game);
            let weaknesses = GameAnalyzer::identify_weaknesses(&analyses);
            all_weaknesses.extend(weaknesses);

            let strengths = self.identify_strengths(&analyses);
            all_strengths.extend(strengths);
        }

        // Analyze aggregate play style
        let style_chars = PlayStyleAnalyzer::analyze_multiple_games(&games);
        self.profile.update_style(style_chars);

        // Deduplicate and prioritize weaknesses
        all_weaknesses.sort();
        all_weaknesses.dedup();
        all_strengths.sort();
        all_strengths.dedup();

        self.profile.update_weaknesses(all_weaknesses.clone());
        self.profile.update_strengths(all_strengths.clone());

        self.generate_recommendations(all_weaknesses, all_strengths)
    }

    fn identify_strengths(&self, analyses: &[chess_engine::MoveAnalysis]) -> Vec<String> {
        let mut strengths = Vec::new();

        if analyses.is_empty() {
            return strengths;
        }

        let brilliant_moves = analyses
            .iter()
            .filter(|a| a.quality == chess_core::MoveQuality::Brilliant)
            .count();

        let great_moves = analyses
            .iter()
            .filter(|a| a.quality == chess_core::MoveQuality::Great)
            .count();

        let avg_loss: i32 = analyses.iter().map(|a| a.centipawn_loss).sum::<i32>()
            / analyses.len().max(1) as i32;

        if brilliant_moves > analyses.len() / 10 {
            strengths.push("Excellent tactical vision".to_string());
        }

        if great_moves > analyses.len() / 5 {
            strengths.push("Strong move selection".to_string());
        }

        if avg_loss < 50 {
            strengths.push("High accuracy".to_string());
        }

        // Analyze opening moves (first 10)
        let opening_moves = &analyses[..analyses.len().min(10)];
        let opening_avg_loss: i32 = opening_moves.iter().map(|a| a.centipawn_loss).sum::<i32>()
            / opening_moves.len().max(1) as i32;

        if opening_avg_loss < 40 {
            strengths.push("Strong opening knowledge".to_string());
        }

        if strengths.is_empty() {
            strengths.push("Solid fundamentals".to_string());
        }

        strengths
    }

    fn generate_recommendations(
        &self,
        weaknesses: Vec<String>,
        strengths: Vec<String>,
    ) -> AgentRecommendation {
        let difficulty = self.profile.get_recommended_difficulty();

        // Determine number of exercises based on weaknesses
        let exercise_count = if weaknesses.len() > 3 {
            10
        } else if weaknesses.len() > 1 {
            7
        } else {
            5
        };

        // Generate focus areas
        let focus_areas = self.determine_focus_areas(&weaknesses);

        // Generate personalized message
        let message = self.generate_personalized_message(&weaknesses, &strengths);

        AgentRecommendation {
            weaknesses_identified: weaknesses,
            strengths_identified: strengths,
            recommended_exercises: exercise_count,
            recommended_difficulty: difficulty,
            personalized_message: message,
            focus_areas,
        }
    }

    fn determine_focus_areas(&self, weaknesses: &[String]) -> Vec<String> {
        let mut areas = Vec::new();

        for weakness in weaknesses {
            let weakness_lower = weakness.to_lowercase();

            if weakness_lower.contains("opening") {
                areas.push("Opening Principles".to_string());
            } else if weakness_lower.contains("tactic") || weakness_lower.contains("blunder") {
                areas.push("Tactical Training".to_string());
            } else if weakness_lower.contains("endgame") {
                areas.push("Endgame Mastery".to_string());
            } else if weakness_lower.contains("positional") {
                areas.push("Positional Understanding".to_string());
            } else if weakness_lower.contains("calculation") {
                areas.push("Calculation Practice".to_string());
            }
        }

        // Deduplicate
        areas.sort();
        areas.dedup();

        if areas.is_empty() {
            areas.push("General Improvement".to_string());
        }

        areas
    }

    fn generate_personalized_message(
        &self,
        weaknesses: &[String],
        strengths: &[String],
    ) -> String {
        let mut message = String::new();

        // Start with play style
        message.push_str(&format!(
            "Your play style is: {:?}. ",
            self.profile.play_style
        ));

        // Add strengths
        if !strengths.is_empty() {
            message.push_str(&format!(
                "Your strengths include: {}. ",
                strengths.join(", ")
            ));
        }

        // Add weaknesses and recommendations
        if !weaknesses.is_empty() {
            message.push_str(&format!(
                "Areas for improvement: {}. ",
                weaknesses.join(", ")
            ));
            message.push_str(
                "I've prepared a customized training session to help you work on these areas."
            );
        } else {
            message.push_str(
                "You're playing well! Let's continue building on your solid foundation."
            );
        }

        message
    }

    pub fn create_training_session(&self) -> TrainingSession {
        TrainingSession::with_weaknesses(
            self.profile.user_id,
            self.profile.weaknesses.clone(),
            self.profile.get_recommended_difficulty(),
        )
    }

    pub fn update_from_training_session(&mut self, session: &TrainingSession) {
        self.profile
            .increment_exercises_completed(session.results.len() as u32);

        // Potentially update skill level based on performance
        let session_result = session.get_session_result();
        if session_result.success_rate > 80.0 {
            // Consider increasing difficulty/rating
            let new_rating = self.profile.estimated_rating + 50;
            self.profile.update_rating(new_rating);
        } else if session_result.success_rate < 40.0 {
            // Consider decreasing difficulty/rating
            let new_rating = self.profile.estimated_rating.saturating_sub(25);
            self.profile.update_rating(new_rating);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::Color;

    #[test]
    fn test_learning_agent_creation() {
        let agent = LearningAgent::new(1);
        assert_eq!(agent.profile.user_id, 1);
    }

    #[test]
    fn test_analyze_game() {
        let mut agent = LearningAgent::new(1);
        let game = ChessGame::new(Color::White);

        let recommendation = agent.analyze_game(game);
        assert!(!recommendation.personalized_message.is_empty());
    }

    #[test]
    fn test_create_training_session() {
        let mut agent = LearningAgent::new(1);
        agent.profile.update_weaknesses(vec!["Weak opening play".to_string()]);

        let session = agent.create_training_session();
        assert!(!session.exercises.is_empty());
    }
}
