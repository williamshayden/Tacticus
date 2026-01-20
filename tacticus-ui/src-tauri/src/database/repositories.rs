use rusqlite::{params, Connection, OptionalExtension, Result};
use serde::{Deserialize, Serialize};

// ============================================================================
// Profile Repository
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: i64,
    pub name: String,
    pub initial_level: String,
    pub current_elo: i32,
    pub peak_elo: i32,
    pub games_played: i32,
    pub exercises_completed: i32,
    pub streak: i32,
    pub style: String,
    pub weaknesses: Vec<String>,
    pub strengths: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn create_profile(conn: &Connection, name: &str, initial_level: &str, initial_elo: i32) -> Result<Profile> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT INTO profiles (name, initial_level, current_elo, peak_elo, style, weaknesses, strengths, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?3, 'Unknown', '[]', '[]', ?4, ?4)
        "#,
        params![name, initial_level, initial_elo, now],
    )?;

    let id = conn.last_insert_rowid();
    get_profile_by_id(conn, id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn get_profile_by_id(conn: &Connection, id: i64) -> Result<Option<Profile>> {
    conn.query_row(
        "SELECT id, name, initial_level, current_elo, peak_elo, games_played, exercises_completed, streak, style, weaknesses, strengths, created_at, updated_at FROM profiles WHERE id = ?1",
        params![id],
        |row| {
            let weaknesses_json: String = row.get(9)?;
            let strengths_json: String = row.get(10)?;
            Ok(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
                initial_level: row.get(2)?,
                current_elo: row.get(3)?,
                peak_elo: row.get(4)?,
                games_played: row.get(5)?,
                exercises_completed: row.get(6)?,
                streak: row.get(7)?,
                style: row.get(8)?,
                weaknesses: serde_json::from_str(&weaknesses_json).unwrap_or_default(),
                strengths: serde_json::from_str(&strengths_json).unwrap_or_default(),
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        },
    )
    .optional()
}

pub fn get_first_profile(conn: &Connection) -> Result<Option<Profile>> {
    conn.query_row(
        "SELECT id, name, initial_level, current_elo, peak_elo, games_played, exercises_completed, streak, style, weaknesses, strengths, created_at, updated_at FROM profiles ORDER BY id LIMIT 1",
        [],
        |row| {
            let weaknesses_json: String = row.get(9)?;
            let strengths_json: String = row.get(10)?;
            Ok(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
                initial_level: row.get(2)?,
                current_elo: row.get(3)?,
                peak_elo: row.get(4)?,
                games_played: row.get(5)?,
                exercises_completed: row.get(6)?,
                streak: row.get(7)?,
                style: row.get(8)?,
                weaknesses: serde_json::from_str(&weaknesses_json).unwrap_or_default(),
                strengths: serde_json::from_str(&strengths_json).unwrap_or_default(),
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        },
    )
    .optional()
}

pub fn update_profile(conn: &Connection, profile: &Profile) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    let weaknesses_json = serde_json::to_string(&profile.weaknesses).unwrap_or_else(|_| "[]".to_string());
    let strengths_json = serde_json::to_string(&profile.strengths).unwrap_or_else(|_| "[]".to_string());

    conn.execute(
        r#"
        UPDATE profiles SET
            name = ?1, current_elo = ?2, peak_elo = ?3, games_played = ?4,
            exercises_completed = ?5, streak = ?6, style = ?7,
            weaknesses = ?8, strengths = ?9, updated_at = ?10
        WHERE id = ?11
        "#,
        params![
            profile.name,
            profile.current_elo,
            profile.peak_elo,
            profile.games_played,
            profile.exercises_completed,
            profile.streak,
            profile.style,
            weaknesses_json,
            strengths_json,
            now,
            profile.id,
        ],
    )?;

    Ok(())
}

// ============================================================================
// Game Repository
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: i64,
    pub profile_id: i64,
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
    pub created_at: String,
    pub finished_at: Option<String>,
}

pub fn create_game(conn: &Connection, game: &Game) -> Result<i64> {
    let moves_json = serde_json::to_string(&game.moves).unwrap_or_else(|_| "[]".to_string());
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT INTO games (profile_id, initial_fen, final_fen, moves, result, player_color, opponent_type, opponent_elo, analysis, mistakes, blunders, opening_name, created_at, finished_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
        "#,
        params![
            game.profile_id,
            game.initial_fen,
            game.final_fen,
            moves_json,
            game.result,
            game.player_color,
            game.opponent_type,
            game.opponent_elo,
            game.analysis,
            game.mistakes,
            game.blunders,
            game.opening_name,
            now,
            game.finished_at,
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_recent_games(conn: &Connection, profile_id: i64, limit: i32) -> Result<Vec<Game>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, profile_id, initial_fen, final_fen, moves, result, player_color, opponent_type, opponent_elo, analysis, mistakes, blunders, opening_name, created_at, finished_at
        FROM games
        WHERE profile_id = ?1
        ORDER BY created_at DESC
        LIMIT ?2
        "#,
    )?;

    let games = stmt.query_map(params![profile_id, limit], |row| {
        let moves_json: String = row.get(4)?;
        Ok(Game {
            id: row.get(0)?,
            profile_id: row.get(1)?,
            initial_fen: row.get(2)?,
            final_fen: row.get(3)?,
            moves: serde_json::from_str(&moves_json).unwrap_or_default(),
            result: row.get(5)?,
            player_color: row.get(6)?,
            opponent_type: row.get(7)?,
            opponent_elo: row.get(8)?,
            analysis: row.get(9)?,
            mistakes: row.get(10)?,
            blunders: row.get(11)?,
            opening_name: row.get(12)?,
            created_at: row.get(13)?,
            finished_at: row.get(14)?,
        })
    })?;

    games.collect()
}

pub fn get_games_by_opening(conn: &Connection, profile_id: i64, opening: &str) -> Result<Vec<Game>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, profile_id, initial_fen, final_fen, moves, result, player_color, opponent_type, opponent_elo, analysis, mistakes, blunders, opening_name, created_at, finished_at
        FROM games
        WHERE profile_id = ?1 AND opening_name LIKE ?2
        ORDER BY created_at DESC
        "#,
    )?;

    let pattern = format!("%{}%", opening);
    let games = stmt.query_map(params![profile_id, pattern], |row| {
        let moves_json: String = row.get(4)?;
        Ok(Game {
            id: row.get(0)?,
            profile_id: row.get(1)?,
            initial_fen: row.get(2)?,
            final_fen: row.get(3)?,
            moves: serde_json::from_str(&moves_json).unwrap_or_default(),
            result: row.get(5)?,
            player_color: row.get(6)?,
            opponent_type: row.get(7)?,
            opponent_elo: row.get(8)?,
            analysis: row.get(9)?,
            mistakes: row.get(10)?,
            blunders: row.get(11)?,
            opening_name: row.get(12)?,
            created_at: row.get(13)?,
            finished_at: row.get(14)?,
        })
    })?;

    games.collect()
}

