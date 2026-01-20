use serde::{Deserialize, Serialize};
use crate::DB;
use crate::database::repositories::{self, Profile};

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

impl From<Profile> for UserProfile {
    fn from(p: Profile) -> Self {
        UserProfile {
            id: p.id,
            name: p.name,
            initial_level: p.initial_level,
            current_elo: p.current_elo,
            peak_elo: p.peak_elo,
            games_played: p.games_played,
            exercises_completed: p.exercises_completed,
            streak: p.streak,
            style: p.style,
            weaknesses: p.weaknesses,
            strengths: p.strengths,
            created_at: p.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub current_elo: i32,
    pub peak_elo: i32,
    pub games_played: i32,
    pub exercises_completed: i32,
    pub streak: i32,
    pub style: String,
    pub exercises_until_calibration: i32,
}

// Initialize API key from database or file on startup
pub fn init_api_key() {
    // First check database
    if let Ok(Some(key)) = DB.with_conn(|conn| repositories::get_setting(conn, "api_key")) {
        if !key.is_empty() {
            std::env::set_var("OPENROUTER_API_KEY", &key);
            return;
        }
    }

    // Fall back to file-based storage for migration
    if let Some(path) = dirs::config_dir().map(|p| p.join("Tacticus").join("api_key")) {
        if path.exists() {
            if let Ok(key) = std::fs::read_to_string(&path) {
                let key = key.trim().to_string();
                if !key.is_empty() {
                    std::env::set_var("OPENROUTER_API_KEY", &key);
                    // Migrate to database
                    let _ = DB.with_conn(|conn| repositories::set_setting(conn, "api_key", &key));
                }
            }
        }
    }

    // Check .env file for backwards compatibility
    dotenv::dotenv().ok();
}

// Initialize profile from database on startup
pub fn init_profile() {
    // Check if there's already a profile in the database
    if let Ok(Some(_)) = DB.with_conn(|conn| repositories::get_first_profile(conn)) {
        return; // Profile exists
    }

    // Try to migrate from file-based storage
    if let Some(path) = dirs::config_dir().map(|p| p.join("Tacticus").join("profile.json")) {
        if path.exists() {
            if let Ok(data) = std::fs::read_to_string(&path) {
                if let Ok(old_profile) = serde_json::from_str::<serde_json::Value>(&data) {
                    // Migrate old profile to database
                    let name = old_profile["name"].as_str().unwrap_or("Player");
                    let initial_level = old_profile["initial_level"].as_str().unwrap_or("beginner");
                    let current_elo = old_profile["current_elo"].as_i64().unwrap_or(800) as i32;

                    if let Ok(profile) = DB.with_conn(|conn| {
                        repositories::create_profile(conn, name, initial_level, current_elo)
                    }) {
                        // Update with additional fields from old profile
                        let mut updated = profile;
                        updated.peak_elo = old_profile["peak_elo"].as_i64().unwrap_or(updated.current_elo as i64) as i32;
                        updated.games_played = old_profile["games_played"].as_i64().unwrap_or(0) as i32;
                        updated.exercises_completed = old_profile["exercises_completed"].as_i64().unwrap_or(0) as i32;
                        updated.streak = old_profile["streak"].as_i64().unwrap_or(0) as i32;
                        updated.style = old_profile["style"].as_str().unwrap_or("Unknown").to_string();

                        let _ = DB.with_conn(|conn| repositories::update_profile(conn, &updated));
                    }
                }
            }
        }
    }
}

#[tauri::command]
pub fn create_user_profile(name: String, initial_level: String) -> Result<UserProfile, String> {
    let elo = match initial_level.as_str() {
        "beginner" => 600,
        "intermediate" => 1000,
        "advanced" => 1400,
        other => other.parse().unwrap_or(800),
    };

    let profile = DB
        .with_conn(|conn| repositories::create_profile(conn, &name, &initial_level, elo))
        .map_err(|e| format!("Failed to create profile: {}", e))?;

    Ok(profile.into())
}

#[tauri::command]
pub fn get_user_profile() -> Option<UserProfile> {
    DB.with_conn(|conn| repositories::get_first_profile(conn))
        .ok()
        .flatten()
        .map(|p| p.into())
}

#[tauri::command]
pub fn get_user_stats() -> Option<UserStats> {
    let profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .ok()
        .flatten()?;

    Some(UserStats {
        current_elo: profile.current_elo,
        peak_elo: profile.peak_elo,
        games_played: profile.games_played,
        exercises_completed: profile.exercises_completed,
        streak: profile.streak,
        style: profile.style,
        exercises_until_calibration: 10 - (profile.exercises_completed % 10),
    })
}

#[tauri::command]
pub fn update_user_elo(new_elo: i32, game_result: String) -> Result<UserProfile, String> {
    let mut profile = DB
        .with_conn(|conn| repositories::get_first_profile(conn))
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No user profile found".to_string())?;

    profile.current_elo = new_elo;
    profile.peak_elo = profile.peak_elo.max(new_elo);
    profile.games_played += 1;

    if game_result == "win" {
        profile.streak += 1;
    } else {
        profile.streak = 0;
    }

    DB.with_conn(|conn| repositories::update_profile(conn, &profile))
        .map_err(|e| format!("Failed to update profile: {}", e))?;

    Ok(profile.into())
}

#[tauri::command]
pub fn save_api_key(api_key: String) -> Result<(), String> {
    // Save to database
    DB.with_conn(|conn| repositories::set_setting(conn, "api_key", &api_key))
        .map_err(|e| format!("Failed to save API key: {}", e))?;

    // Also set as environment variable for immediate use
    std::env::set_var("OPENROUTER_API_KEY", &api_key);

    Ok(())
}

#[tauri::command]
pub fn get_api_key() -> Option<String> {
    // First check environment
    if let Ok(key) = std::env::var("OPENROUTER_API_KEY") {
        if !key.is_empty() {
            return Some(key);
        }
    }

    // Then check database
    DB.with_conn(|conn| repositories::get_setting(conn, "api_key"))
        .ok()
        .flatten()
}

#[tauri::command]
pub fn has_completed_onboarding() -> bool {
    DB.with_conn(|conn| repositories::get_first_profile(conn))
        .ok()
        .flatten()
        .is_some()
}

pub fn calculate_new_elo(user_elo: i32, opponent_elo: i32, result: f32) -> i32 {
    let k = 32;
    let expected = 1.0 / (1.0 + 10.0_f32.powf((opponent_elo - user_elo) as f32 / 400.0));
    let new_elo = user_elo as f32 + k as f32 * (result - expected);
    new_elo.round() as i32
}
