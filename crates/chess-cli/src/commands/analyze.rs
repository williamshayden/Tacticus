use anyhow::Result;
use chess_storage::{Database, GameRepository};
use chess_engine::GameAnalyzer;
use chess_core::MoveQuality;
use crate::ui;

pub async fn run(user_id: u64) -> Result<()> {
    ui::print_header("Game Analysis");

    let db = Database::new("sqlite:chess_trainer.db").await?;
    let game_repo = GameRepository::new(db.pool());

    let games = game_repo.get_user_games(user_id).await?;

    if games.is_empty() {
        ui::print_warning("No games found for this user.");
        ui::print_info("Play a game first with: chess-trainer play");
        return Ok(());
    }

    ui::print_success(&format!("Found {} games", games.len()));

    // Analyze the most recent game
    let game = &games[0];
    ui::print_info("Analyzing most recent game...");

    let analyses = GameAnalyzer::analyze_game(game);

    println!("\n{}", "Move Analysis:".cyan().bold());
    println!("{:<6} {:<12} {:<10} {:<12} {:<8}", "Move", "Your Move", "Quality", "Best Move", "Loss");
    println!("{}", "-".repeat(60));

    for (i, analysis) in analyses.iter().enumerate() {
        let move_num = (i / 2) + 1;
        let color = if i % 2 == 0 { "White" } else { "Black" };

        let quality_str = match analysis.quality {
            MoveQuality::Brilliant => "!!".bright_green(),
            MoveQuality::Great => "!".green(),
            MoveQuality::Good => "".normal(),
            MoveQuality::Inaccuracy => "?!".yellow(),
            MoveQuality::Mistake => "?".red(),
            MoveQuality::Blunder => "??".bright_red(),
        };

        let is_best = if analysis.chess_move == analysis.best_move {
            "✓".green()
        } else {
            "".normal()
        };

        println!(
            "{:<6} {:<12} {:<10} {:<12} {:>5} {}",
            format!("{}.{}", move_num, if color == "White" { "" } else { ".." }),
            format!("{}", analysis.chess_move),
            quality_str,
            format!("{}", analysis.best_move),
            analysis.centipawn_loss,
            is_best
        );

        if analysis.centipawn_loss > 100 {
            println!("       └─ {}", analysis.comment.yellow());
        }
    }

    // Identify weaknesses
    let weaknesses = GameAnalyzer::identify_weaknesses(&analyses);

    println!("\n{}", "Key Insights:".cyan().bold());
    for weakness in weaknesses {
        println!("  • {}", weakness);
    }

    // Calculate statistics
    let brilliant_count = analyses.iter().filter(|a| a.quality == MoveQuality::Brilliant).count();
    let blunder_count = analyses.iter().filter(|a| a.quality == MoveQuality::Blunder).count();
    let avg_loss: i32 = analyses.iter().map(|a| a.centipawn_loss).sum::<i32>() / analyses.len().max(1) as i32;

    println!("\n{}", "Statistics:".cyan().bold());
    println!("  Total Moves: {}", analyses.len());
    println!("  Brilliant Moves: {}", brilliant_count);
    println!("  Blunders: {}", blunder_count);
    println!("  Average Centipawn Loss: {}", avg_loss);

    Ok(())
}

use colored::*;
