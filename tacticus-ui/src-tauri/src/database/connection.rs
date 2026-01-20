use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

use super::schema;

/// Global database connection wrapped in a Mutex for thread-safe access
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Create a new database connection. Creates the database file and directory if needed.
    pub fn new() -> Result<Self> {
        let db_path = Self::get_database_path();

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(&db_path)?;

        // Enable foreign keys
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        let db = Self {
            conn: Mutex::new(conn),
        };

        // Initialize schema
        db.init_schema()?;

        Ok(db)
    }

    /// Create an in-memory database (for testing)
    #[allow(dead_code)]
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        let db = Self {
            conn: Mutex::new(conn),
        };

        db.init_schema()?;

        Ok(db)
    }

    /// Get the path to the database file
    fn get_database_path() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("tacticus")
            .join("tacticus.db")
    }

    /// Initialize the database schema
    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        schema::create_tables(&conn)
    }

    /// Execute a function with a reference to the connection
    pub fn with_conn<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>,
    {
        let conn = self.conn.lock().unwrap();
        f(&conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let db = Database::new_in_memory().expect("Failed to create in-memory database");
        assert!(db.with_conn(|_| Ok(())).is_ok());
    }
}