pub fn get_games_with_mistakes(conn: &Connection, profile_id: i64, min_mistakes: i32) -> Result<Vec<Game>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, profile_id, initial_fen, final_fen, moves, result, player_color, opponent_type, opponent_elo, analysis, mistakes, blunders, opening_name, created_at, finished_at
        FROM games
        WHERE profile_id = ?1 AND (mistakes >= ?2 OR blunders > 0)
        ORDER BY created_at DESC
        "#,
    )?;

    let games = stmt.query_map(params![profile_id, min_mistakes], |row| {
        let moves_json: String = row.get(4)?;
        Ok(Game {
            id: row.get(0)?,
            profile_id: row.get(1)?,
            initial_fen: row.get(2)?,
            final_fen: row.get(3)?,
            moves: serde_json::from_str(&moves_json).unwrap_or_default(),
            result: row.get(5)?,
            player_color: row.get(6)?,
            opponent_type: row.get(7)?,
            opponent_elo: row.get(8)?,
            analysis: row.get(9)?,
            mistakes: row.get(10)?,
            blunders: row.get(11)?,
            opening_name: row.get(12)?,
            created_at: row.get(13)?,
            finished_at: row.get(14)?,
        })
    })?;

    games.collect()
}

// ============================================================================
// Conversation Repository
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: i64,
    pub profile_id: i64,
    pub title: Option<String>,
    pub context: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub conversation_id: i64,
    pub role: String,
    pub content: String,
    pub tool_calls: Option<String>,
    pub tool_results: Option<String>,
    pub created_at: String,
}

