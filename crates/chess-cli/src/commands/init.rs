use anyhow::Result;
use chess_storage::Database;
use crate::ui;

pub async fn run() -> Result<()> {
    ui::print_header("Initializing Chess Trainer Database");

    let db = Database::new("sqlite:chess_trainer.db").await?;
    db.init_schema().await?;

    ui::print_success("Database initialized successfully!");
    ui::print_info("Database file: chess_trainer.db");

    Ok(())
}
