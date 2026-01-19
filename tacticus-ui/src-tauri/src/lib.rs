mod commands;

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize stored data on startup
    commands::user::init_api_key();
    commands::user::init_profile();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Game commands
            get_initial_position,
            get_legal_moves,
            make_move,
            get_engine_move,
            evaluate_position,
            get_position_from_fen,
            // Training commands
            get_training_exercises,
            check_exercise_solution,
            get_exercise_hint,
            get_all_exercise_types,
            // Coach commands
            get_coach_greeting,
            chat_with_coach,
            get_position_feedback,
            analyze_position_with_coach,
            check_api_key_configured,
            // User commands
            get_user_profile,
            create_user_profile,
            update_user_elo,
            get_user_stats,
            save_api_key,
            get_api_key,
            has_completed_onboarding,
            // Learning commands
            get_all_concepts,
            get_concept,
            get_concepts_by_category,
            search_concepts,
            get_concept_categories,
            define_term,
            get_related_concepts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
