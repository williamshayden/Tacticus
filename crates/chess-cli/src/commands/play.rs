use anyhow::Result;
use chess::Color;
use chess_storage::{Database, ProfileRepository, GameRepository};
use chess_ai::{LearningAgent, PlayerProfile};
use crate::{ui, game_loop};

pub async fn run(user_id: u64, color_str: &str) -> Result<()> {
    let player_color = match color_str.to_lowercase().as_str() {
        "white" | "w" => Color::White,
        "black" | "b" => Color::Black,
        _ => {
            ui::print_error("Invalid color! Using white.");
            Color::White
        }
    };

    let db = Database::new("sqlite:chess_trainer.db").await?;
    let profile_repo = ProfileRepository::new(db.pool());
    let game_repo = GameRepository::new(db.pool());

    // Load or create user profile
    let profile = match profile_repo.get(user_id).await? {
        Some(profile) => profile,
        None => {
            ui::print_info(&format!("Creating new profile for Player {}...", user_id));
            let profile = PlayerProfile::new(user_id);
            profile_repo.create(&profile).await?;
            profile
        }
    };

    let mut agent = LearningAgent::from_profile(profile);

    // Play the game
    let game = game_loop::play_game(player_color).await?;

    // Analyze the game
    ui::print_header("Game Analysis");
    ui::print_info("Analyzing your game...");

    let recommendation = agent.analyze_game(game.clone());

    println!("\n{}", recommendation.personalized_message);

    if !recommendation.strengths_identified.is_empty() {
        println!("\n{}", "Strengths:".green().bold());
        for strength in &recommendation.strengths_identified {
            println!("  ✓ {}", strength.green());
        }
    }

    if !recommendation.weaknesses_identified.is_empty() {
        println!("\n{}", "Areas for Improvement:".yellow().bold());
        for weakness in &recommendation.weaknesses_identified {
            println!("  • {}", weakness.yellow());
        }
    }

    println!("\n{}", "Recommendations:".cyan().bold());
    println!("  • Complete {} exercises", recommendation.recommended_exercises);
    println!("  • Difficulty: {:?}", recommendation.recommended_difficulty);
    println!("  • Focus on: {}", recommendation.focus_areas.join(", "));

    // Save game and updated profile
    game_repo.create(&game).await?;
    profile_repo.update(agent.get_profile()).await?;

    ui::print_success("Game saved and profile updated!");

    println!("\nRun 'chess-trainer train' to start a personalized training session!");

    Ok(())
}

use colored::*;
