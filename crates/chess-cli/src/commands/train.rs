use anyhow::Result;
use chess_storage::{Database, ProfileRepository, SessionRepository};
use chess_ai::{LearningAgent, PlayerProfile};
use chess_trainer::{ExerciseResult, ExerciseDifficulty};
use crate::ui;
use std::time::Instant;

pub async fn run(user_id: u64) -> Result<()> {
    ui::print_header("Chess Training Session");

    let db = Database::new("sqlite:chess_trainer.db").await?;
    let profile_repo = ProfileRepository::new(db.pool());
    let session_repo = SessionRepository::new(db.pool());

    // Load or create user profile
    let profile = match profile_repo.get(user_id).await? {
        Some(profile) => {
            ui::print_success(&format!("Welcome back, Player {}!", user_id));
            profile
        }
        None => {
            ui::print_info(&format!("Creating new profile for Player {}...", user_id));
            let profile = PlayerProfile::new(user_id);
            profile_repo.create(&profile).await?;
            profile
        }
    };

    // Create learning agent
    let mut agent = LearningAgent::from_profile(profile);

    // Display profile info
    ui::print_info(&format!("Skill Level: {:?}", agent.get_profile().skill_level));
    ui::print_info(&format!("Rating: {}", agent.get_profile().estimated_rating));
    ui::print_info(&format!("Play Style: {:?}", agent.get_profile().play_style));

    if !agent.get_profile().weaknesses.is_empty() {
        ui::print_warning(&format!(
            "Focus areas: {}",
            agent.get_profile().weaknesses.join(", ")
        ));
    }

    println!();

    // Create training session
    let mut session = agent.create_training_session();
    ui::print_success(&format!("Prepared {} exercises for you!", session.exercises.len()));

    println!();

    // Run through exercises
    while let Some(exercise) = session.current_exercise() {
        let exercise_num = session.current_exercise_index + 1;
        let total = session.exercises.len();

        ui::print_header(&format!("Exercise {}/{}: {}", exercise_num, total, exercise.title));

        println!("{}", exercise.description);
        println!("\nDifficulty: {:?}", exercise.difficulty);
        println!("Type: {:?}", exercise.exercise_type);
        println!("\nPosition (FEN): {}", exercise.position);

        if let Ok(board) = exercise.get_board() {
            ui::print_board(&board, chess::Color::White);
        }

        let start_time = Instant::now();
        let mut attempts = 0;
        let mut hints_used = 0;
        let mut solved = false;

        loop {
            let input = ui::get_user_input("\nYour move (or 'hint' for help, 'skip' to skip, 'quit' to finish):");

            if input.to_lowercase() == "quit" {
                ui::print_warning("Ending training session...");
                session.finish();
                break;
            }

            if input.to_lowercase() == "skip" {
                ui::print_warning("Skipping exercise...");
                break;
            }

            if input.to_lowercase() == "hint" {
                if hints_used < exercise.hints.len() {
                    ui::print_info(&format!("Hint: {}", exercise.hints[hints_used]));
                    hints_used += 1;
                } else if !exercise.hints.is_empty() {
                    ui::print_warning("No more hints available!");
                } else {
                    ui::print_info(&format!("Hint: {}", exercise.explanation));
                }
                continue;
            }

            attempts += 1;

            if exercise.check_solution(&input) {
                ui::print_success("Correct! Well done!");
                println!("\n{}", exercise.explanation);
                solved = true;
                break;
            } else {
                ui::print_error("Incorrect. Try again!");
            }
        }

        let time_taken = start_time.elapsed().as_secs() as u32;

        let result = ExerciseResult {
            exercise_id: exercise.id.unwrap_or(exercise_num as u64),
            user_id,
            solved,
            attempts,
            time_taken_seconds: time_taken,
            hints_used: hints_used as u32,
            completed_at: chrono::Utc::now(),
        };

        session.record_result(result);

        if session.is_finished() {
            break;
        }

        session.next_exercise();

        println!();
        let proceed = ui::get_user_input("Continue to next exercise? (y/n):");
        if proceed.to_lowercase() != "y" && proceed.to_lowercase() != "yes" {
            break;
        }
    }

    // Finish session
    session.finish();
    let session_result = session.get_session_result();

    ui::print_header("Training Session Complete!");
    println!("{}", session_result.summary());

    // Update agent with session results
    agent.update_from_training_session(&session);

    // Save session and updated profile
    session_repo.create(&session).await?;
    profile_repo.update(agent.get_profile()).await?;

    ui::print_success("Progress saved!");

    Ok(())
}
