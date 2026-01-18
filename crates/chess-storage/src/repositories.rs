use sqlx::{SqlitePool, Result, Row};
use chess_ai::PlayerProfile;
use chess_core::ChessGame;
use chess_trainer::{Exercise, ExerciseResult, TrainingSession};
use serde_json;

pub struct ProfileRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> ProfileRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, profile: &PlayerProfile) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO profiles (
                user_id, skill_level, estimated_rating, play_style,
                style_characteristics, games_played, exercises_completed,
                weaknesses, strengths, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(profile.user_id as i64)
        .bind(format!("{:?}", profile.skill_level))
        .bind(profile.estimated_rating as i64)
        .bind(format!("{:?}", profile.play_style))
        .bind(serde_json::to_string(&profile.style_characteristics).unwrap())
        .bind(profile.games_played as i64)
        .bind(profile.exercises_completed as i64)
        .bind(serde_json::to_string(&profile.weaknesses).unwrap())
        .bind(serde_json::to_string(&profile.strengths).unwrap())
        .bind(profile.created_at.to_rfc3339())
        .bind(profile.updated_at.to_rfc3339())
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn get(&self, user_id: u64) -> Result<Option<PlayerProfile>> {
        let row = sqlx::query(
            r#"
            SELECT user_id, skill_level, estimated_rating, play_style,
                   style_characteristics, games_played, exercises_completed,
                   weaknesses, strengths, created_at, updated_at
            FROM profiles
            WHERE user_id = ?
            "#,
        )
        .bind(user_id as i64)
        .fetch_optional(self.pool)
        .await?;

        if let Some(row) = row {
            let profile = PlayerProfile {
                user_id: row.get::<i64, _>("user_id") as u64,
                skill_level: serde_json::from_str(&row.get::<String, _>("skill_level")).unwrap(),
                estimated_rating: row.get::<i64, _>("estimated_rating") as u32,
                play_style: serde_json::from_str(&row.get::<String, _>("play_style")).unwrap(),
                style_characteristics: serde_json::from_str(&row.get::<String, _>("style_characteristics")).unwrap(),
                games_played: row.get::<i64, _>("games_played") as u32,
                exercises_completed: row.get::<i64, _>("exercises_completed") as u32,
                weaknesses: serde_json::from_str(&row.get::<String, _>("weaknesses")).unwrap(),
                strengths: serde_json::from_str(&row.get::<String, _>("strengths")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            };
            Ok(Some(profile))
        } else {
            Ok(None)
        }
    }

    pub async fn update(&self, profile: &PlayerProfile) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE profiles
            SET skill_level = ?, estimated_rating = ?, play_style = ?,
                style_characteristics = ?, games_played = ?, exercises_completed = ?,
                weaknesses = ?, strengths = ?, updated_at = ?
            WHERE user_id = ?
            "#,
        )
        .bind(format!("{:?}", profile.skill_level))
        .bind(profile.estimated_rating as i64)
        .bind(format!("{:?}", profile.play_style))
        .bind(serde_json::to_string(&profile.style_characteristics).unwrap())
        .bind(profile.games_played as i64)
        .bind(profile.exercises_completed as i64)
        .bind(serde_json::to_string(&profile.weaknesses).unwrap())
        .bind(serde_json::to_string(&profile.strengths).unwrap())
        .bind(profile.updated_at.to_rfc3339())
        .bind(profile.user_id as i64)
        .execute(self.pool)
        .await?;

        Ok(())
    }
}

pub struct GameRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> GameRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, game: &ChessGame) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO games (
                user_id, board_fen, move_history, game_state,
                player_color, created_at, finished_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(0i64) // Default user_id, should be set properly
        .bind(game.get_fen())
        .bind(serde_json::to_string(&game.move_history).unwrap())
        .bind(serde_json::to_string(&game.state).unwrap())
        .bind(format!("{:?}", game.player_color))
        .bind(game.created_at.to_rfc3339())
        .bind(game.finished_at.map(|dt| dt.to_rfc3339()))
        .execute(self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_user_games(&self, user_id: u64) -> Result<Vec<ChessGame>> {
        let rows = sqlx::query(
            r#"
            SELECT id, board_fen, move_history, game_state,
                   player_color, created_at, finished_at
            FROM games
            WHERE user_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id as i64)
        .fetch_all(self.pool)
        .await?;

        let mut games = Vec::new();
        for row in rows {
            let game_state: chess_core::GameState =
                serde_json::from_str(&row.get::<String, _>("game_state")).unwrap();
            let player_color_str = row.get::<String, _>("player_color");
            let player_color = match player_color_str.as_str() {
                "\"White\"" | "White" => chess::Color::White,
                "\"Black\"" | "Black" => chess::Color::Black,
                _ => chess::Color::White, // default
            };
            let move_history: chess_core::MoveHistory =
                serde_json::from_str(&row.get::<String, _>("move_history")).unwrap();

            let game = ChessGame {
                id: Some(row.get::<i64, _>("id") as u64),
                board: chess::Board::from_str(&row.get::<String, _>("board_fen")).unwrap(),
                move_history,
                state: game_state,
                player_color,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                finished_at: row
                    .get::<Option<String>, _>("finished_at")
                    .map(|s| chrono::DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&chrono::Utc)),
            };
            games.push(game);
        }

        Ok(games)
    }
}

use std::str::FromStr;

pub struct ExerciseRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> ExerciseRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, exercise: &Exercise) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO exercises (
                exercise_type, difficulty, position, title,
                description, solution_moves, hints, explanation
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(format!("{:?}", exercise.exercise_type))
        .bind(format!("{:?}", exercise.difficulty))
        .bind(&exercise.position)
        .bind(&exercise.title)
        .bind(&exercise.description)
        .bind(serde_json::to_string(&exercise.solution_moves).unwrap())
        .bind(serde_json::to_string(&exercise.hints).unwrap())
        .bind(&exercise.explanation)
        .execute(self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn record_result(&self, result: &ExerciseResult) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO exercise_results (
                exercise_id, user_id, solved, attempts,
                time_taken_seconds, hints_used, completed_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(result.exercise_id as i64)
        .bind(result.user_id as i64)
        .bind(result.solved)
        .bind(result.attempts as i64)
        .bind(result.time_taken_seconds as i64)
        .bind(result.hints_used as i64)
        .bind(result.completed_at.to_rfc3339())
        .execute(self.pool)
        .await?;

        Ok(())
    }
}

pub struct SessionRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SessionRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, session: &TrainingSession) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO training_sessions (
                user_id, exercises, current_exercise_index, results,
                strategies, difficulty, started_at, finished_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(session.user_id as i64)
        .bind(serde_json::to_string(&session.exercises).unwrap())
        .bind(session.current_exercise_index as i64)
        .bind(serde_json::to_string(&session.results).unwrap())
        .bind(serde_json::to_string(&session.strategies).unwrap())
        .bind(format!("{:?}", session.difficulty))
        .bind(session.started_at.to_rfc3339())
        .bind(session.finished_at.map(|dt| dt.to_rfc3339()))
        .execute(self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn update(&self, session: &TrainingSession) -> Result<()> {
        if let Some(id) = session.id {
            sqlx::query(
                r#"
                UPDATE training_sessions
                SET current_exercise_index = ?, results = ?, finished_at = ?
                WHERE id = ?
                "#,
            )
            .bind(session.current_exercise_index as i64)
            .bind(serde_json::to_string(&session.results).unwrap())
            .bind(session.finished_at.map(|dt| dt.to_rfc3339()))
            .bind(id as i64)
            .execute(self.pool)
            .await?;
        }

        Ok(())
    }
}
