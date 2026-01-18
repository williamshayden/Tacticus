use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::exercise::{Exercise, ExerciseDifficulty, ExerciseResult};
use crate::strategy::{Strategy, StrategyLibrary};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSession {
    pub id: Option<u64>,
    pub user_id: u64,
    pub exercises: Vec<Exercise>,
    pub current_exercise_index: usize,
    pub results: Vec<ExerciseResult>,
    pub strategies: Vec<Strategy>,
    pub difficulty: ExerciseDifficulty,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl TrainingSession {
    pub fn new(user_id: u64, difficulty: ExerciseDifficulty) -> Self {
        Self {
            id: None,
            user_id,
            exercises: Vec::new(),
            current_exercise_index: 0,
            results: Vec::new(),
            strategies: Vec::new(),
            difficulty,
            started_at: Utc::now(),
            finished_at: None,
        }
    }

    pub fn with_weaknesses(user_id: u64, weaknesses: Vec<String>, difficulty: ExerciseDifficulty) -> Self {
        let mut session = Self::new(user_id, difficulty);

        // Get strategies based on weaknesses
        for weakness in &weaknesses {
            if let Some(strategy) = StrategyLibrary::get_strategy_for_weakness(weakness) {
                session.add_strategy(strategy);
            }
        }

        // If no specific strategies were found, add general strategies
        if session.strategies.is_empty() {
            session.strategies = StrategyLibrary::get_all_strategies();
        }

        // Generate exercises based on strategies
        session.generate_exercises();

        session
    }

    pub fn add_strategy(&mut self, strategy: Strategy) {
        if !self.strategies.iter().any(|s| s.pattern == strategy.pattern) {
            self.strategies.push(strategy);
        }
    }

    pub fn generate_exercises(&mut self) {
        self.exercises.clear();

        for strategy in &self.strategies {
            let mut exercises = strategy.get_exercises(self.difficulty.clone());
            // Limit to 2-3 exercises per strategy to reach 5-10 total
            exercises.truncate(3);
            self.exercises.extend(exercises);
        }

        // Ensure we have between 5-10 exercises
        if self.exercises.len() < 5 {
            // Add more exercises from all strategies
            let all_strategies = StrategyLibrary::get_all_strategies();
            for strategy in all_strategies {
                let exercises = strategy.get_exercises(self.difficulty.clone());
                self.exercises.extend(exercises);
                if self.exercises.len() >= 5 {
                    break;
                }
            }
        }

        // Limit to 10 exercises maximum
        self.exercises.truncate(10);
        self.current_exercise_index = 0;
    }

    pub fn current_exercise(&self) -> Option<&Exercise> {
        self.exercises.get(self.current_exercise_index)
    }

    pub fn next_exercise(&mut self) -> Option<&Exercise> {
        if self.current_exercise_index < self.exercises.len() {
            self.current_exercise_index += 1;
        }
        self.current_exercise()
    }

    pub fn record_result(&mut self, result: ExerciseResult) {
        self.results.push(result);
    }

    pub fn is_finished(&self) -> bool {
        self.current_exercise_index >= self.exercises.len()
    }

    pub fn finish(&mut self) {
        self.finished_at = Some(Utc::now());
    }

    pub fn get_session_result(&self) -> SessionResult {
        let total_exercises = self.exercises.len();
        let completed_exercises = self.results.len();
        let solved_exercises = self.results.iter().filter(|r| r.solved).count();
        let total_attempts = self.results.iter().map(|r| r.attempts).sum();
        let total_hints_used = self.results.iter().map(|r| r.hints_used).sum();

        let success_rate = if completed_exercises > 0 {
            (solved_exercises as f32 / completed_exercises as f32) * 100.0
        } else {
            0.0
        };

        let duration_seconds = if let Some(finished) = self.finished_at {
            (finished - self.started_at).num_seconds() as u32
        } else {
            0
        };

        SessionResult {
            total_exercises,
            completed_exercises,
            solved_exercises,
            success_rate,
            total_attempts,
            total_hints_used,
            duration_seconds,
            strategies_covered: self.strategies.iter().map(|s| s.name.clone()).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
    pub total_exercises: usize,
    pub completed_exercises: usize,
    pub solved_exercises: usize,
    pub success_rate: f32,
    pub total_attempts: u32,
    pub total_hints_used: u32,
    pub duration_seconds: u32,
    pub strategies_covered: Vec<String>,
}

impl SessionResult {
    pub fn summary(&self) -> String {
        format!(
            "Session Complete!\n\
             Exercises: {}/{} completed, {} solved\n\
             Success Rate: {:.1}%\n\
             Total Attempts: {}\n\
             Hints Used: {}\n\
             Duration: {} seconds\n\
             Strategies Covered: {}",
            self.completed_exercises,
            self.total_exercises,
            self.solved_exercises,
            self.success_rate,
            self.total_attempts,
            self.total_hints_used,
            self.duration_seconds,
            self.strategies_covered.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training_session_creation() {
        let session = TrainingSession::new(1, ExerciseDifficulty::Beginner);
        assert_eq!(session.user_id, 1);
        assert_eq!(session.current_exercise_index, 0);
    }

    #[test]
    fn test_session_with_weaknesses() {
        let weaknesses = vec!["Weak opening play".to_string()];
        let session = TrainingSession::with_weaknesses(1, weaknesses, ExerciseDifficulty::Beginner);

        assert!(!session.exercises.is_empty());
        assert!(!session.strategies.is_empty());
    }

    #[test]
    fn test_session_result() {
        let mut session = TrainingSession::new(1, ExerciseDifficulty::Beginner);
        session.generate_exercises();

        let result = ExerciseResult {
            exercise_id: 1,
            user_id: 1,
            solved: true,
            attempts: 1,
            time_taken_seconds: 30,
            hints_used: 0,
            completed_at: Utc::now(),
        };

        session.record_result(result);
        let session_result = session.get_session_result();

        assert!(session_result.success_rate > 0.0);
    }
}
