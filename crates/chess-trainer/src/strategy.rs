use serde::{Deserialize, Serialize};
use crate::exercise::{Exercise, ExerciseType, ExerciseDifficulty, ExerciseLibrary};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StrategyPattern {
    OpeningPrinciples,      // Control center, develop pieces, castle
    TacticalAwareness,      // Forks, pins, skewers
    EndgameTechnique,       // King and pawn, rook endgames
    PositionalPlay,         // Pawn structure, piece activity
    AttackingPlay,          // King safety, piece coordination
    DefensivePlay,          // Defending weak points, counterplay
    CalculationSkills,      // Visualizing variations
    TimeManagement,         // Managing time in games
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    pub pattern: StrategyPattern,
    pub name: String,
    pub description: String,
    pub key_concepts: Vec<String>,
    pub recommended_exercises: Vec<ExerciseType>,
}

impl Strategy {
    pub fn new(
        pattern: StrategyPattern,
        name: String,
        description: String,
        key_concepts: Vec<String>,
        recommended_exercises: Vec<ExerciseType>,
    ) -> Self {
        Self {
            pattern,
            name,
            description,
            key_concepts,
            recommended_exercises,
        }
    }

    pub fn get_exercises(&self, difficulty: ExerciseDifficulty) -> Vec<Exercise> {
        let all_exercises = ExerciseLibrary::get_all_exercises();

        all_exercises
            .into_iter()
            .filter(|ex| {
                self.recommended_exercises.contains(&ex.exercise_type)
                    && ex.difficulty <= difficulty
            })
            .collect()
    }
}

pub struct StrategyLibrary;

impl StrategyLibrary {
    pub fn get_opening_principles() -> Strategy {
        Strategy::new(
            StrategyPattern::OpeningPrinciples,
            "Opening Principles".to_string(),
            "Learn the fundamental principles of chess openings: control the center, develop your pieces, and ensure king safety.".to_string(),
            vec![
                "Control the center with pawns (e4, d4) or pieces".to_string(),
                "Develop knights before bishops".to_string(),
                "Don't move the same piece twice in the opening".to_string(),
                "Castle early to ensure king safety".to_string(),
                "Don't bring the queen out too early".to_string(),
                "Connect your rooks".to_string(),
            ],
            vec![ExerciseType::Opening, ExerciseType::Positional],
        )
    }

    pub fn get_tactical_awareness() -> Strategy {
        Strategy::new(
            StrategyPattern::TacticalAwareness,
            "Tactical Awareness".to_string(),
            "Develop your ability to spot tactical opportunities like forks, pins, skewers, and discovered attacks.".to_string(),
            vec![
                "Fork: Attack two pieces at once".to_string(),
                "Pin: Restrict piece movement by threatening a more valuable piece behind it".to_string(),
                "Skewer: Force a valuable piece to move, exposing a less valuable piece".to_string(),
                "Discovered attack: Move a piece to reveal an attack from another piece".to_string(),
                "Double attack: Attack two targets simultaneously".to_string(),
            ],
            vec![ExerciseType::Tactics, ExerciseType::Calculation],
        )
    }

    pub fn get_endgame_technique() -> Strategy {
        Strategy::new(
            StrategyPattern::EndgameTechnique,
            "Endgame Technique".to_string(),
            "Master essential endgame positions and techniques to convert advantages into wins.".to_string(),
            vec![
                "Activate your king in the endgame".to_string(),
                "Learn basic checkmates (K+Q vs K, K+R vs K)".to_string(),
                "Understand pawn endgames and the opposition".to_string(),
                "Know when to trade pieces".to_string(),
                "Create passed pawns".to_string(),
            ],
            vec![ExerciseType::Endgame, ExerciseType::Calculation],
        )
    }

    pub fn get_positional_play() -> Strategy {
        Strategy::new(
            StrategyPattern::PositionalPlay,
            "Positional Play".to_string(),
            "Improve your understanding of positional chess: pawn structure, piece activity, and long-term planning.".to_string(),
            vec![
                "Understand pawn structures (chains, islands, doubled pawns)".to_string(),
                "Improve piece activity and coordination".to_string(),
                "Control key squares and files".to_string(),
                "Identify weak pawns and squares".to_string(),
                "Create and exploit weaknesses in opponent's position".to_string(),
            ],
            vec![ExerciseType::Positional, ExerciseType::Strategy],
        )
    }

    pub fn get_all_strategies() -> Vec<Strategy> {
        vec![
            Self::get_opening_principles(),
            Self::get_tactical_awareness(),
            Self::get_endgame_technique(),
            Self::get_positional_play(),
        ]
    }

    pub fn get_strategy_for_weakness(weakness: &str) -> Option<Strategy> {
        let weakness_lower = weakness.to_lowercase();

        if weakness_lower.contains("opening") {
            Some(Self::get_opening_principles())
        } else if weakness_lower.contains("tactic") || weakness_lower.contains("blunder") {
            Some(Self::get_tactical_awareness())
        } else if weakness_lower.contains("endgame") {
            Some(Self::get_endgame_technique())
        } else if weakness_lower.contains("positional") || weakness_lower.contains("inaccuracy") {
            Some(Self::get_positional_play())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_creation() {
        let strategy = StrategyLibrary::get_opening_principles();
        assert_eq!(strategy.pattern, StrategyPattern::OpeningPrinciples);
        assert!(!strategy.key_concepts.is_empty());
    }

    #[test]
    fn test_get_exercises_for_strategy() {
        let strategy = StrategyLibrary::get_tactical_awareness();
        let exercises = strategy.get_exercises(ExerciseDifficulty::Expert);
        // Should return some tactical exercises
        assert!(!exercises.is_empty());
    }

    #[test]
    fn test_strategy_for_weakness() {
        let strategy = StrategyLibrary::get_strategy_for_weakness("weak opening play");
        assert!(strategy.is_some());
        assert_eq!(strategy.unwrap().pattern, StrategyPattern::OpeningPrinciples);
    }
}
