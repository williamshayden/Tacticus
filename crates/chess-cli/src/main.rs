mod commands;
mod ui;
mod game_loop;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "chess-trainer")]
#[command(about = "An adaptive chess training application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new training session
    Train {
        /// User ID for tracking progress
        #[arg(short, long, default_value = "1")]
        user_id: u64,
    },
    /// Play a practice game
    Play {
        /// User ID for tracking progress
        #[arg(short, long, default_value = "1")]
        user_id: u64,
        /// Play as white or black
        #[arg(short, long, default_value = "white")]
        color: String,
    },
    /// View your player profile
    Profile {
        /// User ID
        #[arg(short, long, default_value = "1")]
        user_id: u64,
    },
    /// Analyze a completed game
    Analyze {
        /// User ID
        #[arg(short, long, default_value = "1")]
        user_id: u64,
    },
    /// Initialize the database
    Init,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Train { user_id } => {
            commands::train::run(user_id).await?;
        }
        Commands::Play { user_id, color } => {
            commands::play::run(user_id, &color).await?;
        }
        Commands::Profile { user_id } => {
            commands::profile::run(user_id).await?;
        }
        Commands::Analyze { user_id } => {
            commands::analyze::run(user_id).await?;
        }
        Commands::Init => {
            commands::init::run().await?;
        }
    }

    Ok(())
}
