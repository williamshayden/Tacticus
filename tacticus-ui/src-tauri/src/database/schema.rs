use rusqlite::{Connection, Result};

/// Create all database tables
pub fn create_tables(conn: &Connection) -> Result<()> {
    // Profiles table - user data, ELO, skill level
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS profiles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            initial_level TEXT NOT NULL,
            current_elo INTEGER NOT NULL,
            peak_elo INTEGER NOT NULL,
            games_played INTEGER NOT NULL DEFAULT 0,
            exercises_completed INTEGER NOT NULL DEFAULT 0,
            streak INTEGER NOT NULL DEFAULT 0,
            style TEXT NOT NULL DEFAULT 'Unknown',
            weaknesses TEXT NOT NULL DEFAULT '[]',
            strengths TEXT NOT NULL DEFAULT '[]',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#,
    )?;

    // Games table - game records with FEN, moves, analysis
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS games (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            profile_id INTEGER NOT NULL,
            initial_fen TEXT NOT NULL,
            final_fen TEXT NOT NULL,
            moves TEXT NOT NULL,
            result TEXT NOT NULL,
            player_color TEXT NOT NULL,
            opponent_type TEXT NOT NULL,
            opponent_elo INTEGER,
            analysis TEXT,
            mistakes INTEGER NOT NULL DEFAULT 0,
            blunders INTEGER NOT NULL DEFAULT 0,
            opening_name TEXT,
            created_at TEXT NOT NULL,
            finished_at TEXT,
            FOREIGN KEY (profile_id) REFERENCES profiles(id)
        );

        CREATE INDEX IF NOT EXISTS idx_games_profile_id ON games(profile_id);
        CREATE INDEX IF NOT EXISTS idx_games_created_at ON games(created_at);
        CREATE INDEX IF NOT EXISTS idx_games_opening_name ON games(opening_name);
        "#,
    )?;

    // Conversations table - chat sessions with coach
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS conversations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            profile_id INTEGER NOT NULL,
            title TEXT,
            context TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (profile_id) REFERENCES profiles(id)
        );

        CREATE INDEX IF NOT EXISTS idx_conversations_profile_id ON conversations(profile_id);
        "#,
    )?;

    // Messages table - individual chat messages
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            conversation_id INTEGER NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            tool_calls TEXT,
            tool_results TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY (conversation_id) REFERENCES conversations(id)
        );

        CREATE INDEX IF NOT EXISTS idx_messages_conversation_id ON messages(conversation_id);
        "#,
    )?;

    // Exercise results table - training attempt records
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS exercise_results (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            profile_id INTEGER NOT NULL,
            exercise_type TEXT NOT NULL,
            difficulty TEXT NOT NULL,
            position_fen TEXT NOT NULL,
            solved INTEGER NOT NULL,
            attempts INTEGER NOT NULL,
            time_seconds INTEGER NOT NULL,
            hints_used INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (profile_id) REFERENCES profiles(id)
        );

        CREATE INDEX IF NOT EXISTS idx_exercise_results_profile_id ON exercise_results(profile_id);
        CREATE INDEX IF NOT EXISTS idx_exercise_results_type ON exercise_results(exercise_type);
        "#,
    )?;

    // Settings table - key-value store for app settings
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tables() {
        let conn = Connection::open_in_memory().unwrap();
        assert!(create_tables(&conn).is_ok());

        // Verify tables exist
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        assert!(tables.contains(&"profiles".to_string()));
        assert!(tables.contains(&"games".to_string()));
        assert!(tables.contains(&"conversations".to_string()));
        assert!(tables.contains(&"messages".to_string()));
        assert!(tables.contains(&"exercise_results".to_string()));
        assert!(tables.contains(&"settings".to_string()));
    }
}
