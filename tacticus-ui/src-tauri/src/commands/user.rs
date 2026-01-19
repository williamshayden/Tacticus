use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i64,
    pub name: String,
    pub initial_level: String,
    pub current_elo: i32,
    pub peak_elo: i32,
    pub games_played: i32,
    pub exercises_completed: i32,
    pub streak: i32,
    pub style: String,
    pub weaknesses: Vec<String>,
    pub strengths: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetric {
    pub skill_type: String,
    pub proficiency: f32,
    pub samples: i32,
}

// In-memory state for now (will be replaced with SQLite)
lazy_static::lazy_static! {
    static ref USER_PROFILE: Mutex<Option<UserProfile>> = Mutex::new(None);
}

#[tauri::command]
pub fn get_user_profile() -> Option<UserProfile> {
    USER_PROFILE.lock().unwrap().clone()
}

#[tauri::command]
pub fn create_user_profile(name: String, initial_level: String) -> UserProfile {
    let initial_elo = match initial_level.as_str() {
        "beginner" => 600,
        "intermediate" => 1000,
        "advanced" => 1400,
        _ => {
            // Try to parse as custom ELO
            initial_level.parse().unwrap_or(800)
        }
    };
    
    let profile = UserProfile {
        id: 1,
        name,
        initial_level: initial_level.clone(),
        current_elo: initial_elo,
        peak_elo: initial_elo,
        games_played: 0,
        exercises_completed: 0,
        streak: 0,
        style: "Unknown".to_string(),
        weaknesses: vec![],
        strengths: vec![],
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    *USER_PROFILE.lock().unwrap() = Some(profile.clone());
    profile
}

#[tauri::command]
pub fn update_user_elo(new_elo: i32, game_result: String) -> UserProfile {
    let mut guard = USER_PROFILE.lock().unwrap();
    
    if let Some(ref mut profile) = *guard {
        profile.current_elo = new_elo;
        if new_elo > profile.peak_elo {
            profile.peak_elo = new_elo;
        }
        
        if game_result == "win" || game_result == "exercise_correct" {
            profile.streak += 1;
        } else {
            profile.streak = 0;
        }
        
        if game_result == "win" || game_result == "loss" || game_result == "draw" {
            profile.games_played += 1;
        }
        
        if game_result == "exercise_correct" || game_result == "exercise_incorrect" {
            profile.exercises_completed += 1;
        }
        
        profile.clone()
    } else {
        // Create default profile if none exists
        create_user_profile("Player".to_string(), "beginner".to_string())
    }
}

#[tauri::command]
pub fn get_user_stats() -> Option<UserStats> {
    let guard = USER_PROFILE.lock().unwrap();
    
    guard.as_ref().map(|profile| UserStats {
        current_elo: profile.current_elo,
        peak_elo: profile.peak_elo,
        games_played: profile.games_played,
        exercises_completed: profile.exercises_completed,
        streak: profile.streak,
        style: profile.style.clone(),
        exercises_until_calibration: 10 - (profile.exercises_completed % 10),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    pub current_elo: i32,
    pub peak_elo: i32,
    pub games_played: i32,
    pub exercises_completed: i32,
    pub streak: i32,
    pub style: String,
    pub exercises_until_calibration: i32,
}

#[tauri::command]
pub fn calculate_new_elo(user_elo: i32, opponent_elo: i32, result: f32) -> i32 {
    // Standard ELO calculation
    // result: 1.0 = win, 0.5 = draw, 0.0 = loss
    let k = 32; // K-factor
    let expected = 1.0 / (1.0 + 10.0_f32.powf((opponent_elo - user_elo) as f32 / 400.0));
    let new_elo = user_elo as f32 + k as f32 * (result - expected);
    new_elo.round() as i32
}

#[tauri::command]
pub fn save_api_key(api_key: String) -> Result<(), String> {
    // Save to .env file
    use std::io::Write;
    let path = std::path::Path::new(".env");
    let mut file = std::fs::File::create(path)
        .map_err(|e| format!("Failed to create .env file: {}", e))?;
    writeln!(file, "OPENROUTER_API_KEY={}", api_key)
        .map_err(|e| format!("Failed to write to .env file: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn get_api_key() -> Option<String> {
    dotenv::dotenv().ok();
    std::env::var("OPENROUTER_API_KEY").ok()
}

#[tauri::command]
pub fn has_completed_onboarding() -> bool {
    USER_PROFILE.lock().unwrap().is_some()
}
