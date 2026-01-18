use anyhow::Result;
use chess_storage::{Database, ProfileRepository};
use crate::ui;

pub async fn run(user_id: u64) -> Result<()> {
    ui::print_header("Player Profile");

    let db = Database::new("sqlite:chess_trainer.db").await?;
    let profile_repo = ProfileRepository::new(db.pool());

    match profile_repo.get(user_id).await? {
        Some(profile) => {
            println!("{}", profile.summary());

            println!("\n{}", "Style Characteristics:".cyan().bold());
            println!("  Aggression: {:.1}%", profile.style_characteristics.aggression_score * 100.0);
            println!("  Tactical: {:.1}%", profile.style_characteristics.tactical_score * 100.0);
            println!("  Positional: {:.1}%", profile.style_characteristics.positional_score * 100.0);
            println!("  Risk Taking: {:.1}%", profile.style_characteristics.risk_taking_score * 100.0);
            println!("  Accuracy: {:.1}%", profile.style_characteristics.accuracy_score * 100.0);
        }
        None => {
            ui::print_error(&format!("Profile not found for user {}", user_id));
            ui::print_info("Run 'chess-trainer init' to initialize the database");
            ui::print_info("Then play a game or start training to create your profile");
        }
    }

    Ok(())
}

use colored::*;
