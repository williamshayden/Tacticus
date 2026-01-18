use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Result;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn init_schema(&self) -> Result<()> {
        // Create profiles table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS profiles (
                user_id INTEGER PRIMARY KEY,
                skill_level TEXT NOT NULL,
                estimated_rating INTEGER NOT NULL,
                play_style TEXT NOT NULL,
                style_characteristics TEXT NOT NULL,
                games_played INTEGER NOT NULL DEFAULT 0,
                exercises_completed INTEGER NOT NULL DEFAULT 0,
                weaknesses TEXT NOT NULL,
                strengths TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create games table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS games (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                board_fen TEXT NOT NULL,
                move_history TEXT NOT NULL,
                game_state TEXT NOT NULL,
                player_color TEXT NOT NULL,
                created_at TEXT NOT NULL,
                finished_at TEXT,
                FOREIGN KEY (user_id) REFERENCES profiles(user_id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create exercises table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS exercises (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                exercise_type TEXT NOT NULL,
                difficulty TEXT NOT NULL,
                position TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                solution_moves TEXT NOT NULL,
                hints TEXT NOT NULL,
                explanation TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create exercise_results table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS exercise_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                exercise_id INTEGER NOT NULL,
                user_id INTEGER NOT NULL,
                solved BOOLEAN NOT NULL,
                attempts INTEGER NOT NULL,
                time_taken_seconds INTEGER NOT NULL,
                hints_used INTEGER NOT NULL,
                completed_at TEXT NOT NULL,
                FOREIGN KEY (exercise_id) REFERENCES exercises(id),
                FOREIGN KEY (user_id) REFERENCES profiles(user_id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create training_sessions table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS training_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                exercises TEXT NOT NULL,
                current_exercise_index INTEGER NOT NULL,
                results TEXT NOT NULL,
                strategies TEXT NOT NULL,
                difficulty TEXT NOT NULL,
                started_at TEXT NOT NULL,
                finished_at TEXT,
                FOREIGN KEY (user_id) REFERENCES profiles(user_id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_creation() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        assert!(db.init_schema().await.is_ok());
    }
}
