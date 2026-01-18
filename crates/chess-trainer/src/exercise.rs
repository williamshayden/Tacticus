use chess::{Board, ChessMove, Color};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExerciseType {
    Tactics,           // Find the best tactical move
    Endgame,          // Practice endgame positions
    Opening,          // Learn opening principles
    Positional,       // Improve positional understanding
    Calculation,      // Calculate variations
    Strategy,         // Strategic planning
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ExerciseDifficulty {
    Beginner = 1,
    Intermediate = 2,
    Advanced = 3,
    Expert = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: Option<u64>,
    pub exercise_type: ExerciseType,
    pub difficulty: ExerciseDifficulty,
    pub position: String,        // FEN notation
    pub title: String,
    pub description: String,
    pub solution_moves: Vec<String>, // Best move(s) in algebraic notation
    pub hints: Vec<String>,
    pub explanation: String,
}

impl Exercise {
    pub fn new(
        exercise_type: ExerciseType,
        difficulty: ExerciseDifficulty,
        position: String,
        title: String,
        description: String,
        solution_moves: Vec<String>,
        explanation: String,
    ) -> Self {
        Self {
            id: None,
            exercise_type,
            difficulty,
            position,
            title,
            description,
            solution_moves,
            hints: Vec::new(),
            explanation,
        }
    }

    pub fn with_hints(mut self, hints: Vec<String>) -> Self {
        self.hints = hints;
        self
    }

    pub fn get_board(&self) -> Result<Board, String> {
        Board::from_str(&self.position)
            .map_err(|e| format!("Invalid FEN in exercise: {}", e))
    }

    pub fn check_solution(&self, user_move: &str) -> bool {
        self.solution_moves.iter().any(|sol| sol == user_move)
    }

    pub fn is_correct_move(&self, chess_move: ChessMove) -> bool {
        let move_str = format!("{}", chess_move);
        self.check_solution(&move_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseResult {
    pub exercise_id: u64,
    pub user_id: u64,
    pub solved: bool,
    pub attempts: u32,
    pub time_taken_seconds: u32,
    pub hints_used: u32,
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

impl ExerciseResult {
    pub fn new(exercise_id: u64, user_id: u64) -> Self {
        Self {
            exercise_id,
            user_id,
            solved: false,
            attempts: 0,
            time_taken_seconds: 0,
            hints_used: 0,
            completed_at: chrono::Utc::now(),
        }
    }
}

// Predefined exercises for different strategies
pub struct ExerciseLibrary;

impl ExerciseLibrary {
    pub fn get_tactical_exercises() -> Vec<Exercise> {
        vec![
            Exercise::new(
                ExerciseType::Tactics,
                ExerciseDifficulty::Beginner,
                "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4".to_string(),
                "Scholar's Mate Defense".to_string(),
                "White is threatening checkmate. How should Black defend?".to_string(),
                vec!["Qe7".to_string(), "Qf6".to_string()],
                "Black must defend against the checkmate threat on f7. Moving the Queen to e7 or f6 protects the f7 pawn.".to_string(),
            ),
            Exercise::new(
                ExerciseType::Tactics,
                ExerciseDifficulty::Intermediate,
                "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3".to_string(),
                "Control the Center".to_string(),
                "Find the best move to control the center.".to_string(),
                vec!["d4".to_string()],
                "d4 is the strongest move, immediately challenging Black's central pawn on e5 and opening lines for development.".to_string(),
            )
            .with_hints(vec!["Think about pawn breaks in the center.".to_string()]),
        ]
    }

    pub fn get_opening_exercises() -> Vec<Exercise> {
        vec![
            Exercise::new(
                ExerciseType::Opening,
                ExerciseDifficulty::Beginner,
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
                "Opening Principles".to_string(),
                "What is a good first move that controls the center?".to_string(),
                vec!["e4".to_string(), "d4".to_string(), "Nf3".to_string(), "c4".to_string()],
                "The best opening moves control the center, develop pieces, and prepare for castling. e4, d4, Nf3, and c4 are all excellent first moves.".to_string(),
            )
            .with_hints(vec!["Start by controlling the center with pawns or pieces.".to_string()]),
        ]
    }

    pub fn get_endgame_exercises() -> Vec<Exercise> {
        vec![
            Exercise::new(
                ExerciseType::Endgame,
                ExerciseDifficulty::Beginner,
                "8/8/8/8/8/3k4/3P4/3K4 w - - 0 1".to_string(),
                "King and Pawn Endgame".to_string(),
                "White to move and win. How do you promote the pawn?".to_string(),
                vec!["Kd1".to_string()],
                "The key is to support your pawn with the king. Kd1 prepares to advance the pawn safely.".to_string(),
            )
            .with_hints(vec!["Your king must support the pawn's advance.".to_string()]),
        ]
    }

    pub fn get_positional_exercises() -> Vec<Exercise> {
        vec![
            Exercise::new(
                ExerciseType::Positional,
                ExerciseDifficulty::Intermediate,
                "r1bq1rk1/ppp2ppp/2np1n2/2b1p3/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 w - - 0 8".to_string(),
                "Develop Your Pieces".to_string(),
                "Which piece should White develop next?".to_string(),
                vec!["Be3".to_string(), "Bg5".to_string()],
                "Developing the light-squared bishop with Be3 or Bg5 completes White's development and prepares to control key squares.".to_string(),
            )
            .with_hints(vec!["Complete your development before launching an attack.".to_string()]),
        ]
    }

    pub fn get_all_exercises() -> Vec<Exercise> {
        let mut exercises = Vec::new();
        exercises.extend(Self::get_tactical_exercises());
        exercises.extend(Self::get_opening_exercises());
        exercises.extend(Self::get_endgame_exercises());
        exercises.extend(Self::get_positional_exercises());
        exercises
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_creation() {
        let exercise = Exercise::new(
            ExerciseType::Tactics,
            ExerciseDifficulty::Beginner,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            "Test".to_string(),
            "Test exercise".to_string(),
            vec!["e4".to_string()],
            "Explanation".to_string(),
        );

        assert_eq!(exercise.exercise_type, ExerciseType::Tactics);
        assert!(exercise.get_board().is_ok());
    }

    #[test]
    fn test_check_solution() {
        let exercise = Exercise::new(
            ExerciseType::Tactics,
            ExerciseDifficulty::Beginner,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            "Test".to_string(),
            "Test exercise".to_string(),
            vec!["e4".to_string()],
            "Explanation".to_string(),
        );

        assert!(exercise.check_solution("e4"));
        assert!(!exercise.check_solution("d4"));
    }
}