pub fn create_conversation(conn: &Connection, profile_id: i64, title: Option<&str>, context: Option<&str>) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO conversations (profile_id, title, context, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4)",
        params![profile_id, title, context, now],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_conversation(conn: &Connection, id: i64) -> Result<Option<Conversation>> {
    conn.query_row(
        "SELECT id, profile_id, title, context, created_at, updated_at FROM conversations WHERE id = ?1",
        params![id],
        |row| Ok(Conversation {
            id: row.get(0)?,
            profile_id: row.get(1)?,
            title: row.get(2)?,
            context: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        }),
    )
    .optional()
}

pub fn get_recent_conversations(conn: &Connection, profile_id: i64, limit: i32) -> Result<Vec<Conversation>> {
    let mut stmt = conn.prepare(
        "SELECT id, profile_id, title, context, created_at, updated_at FROM conversations WHERE profile_id = ?1 ORDER BY updated_at DESC LIMIT ?2",
    )?;

    let convs = stmt.query_map(params![profile_id, limit], |row| {
        Ok(Conversation {
            id: row.get(0)?,
            profile_id: row.get(1)?,
            title: row.get(2)?,
            context: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })?;

    convs.collect()
}

pub fn add_message(conn: &Connection, conversation_id: i64, role: &str, content: &str, tool_calls: Option<&str>, tool_results: Option<&str>) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO messages (conversation_id, role, content, tool_calls, tool_results, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![conversation_id, role, content, tool_calls, tool_results, now],
    )?;

    // Update conversation updated_at
    conn.execute(
        "UPDATE conversations SET updated_at = ?1 WHERE id = ?2",
        params![now, conversation_id],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_conversation_messages(conn: &Connection, conversation_id: i64) -> Result<Vec<Message>> {
    let mut stmt = conn.prepare(
        "SELECT id, conversation_id, role, content, tool_calls, tool_results, created_at FROM messages WHERE conversation_id = ?1 ORDER BY created_at ASC",
    )?;

    let messages = stmt.query_map(params![conversation_id], |row| {
        Ok(Message {
            id: row.get(0)?,
            conversation_id: row.get(1)?,
            role: row.get(2)?,
            content: row.get(3)?,
            tool_calls: row.get(4)?,
            tool_results: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;

    messages.collect()
}

// ============================================================================
// Exercise Results Repository
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseResult {
    pub id: i64,
    pub profile_id: i64,
    pub exercise_type: String,
    pub difficulty: String,
    pub position_fen: String,
    pub solved: bool,
    pub attempts: i32,
    pub time_seconds: i32,
    pub hints_used: i32,
    pub created_at: String,
}

pub fn record_exercise_result(conn: &Connection, result: &ExerciseResult) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT INTO exercise_results (profile_id, exercise_type, difficulty, position_fen, solved, attempts, time_seconds, hints_used, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#,
        params![
            result.profile_id,
            result.exercise_type,
            result.difficulty,
            result.position_fen,
            result.solved as i32,
            result.attempts,
            result.time_seconds,
            result.hints_used,
            now,
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_training_progress(conn: &Connection, profile_id: i64, exercise_type: Option<&str>) -> Result<TrainingProgress> {
    let (total, solved, avg_time, avg_hints): (i32, i32, f64, f64) = if let Some(ex_type) = exercise_type {
        conn.query_row(
            r#"
            SELECT
                COUNT(*) as total,
                SUM(CASE WHEN solved = 1 THEN 1 ELSE 0 END) as solved,
                AVG(time_seconds) as avg_time,
                AVG(hints_used) as avg_hints
            FROM exercise_results
            WHERE profile_id = ?1 AND exercise_type = ?2
            "#,
            params![profile_id, ex_type],
            |row| Ok((row.get(0)?, row.get(1)?, row.get::<_, Option<f64>>(2)?.unwrap_or(0.0), row.get::<_, Option<f64>>(3)?.unwrap_or(0.0))),
        )?
    } else {
        conn.query_row(
            r#"
            SELECT
                COUNT(*) as total,
                SUM(CASE WHEN solved = 1 THEN 1 ELSE 0 END) as solved,
                AVG(time_seconds) as avg_time,
                AVG(hints_used) as avg_hints
            FROM exercise_results
            WHERE profile_id = ?1
            "#,
            params![profile_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get::<_, Option<f64>>(2)?.unwrap_or(0.0), row.get::<_, Option<f64>>(3)?.unwrap_or(0.0))),
        )?
    };

    Ok(TrainingProgress {
        total_attempted: total,
        total_solved: solved,
        success_rate: if total > 0 { (solved as f64 / total as f64) * 100.0 } else { 0.0 },
        avg_time_seconds: avg_time,
        avg_hints_used: avg_hints,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingProgress {
    pub total_attempted: i32,
    pub total_solved: i32,
    pub success_rate: f64,
    pub avg_time_seconds: f64,
    pub avg_hints_used: f64,
}

// ============================================================================
// Settings Repository
// ============================================================================

pub fn get_setting(conn: &Connection, key: &str) -> Result<Option<String>> {
    conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        params![key],
        |row| row.get(0),
    )
    .optional()
}

pub fn set_setting(conn: &Connection, key: &str, value: &str) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3) ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = ?3",
        params![key, value, now],
    )?;

    Ok(())
}

// ============================================================================
// Player Stats (computed from data)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub current_elo: i32,
    pub peak_elo: i32,
    pub games_played: i32,
    pub wins: i32,
    pub losses: i32,
    pub draws: i32,
    pub win_rate: f64,
    pub exercises_completed: i32,
    pub exercises_solved: i32,
    pub exercise_success_rate: f64,
    pub streak: i32,
    pub style: String,
    pub weaknesses: Vec<String>,
    pub strengths: Vec<String>,
}

pub fn get_player_stats(conn: &Connection, profile_id: i64) -> Result<Option<PlayerStats>> {
    let profile = match get_profile_by_id(conn, profile_id)? {
        Some(p) => p,
        None => return Ok(None),
    };

    // Get game stats
    let (wins, losses, draws): (i32, i32, i32) = conn.query_row(
        r#"
        SELECT
            SUM(CASE WHEN result = 'win' THEN 1 ELSE 0 END),
            SUM(CASE WHEN result = 'loss' THEN 1 ELSE 0 END),
            SUM(CASE WHEN result = 'draw' THEN 1 ELSE 0 END)
        FROM games WHERE profile_id = ?1
        "#,
        params![profile_id],
        |row| Ok((
            row.get::<_, Option<i32>>(0)?.unwrap_or(0),
            row.get::<_, Option<i32>>(1)?.unwrap_or(0),
            row.get::<_, Option<i32>>(2)?.unwrap_or(0),
        )),
    )?;

    let total_games = wins + losses + draws;
    let win_rate = if total_games > 0 { (wins as f64 / total_games as f64) * 100.0 } else { 0.0 };

    // Get exercise stats
    let (exercises_completed, exercises_solved): (i32, i32) = conn.query_row(
        r#"
        SELECT
            COUNT(*),
            SUM(CASE WHEN solved = 1 THEN 1 ELSE 0 END)
        FROM exercise_results WHERE profile_id = ?1
        "#,
        params![profile_id],
        |row| Ok((
            row.get::<_, Option<i32>>(0)?.unwrap_or(0),
            row.get::<_, Option<i32>>(1)?.unwrap_or(0),
        )),
    )?;

    let exercise_success_rate = if exercises_completed > 0 {
        (exercises_solved as f64 / exercises_completed as f64) * 100.0
    } else {
        0.0
    };

    Ok(Some(PlayerStats {
        current_elo: profile.current_elo,
        peak_elo: profile.peak_elo,
        games_played: total_games,
        wins,
        losses,
        draws,
        win_rate,
        exercises_completed,
        exercises_solved,
        exercise_success_rate,
        streak: profile.streak,
        style: profile.style,
        weaknesses: profile.weaknesses,
        strengths: profile.strengths,
    }))
}

// ============================================================================
// Improvement Trend
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementTrend {
    pub elo_change: i32,
    pub games_in_period: i32,
    pub win_rate_in_period: f64,
    pub exercises_in_period: i32,
    pub exercise_success_rate_in_period: f64,
}

pub fn get_improvement_trend(conn: &Connection, profile_id: i64, days: i32) -> Result<ImprovementTrend> {
    let cutoff = chrono::Utc::now() - chrono::Duration::days(days as i64);
    let cutoff_str = cutoff.to_rfc3339();

    // Get games in period
    let (games_count, wins): (i32, i32) = conn.query_row(
        r#"
        SELECT
            COUNT(*),
            SUM(CASE WHEN result = 'win' THEN 1 ELSE 0 END)
        FROM games
        WHERE profile_id = ?1 AND created_at >= ?2
        "#,
        params![profile_id, cutoff_str],
        |row| Ok((
            row.get::<_, Option<i32>>(0)?.unwrap_or(0),
            row.get::<_, Option<i32>>(1)?.unwrap_or(0),
        )),
    )?;

    let win_rate = if games_count > 0 { (wins as f64 / games_count as f64) * 100.0 } else { 0.0 };

    // Get exercises in period
    let (exercises_count, exercises_solved): (i32, i32) = conn.query_row(
        r#"
        SELECT
            COUNT(*),
            SUM(CASE WHEN solved = 1 THEN 1 ELSE 0 END)
        FROM exercise_results
        WHERE profile_id = ?1 AND created_at >= ?2
        "#,
        params![profile_id, cutoff_str],
        |row| Ok((
            row.get::<_, Option<i32>>(0)?.unwrap_or(0),
            row.get::<_, Option<i32>>(1)?.unwrap_or(0),
        )),
    )?;

    let exercise_success_rate = if exercises_count > 0 {
        (exercises_solved as f64 / exercises_count as f64) * 100.0
    } else {
        0.0
    };

    // Calculate ELO change (simplified - based on win/loss ratio)
    let elo_change = (wins - (games_count - wins)) * 15; // Rough estimate

    Ok(ImprovementTrend {
        elo_change,
        games_in_period: games_count,
        win_rate_in_period: win_rate,
        exercises_in_period: exercises_count,
        exercise_success_rate_in_period: exercise_success_rate,
    })
}

// ============================================================================
// Weakness History
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaknessEntry {
    pub exercise_type: String,
    pub total_attempts: i32,
    pub success_rate: f64,
    pub recent_trend: String, // "improving", "stable", "declining"
}

pub fn get_weakness_history(conn: &Connection, profile_id: i64, days: i32) -> Result<Vec<WeaknessEntry>> {
    let cutoff = chrono::Utc::now() - chrono::Duration::days(days as i64);
    let cutoff_str = cutoff.to_rfc3339();

    let mut stmt = conn.prepare(
        r#"
        SELECT
            exercise_type,
            COUNT(*) as attempts,
            AVG(CASE WHEN solved = 1 THEN 1.0 ELSE 0.0 END) as success_rate
        FROM exercise_results
        WHERE profile_id = ?1 AND created_at >= ?2
        GROUP BY exercise_type
        ORDER BY success_rate ASC
        "#,
    )?;

    let entries = stmt.query_map(params![profile_id, cutoff_str], |row| {
        let success_rate: f64 = row.get(2)?;
        let trend = if success_rate < 0.5 {
            "declining"
        } else if success_rate < 0.75 {
            "stable"
        } else {
            "improving"
        };

        Ok(WeaknessEntry {
            exercise_type: row.get(0)?,
            total_attempts: row.get(1)?,
            success_rate: success_rate * 100.0,
            recent_trend: trend.to_string(),
        })
    })?;

    entries.collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::create_tables;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();
        conn
    }

    #[test]
    fn test_profile_crud() {
        let conn = setup_test_db();

        // Create
        let profile = create_profile(&conn, "Test User", "beginner", 800).unwrap();
        assert_eq!(profile.name, "Test User");
        assert_eq!(profile.current_elo, 800);

        // Read
        let fetched = get_profile_by_id(&conn, profile.id).unwrap().unwrap();
        assert_eq!(fetched.name, "Test User");

        // Update
        let mut updated = fetched.clone();
        updated.current_elo = 900;
        update_profile(&conn, &updated).unwrap();

        let refetched = get_profile_by_id(&conn, profile.id).unwrap().unwrap();
        assert_eq!(refetched.current_elo, 900);
    }

    #[test]
    fn test_settings() {
        let conn = setup_test_db();

        set_setting(&conn, "api_key", "test-key-123").unwrap();
        let value = get_setting(&conn, "api_key").unwrap();
        assert_eq!(value, Some("test-key-123".to_string()));

        // Update
        set_setting(&conn, "api_key", "new-key-456").unwrap();
        let value = get_setting(&conn, "api_key").unwrap();
        assert_eq!(value, Some("new-key-456".to_string()));
    }
}
