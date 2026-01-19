use chess_trainer::{Exercise, ExerciseLibrary, ExerciseDifficulty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExerciseData {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub exercise_type: String,
    pub fen: String,
    pub hints: Vec<String>,
    pub solution_moves: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingSessionData {
    pub exercises: Vec<ExerciseData>,
    pub focus_areas: Vec<String>,
    pub total_exercises: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExerciseResult {
    pub correct: bool,
    pub explanation: String,
    pub correct_move: Option<String>,
}

fn exercise_to_data(exercise: &Exercise, id: usize) -> ExerciseData {
    ExerciseData {
        id,
        title: exercise.title.clone(),
        description: exercise.description.clone(),
        difficulty: match exercise.difficulty {
            ExerciseDifficulty::Beginner => "Beginner".to_string(),
            ExerciseDifficulty::Intermediate => "Intermediate".to_string(),
            ExerciseDifficulty::Advanced => "Advanced".to_string(),
            ExerciseDifficulty::Expert => "Expert".to_string(),
        },
        exercise_type: format!("{:?}", exercise.exercise_type),
        fen: exercise.position.clone(),
        hints: exercise.hints.clone(),
        solution_moves: exercise.solution_moves.clone(),
    }
}

#[tauri::command]
pub fn get_training_exercises(count: usize, _user_elo: i32, weaknesses: Vec<String>) -> TrainingSessionData {
    // Get all exercises
    let all_exercises = ExerciseLibrary::get_all_exercises();
    
    // For now, just return the first N exercises
    // Later this will be adaptive based on user weaknesses and ELO
    let exercises: Vec<ExerciseData> = all_exercises
        .iter()
        .take(count)
        .enumerate()
        .map(|(i, e)| exercise_to_data(e, i))
        .collect();
    
    let focus_areas = if weaknesses.is_empty() {
        vec!["General tactics".to_string(), "Pattern recognition".to_string()]
    } else {
        weaknesses
    };
    
    TrainingSessionData {
        total_exercises: exercises.len(),
        exercises,
        focus_areas,
    }
}

#[tauri::command]
pub fn check_exercise_solution(exercise_id: usize, user_move: String) -> ExerciseResult {
    let all_exercises = ExerciseLibrary::get_all_exercises();
    
    if let Some(exercise) = all_exercises.get(exercise_id) {
        let is_correct = exercise.check_solution(&user_move);
        
        ExerciseResult {
            correct: is_correct,
            explanation: if is_correct {
                exercise.explanation.clone()
            } else {
                format!("Not quite! {}", exercise.hints.first().unwrap_or(&"Try again.".to_string()))
            },
            correct_move: if !is_correct {
                exercise.solution_moves.first().cloned()
            } else {
                None
            },
        }
    } else {
        ExerciseResult {
            correct: false,
            explanation: "Exercise not found".to_string(),
            correct_move: None,
        }
    }
}

#[tauri::command]
pub fn get_exercise_hint(exercise_id: usize, hint_index: usize) -> Option<String> {
    let all_exercises = ExerciseLibrary::get_all_exercises();
    
    all_exercises
        .get(exercise_id)
        .and_then(|e| e.hints.get(hint_index).cloned())
}

#[tauri::command]
pub fn get_all_exercise_types() -> Vec<String> {
    vec![
        "Tactics Blitz".to_string(),
        "Guided Discovery".to_string(),
        "Find the Plan".to_string(),
        "Blind Spots".to_string(),
        "Pattern Recognition".to_string(),
        "Calculation Ladder".to_string(),
        "Endgame Drills".to_string(),
        "Opening Traps".to_string(),
    ]
}
