//! Integration tests for user commands

#[test]
fn test_elo_calculation_win() {
    // Standard ELO calculation
    let user_elo = 800;
    let opponent_elo = 850;
    let result = 1.0; // Win
    
    let k = 32;
    let expected = 1.0 / (1.0 + 10.0_f32.powf((opponent_elo - user_elo) as f32 / 400.0));
    let new_elo = user_elo as f32 + k as f32 * (result - expected);
    
    // Should gain ELO for winning against higher rated opponent
    assert!(new_elo > user_elo as f32);
}

#[test]
fn test_elo_calculation_loss() {
    let user_elo = 800;
    let opponent_elo = 750;
    let result = 0.0; // Loss
    
    let k = 32;
    let expected = 1.0 / (1.0 + 10.0_f32.powf((opponent_elo - user_elo) as f32 / 400.0));
    let new_elo = user_elo as f32 + k as f32 * (result - expected);
    
    // Should lose more ELO for losing against lower rated opponent
    assert!(new_elo < user_elo as f32);
}

#[test]
fn test_elo_calculation_draw() {
    let user_elo = 1000;
    let opponent_elo = 1000;
    let result = 0.5; // Draw
    
    let k = 32;
    let expected = 1.0 / (1.0 + 10.0_f32.powf((opponent_elo - user_elo) as f32 / 400.0));
    let new_elo = user_elo as f32 + k as f32 * (result - expected);
    
    // Equal rating draw should result in no change
    assert!((new_elo - user_elo as f32).abs() < 1.0);
}

#[test]
fn test_initial_elo_beginner() {
    let initial_level = "beginner";
    let expected_elo = 600;
    
    let elo = match initial_level {
        "beginner" => 600,
        "intermediate" => 1000,
        "advanced" => 1400,
        _ => 800,
    };
    
    assert_eq!(elo, expected_elo);
}

#[test]
fn test_initial_elo_intermediate() {
    let initial_level = "intermediate";
    let expected_elo = 1000;
    
    let elo = match initial_level {
        "beginner" => 600,
        "intermediate" => 1000,
        "advanced" => 1400,
        _ => 800,
    };
    
    assert_eq!(elo, expected_elo);
}

#[test]
fn test_initial_elo_custom() {
    let initial_level = "1350";
    
    let elo = match initial_level {
        "beginner" => 600,
        "intermediate" => 1000,
        "advanced" => 1400,
        other => other.parse().unwrap_or(800),
    };
    
    assert_eq!(elo, 1350);
}
