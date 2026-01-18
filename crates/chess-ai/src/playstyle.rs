use chess_core::ChessGame;
use chess_engine::{GameAnalyzer, MoveAnalysis};
use chess_core::MoveQuality;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayStyle {
    Aggressive,     // Prefers attacking, sacrifices
    Positional,     // Focuses on long-term advantages
    Tactical,       // Looks for tactical opportunities
    Solid,          // Defensive, safe play
    Balanced,       // Mix of all styles
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleCharacteristics {
    pub aggression_score: f32,      // 0.0 to 1.0
    pub tactical_score: f32,        // 0.0 to 1.0
    pub positional_score: f32,      // 0.0 to 1.0
    pub risk_taking_score: f32,     // 0.0 to 1.0
    pub accuracy_score: f32,        // 0.0 to 1.0
    pub primary_style: PlayStyle,
}

impl StyleCharacteristics {
    pub fn determine_play_style(&self) -> PlayStyle {
        if self.aggression_score > 0.7 && self.risk_taking_score > 0.6 {
            PlayStyle::Aggressive
        } else if self.tactical_score > 0.7 {
            PlayStyle::Tactical
        } else if self.positional_score > 0.7 {
            PlayStyle::Positional
        } else if self.accuracy_score > 0.8 && self.risk_taking_score < 0.4 {
            PlayStyle::Solid
        } else {
            PlayStyle::Balanced
        }
    }
}

pub struct PlayStyleAnalyzer;

impl PlayStyleAnalyzer {
    pub fn analyze_game(game: &ChessGame) -> StyleCharacteristics {
        let analyses = GameAnalyzer::analyze_game(game);

        let aggression_score = Self::calculate_aggression(&analyses);
        let tactical_score = Self::calculate_tactical_awareness(&analyses);
        let positional_score = Self::calculate_positional_understanding(&analyses);
        let risk_taking_score = Self::calculate_risk_taking(&analyses);
        let accuracy_score = Self::calculate_accuracy(&analyses);

        let mut characteristics = StyleCharacteristics {
            aggression_score,
            tactical_score,
            positional_score,
            risk_taking_score,
            accuracy_score,
            primary_style: PlayStyle::Balanced,
        };

        characteristics.primary_style = characteristics.determine_play_style();
        characteristics
    }

    pub fn analyze_multiple_games(games: &[ChessGame]) -> StyleCharacteristics {
        if games.is_empty() {
            return Self::default_characteristics();
        }

        let mut total_aggression = 0.0;
        let mut total_tactical = 0.0;
        let mut total_positional = 0.0;
        let mut total_risk = 0.0;
        let mut total_accuracy = 0.0;

        for game in games {
            let chars = Self::analyze_game(game);
            total_aggression += chars.aggression_score;
            total_tactical += chars.tactical_score;
            total_positional += chars.positional_score;
            total_risk += chars.risk_taking_score;
            total_accuracy += chars.accuracy_score;
        }

        let count = games.len() as f32;
        let mut characteristics = StyleCharacteristics {
            aggression_score: total_aggression / count,
            tactical_score: total_tactical / count,
            positional_score: total_positional / count,
            risk_taking_score: total_risk / count,
            accuracy_score: total_accuracy / count,
            primary_style: PlayStyle::Balanced,
        };

        characteristics.primary_style = characteristics.determine_play_style();
        characteristics
    }

    fn calculate_aggression(analyses: &[MoveAnalysis]) -> f32 {
        if analyses.is_empty() {
            return 0.5;
        }

        // Aggression based on sacrifices, checks, attacks
        let aggressive_moves = analyses.iter().filter(|a| {
            a.centipawn_loss > 100 // Willing to sacrifice material
                || a.evaluation_after - a.evaluation_before > 50 // Improved position
        }).count();

        (aggressive_moves as f32 / analyses.len() as f32).min(1.0)
    }

    fn calculate_tactical_awareness(analyses: &[MoveAnalysis]) -> f32 {
        if analyses.is_empty() {
            return 0.5;
        }

        // Tactical awareness based on move quality
        let good_tactical_moves = analyses.iter().filter(|a| {
            matches!(a.quality, MoveQuality::Brilliant | MoveQuality::Great)
        }).count();

        (good_tactical_moves as f32 / analyses.len() as f32 * 2.0).min(1.0)
    }

    fn calculate_positional_understanding(analyses: &[MoveAnalysis]) -> f32 {
        if analyses.is_empty() {
            return 0.5;
        }

        // Positional play: consistent, solid moves without tactical fireworks
        let solid_moves = analyses.iter().filter(|a| {
            a.centipawn_loss < 50 && a.quality == MoveQuality::Good
        }).count();

        (solid_moves as f32 / analyses.len() as f32 * 1.5).min(1.0)
    }

    fn calculate_risk_taking(analyses: &[MoveAnalysis]) -> f32 {
        if analyses.is_empty() {
            return 0.5;
        }

        // Risk taking based on centipawn loss variance
        let risky_moves = analyses.iter().filter(|a| {
            a.centipawn_loss > 150
        }).count();

        (risky_moves as f32 / analyses.len() as f32 * 2.0).min(1.0)
    }

    fn calculate_accuracy(analyses: &[MoveAnalysis]) -> f32 {
        if analyses.is_empty() {
            return 0.5;
        }

        // Accuracy based on average centipawn loss
        let total_loss: i32 = analyses.iter().map(|a| a.centipawn_loss).sum();
        let avg_loss = total_loss as f32 / analyses.len() as f32;

        // Lower centipawn loss = higher accuracy
        (1.0 - (avg_loss / 200.0)).max(0.0).min(1.0)
    }

    fn default_characteristics() -> StyleCharacteristics {
        StyleCharacteristics {
            aggression_score: 0.5,
            tactical_score: 0.5,
            positional_score: 0.5,
            risk_taking_score: 0.5,
            accuracy_score: 0.5,
            primary_style: PlayStyle::Balanced,
        }
    }

    pub fn get_style_description(style: &PlayStyle) -> &str {
        match style {
            PlayStyle::Aggressive => {
                "You prefer aggressive, attacking chess with tactical complications and sacrifices."
            }
            PlayStyle::Tactical => {
                "You excel at spotting tactical opportunities and calculating variations."
            }
            PlayStyle::Positional => {
                "You focus on long-term positional advantages and strategic planning."
            }
            PlayStyle::Solid => {
                "You play solid, defensive chess with minimal risk-taking."
            }
            PlayStyle::Balanced => {
                "You have a balanced playing style, adapting to different positions."
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::Color;

    #[test]
    fn test_default_characteristics() {
        let chars = PlayStyleAnalyzer::default_characteristics();
        assert_eq!(chars.primary_style, PlayStyle::Balanced);
    }

    #[test]
    fn test_analyze_game() {
        let game = ChessGame::new(Color::White);
        let chars = PlayStyleAnalyzer::analyze_game(&game);
        assert!(chars.aggression_score >= 0.0 && chars.aggression_score <= 1.0);
    }

    #[test]
    fn test_style_determination() {
        let aggressive_chars = StyleCharacteristics {
            aggression_score: 0.8,
            tactical_score: 0.6,
            positional_score: 0.4,
            risk_taking_score: 0.7,
            accuracy_score: 0.6,
            primary_style: PlayStyle::Balanced,
        };

        assert_eq!(aggressive_chars.determine_play_style(), PlayStyle::Aggressive);
    }
}
