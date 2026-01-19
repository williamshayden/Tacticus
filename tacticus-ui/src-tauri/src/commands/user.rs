use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::path::PathBuf;
use std::fs;

lazy_static::lazy_static! {
    static ref USER_PROFILE: Mutex<Option<UserProfile>> = Mutex::new(None);
    static ref API_KEY: Mutex<Option<String>> = Mutex::new(None);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: u32,
    pub name: String,
    pub initial_level: String,
    pub current_elo: i32,
    pub peak_elo: i32,
    pub games_played: u32,
    pub exercises_completed: u32,
    pub streak: u32,
    pub style: String,
    pub weaknesses: Vec<String>,
    pub strengths: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetric {
    pub name: String,
    pub value: f32,
    pub trend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub current_elo: i32,
    pub peak_elo: i32,
    pub games_played: u32,
    pub exercises_completed: u32,
    pub streak: u32,
    pub style: String,
    pub exercises_until_calibration: u32,
}

fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("Tacticus"))
}

fn ensure_config_dir() -> Result<PathBuf, String> {
    let config_dir = get_config_dir()
        .ok_or_else(|| "Could not determine config directory".to_string())?;
    
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    
    Ok(config_dir)
}

fn get_api_key_path() -> Option<PathBuf> {
    get_config_dir().map(|p| p.join("api_key"))
}

fn get_profile_path() -> Option<PathBuf> {
    get_config_dir().map(|p| p.join("profile.json"))
}

// Initialize API key from file on startup
pub fn init_api_key() {
    if let Some(path) = get_api_key_path() {
        if path.exists() {
            if let Ok(key) = fs::read_to_string(&path) {
                let key = key.trim().to_string();
                if !key.is_empty() {
                    // Also set as environment variable for the LLM agent
                    std::env::set_var("OPENROUTER_API_KEY", &key);
                    *API_KEY.lock().unwrap() = Some(key);
                }
            }
        }
    }
    
    // Also check .env file for backwards compatibility
    dotenv::dotenv().ok();
    if API_KEY.lock().unwrap().is_none() {
        if let Ok(key) = std::env::var("OPENROUTER_API_KEY") {
            *API_KEY.lock().unwrap() = Some(key);
        }
    }
}

// Initialize profile from file on startup
pub fn init_profile() {
    if let Some(path) = get_profile_path() {
        if path.exists() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Ok(profile) = serde_json::from_str::<UserProfile>(&data) {
                    *USER_PROFILE.lock().unwrap() = Some(profile);
                }
            }
        }
    }
}

fn save_profile_to_file(profile: &UserProfile) -> Result<(), String> {
    let config_dir = ensure_config_dir()?;
    let path = config_dir.join("profile.json");
    
    let data = serde_json::to_string_pretty(profile)
        .map_err(|e| format!("Failed to serialize profile: {}", e))?;
    
    fs::write(&path, data)
        .map_err(|e| format!("Failed to save profile: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub fn create_user_profile(name: String, initial_level: String) -> Result<UserProfile, String> {
    let elo = match initial_level.as_str() {
        "beginner" => 600,
        "intermediate" => 1000,
        "advanced" => 1400,
        other => other.parse().unwrap_or(800),
    };

    let profile = UserProfile {
        id: 1,
        name,
        initial_level,
        current_elo: elo,
        peak_elo: elo,
        games_played: 0,
        exercises_completed: 0,
        streak: 0,
        style: "Unknown".to_string(),
        weaknesses: vec![],
        strengths: vec![],
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    // Save to file
    save_profile_to_file(&profile)?;
    
    *USER_PROFILE.lock().unwrap() = Some(profile.clone());
    Ok(profile)
}

#[tauri::command]
pub fn get_user_profile() -> Option<UserProfile> {
    USER_PROFILE.lock().unwrap().clone()
}

#[tauri::command]
pub fn get_user_stats() -> Option<UserStats> {
    let profile = USER_PROFILE.lock().unwrap();
    profile.as_ref().map(|p| UserStats {
        current_elo: p.current_elo,
        peak_elo: p.peak_elo,
        games_played: p.games_played,
        exercises_completed: p.exercises_completed,
        streak: p.streak,
        style: p.style.clone(),
        exercises_until_calibration: 10 - (p.exercises_completed % 10),
    })
}

#[tauri::command]
pub fn update_user_elo(new_elo: i32, game_result: String) -> Result<UserProfile, String> {
    let mut profile_lock = USER_PROFILE.lock().unwrap();
    let profile = profile_lock
        .as_mut()
        .ok_or_else(|| "No user profile found".to_string())?;

    profile.current_elo = new_elo;
    profile.peak_elo = profile.peak_elo.max(new_elo);
    profile.games_played += 1;

    if game_result == "win" {
        profile.streak += 1;
    } else {
        profile.streak = 0;
    }

    let result = profile.clone();
    
    // Save to file
    drop(profile_lock);
    save_profile_to_file(&result)?;
    
    Ok(result)
}

pub fn calculate_new_elo(user_elo: i32, opponent_elo: i32, result: f32) -> i32 {
    let k = 32;
    let expected = 1.0 / (1.0 + 10.0_f32.powf((opponent_elo - user_elo) as f32 / 400.0));
    let new_elo = user_elo as f32 + k as f32 * (result - expected);
    new_elo.round() as i32
}

#[tauri::command]
pub fn save_api_key(api_key: String) -> Result<(), String> {
    let config_dir = ensure_config_dir()?;
    let path = config_dir.join("api_key");
    
    fs::write(&path, &api_key)
        .map_err(|e| format!("Failed to save API key: {}", e))?;
    
    // Also set as environment variable for immediate use
    std::env::set_var("OPENROUTER_API_KEY", &api_key);
    *API_KEY.lock().unwrap() = Some(api_key);
    
    Ok(())
}

#[tauri::command]
pub fn get_api_key() -> Option<String> {
    API_KEY.lock().unwrap().clone()
}

#[tauri::command]
pub fn has_completed_onboarding() -> bool {
    USER_PROFILE.lock().unwrap().is_some()
}
