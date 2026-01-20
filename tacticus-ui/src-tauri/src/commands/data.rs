use serde::{Deserialize, Serialize};
use crate::DB;
use crate::database::repositories::{self, Game, ExerciseResult as DbExerciseResult, TrainingProgress, PlayerStats, ImprovementTrend, WeaknessEntry};

// ============================================================================
// Game Commands
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGameRequest {
    pub initial_fen: String,
    pub final_fen: String,
    pub moves: Vec<String>,
    pub result: String,
    pub player_color: String,
    pub opponent_type: String,
    pub opponent_elo: Option<i32>,
    pub analysis: Option<String>,
    pub mistakes: i32,
    pub blunders: i32,
    pub opening_name: Option<String>,
}

#[tauri::command]
pub fn save_game(game: SaveGameRequest) -> Result<i64, String> {
    let profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No user profile found".to_string())?;

    let db_game = Game {
        id: 0,
        profile_id: profile.id,
        initial_fen: game.initial_fen,
        final_fen: game.final_fen,
        moves: game.moves,
        result: game.result,
        player_color: game.player_color,
        opponent_type: game.opponent_type,
        opponent_elo: game.opponent_elo,
        analysis: game.analysis,
        mistakes: game.mistakes,
        blunders: game.blunders,
        opening_name: game.opening_name,
        created_at: String::new(),
        finished_at: Some(chrono::Utc::now().to_rfc3339()),
    };

    DB.with_conn(|conn| repositories::create_game(conn, &db_game))
        .map_err(|e| format!("Failed to save game: {}", e))
}

#[tauri::command]
pub fn get_recent_games(count: i32) -> Result<Vec<Game>, String> {
    let profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No user profile found".to_string())?;

    DB.with_conn(|conn| repositories::get_recent_games(conn, profile.id, count))
        .map_err(|e| format!("Failed to get games: {}", e))
}

#[tauri::command]
pub fn search_games_by_opening(opening_name: String) -> Result<Vec<Game>, String> {
    let profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No user profile found".to_string())?;

    DB.with_conn(|conn| repositories::get_games_by_opening(conn, profile.id, &opening_name))
        .map_err(|e| format!("Failed to search games: {}", e))
}

#[tauri::command]
pub fn get_games_with_mistakes(min_mistakes: i32) -> Result<Vec<Game>, String> {
    let profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No user profile found".to_string())?;

    DB.with_conn(|conn| repositories::get_games_with_mistakes(conn, profile.id, min_mistakes))
        .map_err(|e| format!("Failed to get games: {}", e))
}

// ============================================================================
// Exercise Result Commands
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordExerciseRequest {
    pub exercise_type: String,
    pub difficulty: String,
    pub position_fen: String,
    pub solved: bool,
    pub attempts: i32,
    pub time_seconds: i32,
    pub hints_used: i32,
}

#[tauri::command]
pub fn record_exercise_result(result: RecordExerciseRequest) -> Result<i64, String> {
    let profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No user profile found".to_string())?;

    let db_result = DbExerciseResult {
        id: 0,
        profile_id: profile.id,
        exercise_type: result.exercise_type,
        difficulty: result.difficulty,
        position_fen: result.position_fen,
        solved: result.solved,
        attempts: result.attempts,
        time_seconds: result.time_seconds,
        hints_used: result.hints_used,
        created_at: String::new(),
    };

    let result_id = DB
        .with_conn(|conn| repositories::record_exercise_result(conn, &db_result))
        .map_err(|e| format!("Failed to record exercise: {}", e))?;

    // Update profile exercise count
    let mut updated_profile = profile;
    updated_profile.exercises_completed += 1;
    DB.with_conn(|conn| repositories::update_profile(conn, &updated_profile))
        .map_err(|e| format!("Failed to update profile: {}", e))?;

    Ok(result_id)
}

#[tauri::command]
pub fn get_training_progress(exercise_type: Option<String>) -> Result<TrainingProgress, String> {
    let profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No user profile found".to_string())?;

    DB.with_conn(|conn| {
        repositories::get_training_progress(conn, profile.id, exercise_type.as_deref())
    })
    .map_err(|e| format!("Failed to get training progress: {}", e))
}

// ============================================================================
// Player Stats Commands (for AI agent)
// ============================================================================

#[tauri::command]
pub fn get_player_stats() -> Result<PlayerStats, String> {
    let profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No user profile found".to_string())?;

    DB.with_conn(|conn| repositories::get_player_stats(conn, profile.id))
        .map_err(|e| format!("Failed to get player stats: {}", e))?
        .ok_or_else(|| "No stats found".to_string())
}

#[tauri::command]
pub fn get_improvement_trend(days: i32) -> Result<ImprovementTrend, String> {
    let profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No user profile found".to_string())?;

    DB.with_conn(|conn| repositories::get_improvement_trend(conn, profile.id, days))
        .map_err(|e| format!("Failed to get improvement trend: {}", e))
}

#[tauri::command]
pub fn get_weakness_history(days: i32) -> Result<Vec<WeaknessEntry>, String> {
    let profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No user profile found".to_string())?;

    DB.with_conn(|conn| repositories::get_weakness_history(conn, profile.id, days))
        .map_err(|e| format!("Failed to get weakness history: {}", e))
}

// ============================================================================
// Conversation Commands
// ============================================================================

#[tauri::command]
pub fn create_conversation(title: Option<String>, context: Option<String>) -> Result<i64, String> {
    let profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No user profile found".to_string())?;

    DB.with_conn(|conn| {
        repositories::create_conversation(conn, profile.id, title.as_deref(), context.as_deref())
    })
    .map_err(|e| format!("Failed to create conversation: {}", e))
}

#[tauri::command]
pub fn add_message(
    conversation_id: i64,
    role: String,
    content: String,
    tool_calls: Option<String>,
    tool_results: Option<String>,
) -> Result<i64, String> {
    DB.with_conn(|conn| {
        repositories::add_message(
            conn,
            conversation_id,
            &role,
            &content,
            tool_calls.as_deref(),
            tool_results.as_deref(),
        )
    })
    .map_err(|e| format!("Failed to add message: {}", e))
}

#[tauri::command]
pub fn get_conversation_messages(conversation_id: i64) -> Result<Vec<repositories::Message>, String> {
    DB.with_conn(|conn| repositories::get_conversation_messages(conn, conversation_id))
        .map_err(|e| format!("Failed to get messages: {}", e))
}

#[tauri::command]
pub fn get_recent_conversations(limit: i32) -> Result<Vec<repositories::Conversation>, String> {
    let profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No user profile found".to_string())?;

    DB.with_conn(|conn| repositories::get_recent_conversations(conn, profile.id, limit))
        .map_err(|e| format!("Failed to get conversations: {}", e))
}
