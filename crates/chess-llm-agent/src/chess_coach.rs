use chess_core::{ChessGame, MoveQuality};
use chess_engine::{GameAnalyzer, MoveAnalysis};
use chess_ai::{PlayerProfile, PlayStyle};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::openrouter::{OpenRouterClient, ChatMessage};
use crate::prompts::ChessCoachPrompts;
use crate::conversation::ConversationManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoachFeedback {
    pub overall_assessment: String,
    pub key_moments: Vec<String>,
    pub strengths: Vec<String>,
    pub areas_to_improve: Vec<String>,
    pub training_recommendations: Vec<String>,
    pub motivational_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoachingSession {
    pub session_id: String,
    pub player_id: u64,
    pub conversation: ConversationManager,
    pub context: SessionContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub recent_games: Vec<GameSummary>,
    pub player_stats: PlayerStats,
    pub current_focus: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSummary {
    pub game_id: u64,
    pub result: String,
    pub player_color: String,
    pub opening: String,
    pub move_count: usize,
    pub blunders: u32,
    pub mistakes: u32,
    pub average_centipawn_loss: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub rating: u32,
    pub games_played: u32,
    pub win_rate: f32,
    pub play_style: String,
    pub top_weaknesses: Vec<String>,
    pub recent_progress: String,
}

/// Chess coach powered by LLM with tool-calling capabilities
pub struct ChessCoach {
    client: OpenRouterClient,
    model: String,
}

impl ChessCoach {
    pub fn new(client: OpenRouterClient) -> Self {
        Self {
            client,
            model: "anthropic/claude-3.5-sonnet".to_string(), // High-quality model for coaching
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Start a new coaching session
    pub fn start_session(player_id: u64, profile: &PlayerProfile) -> CoachingSession {
        let system_prompt = ChessCoachPrompts::system_prompt();
        let conversation = ConversationManager::new(system_prompt);

        CoachingSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            player_id,
            conversation,
            context: SessionContext {
                recent_games: Vec::new(),
                player_stats: PlayerStats {
                    rating: profile.estimated_rating,
                    games_played: profile.games_played,
                    win_rate: 0.0, // Calculate from games
                    play_style: format!("{:?}", profile.play_style),
                    top_weaknesses: profile.weaknesses.clone(),
                    recent_progress: "Just starting out".to_string(),
                },
                current_focus: None,
            },
        }
    }

    /// Analyze a game and provide coaching feedback
    pub async fn analyze_game(
        &self,
        session: &mut CoachingSession,
        game: &ChessGame,
        analyses: &[MoveAnalysis],
    ) -> Result<String> {
        // Build move quality summary
        let move_quality_summary = self.build_move_quality_summary(analyses);

        // Get weaknesses
        let weaknesses = chess_engine::GameAnalyzer::identify_weaknesses(analyses);

        // Convert game to PGN-like representation
        let pgn = self.game_to_simple_notation(game, analyses);

        // Create analysis prompt
        let player_color = format!("{:?}", game.player_color);
        let prompt = ChessCoachPrompts::game_analysis_prompt(
            &pgn,
            &player_color,
            &move_quality_summary,
            &weaknesses,
        );

        // Add to conversation
        session.conversation.add_user_message(prompt);

        // Get LLM response
        let messages = session.conversation.get_chat_messages();
        let response = self.client.simple_chat(&self.model, messages).await?;

        // Add response to conversation
        session.conversation.add_assistant_message(response.clone());

        Ok(response)
    }

    /// Get playstyle analysis with personalized insights
    pub async fn analyze_playstyle(
        &self,
        session: &mut CoachingSession,
        profile: &PlayerProfile,
        games_count: usize,
    ) -> Result<String> {
        let prompt = ChessCoachPrompts::playstyle_analysis_prompt(
            &profile.play_style,
            profile.style_characteristics.aggression_score,
            profile.style_characteristics.tactical_score,
            profile.style_characteristics.positional_score,
            games_count,
        );

        session.conversation.add_user_message(prompt);
        let messages = session.conversation.get_chat_messages();
        let response = self.client.simple_chat(&self.model, messages).await?;
        session.conversation.add_assistant_message(response.clone());

        Ok(response)
    }

    /// Provide a hint for an exercise
    pub async fn provide_hint(
        &self,
        session: &mut CoachingSession,
        position_fen: &str,
        exercise_goal: &str,
        hint_level: u32,
    ) -> Result<String> {
        let prompt = ChessCoachPrompts::exercise_hint_prompt(position_fen, exercise_goal, hint_level);

        session.conversation.add_user_message(prompt);
        let messages = session.conversation.get_chat_messages();
        let response = self.client.simple_chat(&self.model, messages).await?;
        session.conversation.add_assistant_message(response.clone());

        Ok(response)
    }

    /// Create a personalized training plan
    pub async fn create_training_plan(
        &self,
        session: &mut CoachingSession,
        profile: &PlayerProfile,
    ) -> Result<String> {
        let top_weaknesses: Vec<String> = profile.weaknesses.iter().take(3).cloned().collect();

        let prompt = ChessCoachPrompts::personalized_training_plan_prompt(
            profile.estimated_rating,
            &profile.play_style,
            &top_weaknesses,
            &session.context.player_stats.recent_progress,
        );

        session.conversation.add_user_message(prompt);
        let messages = session.conversation.get_chat_messages();
        let response = self.client.simple_chat(&self.model, messages).await?;
        session.conversation.add_assistant_message(response.clone());

        Ok(response)
    }

    /// Chat freely with the coach
    pub async fn chat(
        &self,
        session: &mut CoachingSession,
        user_message: &str,
    ) -> Result<String> {
        session.conversation.add_user_message(user_message);
        let messages = session.conversation.get_chat_messages();
        let response = self.client.simple_chat(&self.model, messages).await?;
        session.conversation.add_assistant_message(response.clone());

        Ok(response)
    }

    /// Provide encouragement based on context
    pub async fn encourage(
        &self,
        context: &str,
    ) -> Result<String> {
        let prompt = ChessCoachPrompts::encouragement_prompt(context);
        let messages = vec![
            ChatMessage::system(ChessCoachPrompts::system_prompt()),
            ChatMessage::user(prompt),
        ];

        let response = self.client.simple_chat(&self.model, messages).await?;
        Ok(response)
    }

    // Helper methods

    fn build_move_quality_summary(&self, analyses: &[MoveAnalysis]) -> String {
        let total = analyses.len();
        let brilliant = analyses.iter().filter(|a| a.quality == MoveQuality::Brilliant).count();
        let great = analyses.iter().filter(|a| a.quality == MoveQuality::Great).count();
        let good = analyses.iter().filter(|a| a.quality == MoveQuality::Good).count();
        let inaccuracies = analyses.iter().filter(|a| a.quality == MoveQuality::Inaccuracy).count();
        let mistakes = analyses.iter().filter(|a| a.quality == MoveQuality::Mistake).count();
        let blunders = analyses.iter().filter(|a| a.quality == MoveQuality::Blunder).count();

        let avg_loss: i32 = analyses.iter().map(|a| a.centipawn_loss).sum::<i32>() / total.max(1) as i32;

        format!(
            "Total Moves: {}\n\
             Brilliant: {} (!!)\n\
             Great: {} (!)\n\
             Good: {}\n\
             Inaccuracies: {} (?!)\n\
             Mistakes: {} (?)\n\
             Blunders: {} (??)\n\
             Average Centipawn Loss: {}",
            total, brilliant, great, good, inaccuracies, mistakes, blunders, avg_loss
        )
    }

    fn game_to_simple_notation(&self, game: &ChessGame, analyses: &[MoveAnalysis]) -> String {
        let mut notation = String::new();

        for (i, analysis) in analyses.iter().enumerate() {
            let move_num = (i / 2) + 1;
            let color = if i % 2 == 0 { "White" } else { "Black" };

            let quality_symbol = match analysis.quality {
                MoveQuality::Brilliant => "!!",
                MoveQuality::Great => "!",
                MoveQuality::Good => "",
                MoveQuality::Inaccuracy => "?!",
                MoveQuality::Mistake => "?",
                MoveQuality::Blunder => "??",
            };

            notation.push_str(&format!(
                "{}. {} {}{} (eval: {}, loss: {})\n",
                move_num,
                color,
                analysis.chess_move,
                quality_symbol,
                analysis.evaluation_after,
                analysis.centipawn_loss
            ));
        }

        notation
    }
}

// UUID helper (simple implementation)
mod uuid {
    use std::time::{SystemTime, UNIX_EPOCH};
    use rand::Rng;

    pub struct Uuid;

    impl Uuid {
        pub fn new_v4() -> impl ToString {
            let mut rng = rand::thread_rng();
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            UuidString(format!(
                "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
                timestamp & 0xffffffff,
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u64>() & 0xffffffffffff,
            ))
        }
    }

    struct UuidString(String);

    impl ToString for UuidString {
        fn to_string(&self) -> String {
            self.0.clone()
        }
    }
}
