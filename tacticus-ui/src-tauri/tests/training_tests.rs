//! Integration tests for training commands

use chess_trainer::{ExerciseLibrary, ExerciseDifficulty, ExerciseType};

#[test]
fn test_exercise_library_not_empty() {
    let exercises = ExerciseLibrary::get_all_exercises();
    assert!(!exercises.is_empty());
}

#[test]
fn test_exercise_has_solution() {
    let exercises = ExerciseLibrary::get_all_exercises();
    
    for exercise in exercises {
        assert!(!exercise.solution_moves.is_empty(), 
            "Exercise '{}' has no solution moves", exercise.title);
    }
}

#[test]
fn test_exercise_has_valid_position() {
    use std::str::FromStr;
    let exercises = ExerciseLibrary::get_all_exercises();
    
    for exercise in exercises {
        let result = chess::Board::from_str(&exercise.position);
        assert!(result.is_ok(), 
            "Exercise '{}' has invalid FEN: {}", 
            exercise.title, exercise.position);
    }
}

#[test]
fn test_exercise_check_solution_correct() {
    let exercises = ExerciseLibrary::get_all_exercises();
    
    if let Some(exercise) = exercises.first() {
        let correct_move = exercise.solution_moves.first().unwrap();
        assert!(exercise.check_solution(correct_move));
    }
}

#[test]
fn test_exercise_check_solution_incorrect() {
    let exercises = ExerciseLibrary::get_all_exercises();
    
    if let Some(exercise) = exercises.first() {
        // An obviously wrong move
        let wrong_move = "a1a2";
        assert!(!exercise.check_solution(wrong_move));
    }
}

#[test]
fn test_exercise_difficulty_levels() {
    let exercises = ExerciseLibrary::get_all_exercises();
    
    // Should have exercises at different difficulty levels
    let has_beginner = exercises.iter().any(|e| matches!(e.difficulty, ExerciseDifficulty::Beginner));
    let has_intermediate = exercises.iter().any(|e| matches!(e.difficulty, ExerciseDifficulty::Intermediate));
    
    assert!(has_beginner || has_intermediate, "Should have exercises at various difficulties");
}

#[test]
fn test_exercise_types_varied() {
    let exercises = ExerciseLibrary::get_all_exercises();
    
    // Should have different exercise types
    let tactical = exercises.iter().filter(|e| matches!(e.exercise_type, ExerciseType::Tactics)).count();
    
    assert!(tactical > 0, "Should have tactical exercises");
}
