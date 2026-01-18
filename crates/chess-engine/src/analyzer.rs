use chess::{Board, ChessMove, Color};
use chess_core::{ChessGame, MoveQuality, AnnotatedMove};
use serde::{Deserialize, Serialize};
use crate::evaluator::Evaluator;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TacticalPattern {
    Fork,
    Pin,
    Skewer,
    DiscoveredAttack,
    DoubleThreat,
    Sacrifice,
    Checkmate,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveAnalysis {
    pub move_number: usize,
    #[serde(serialize_with = "serialize_chess_move", deserialize_with = "deserialize_chess_move")]
    pub chess_move: ChessMove,
    pub evaluation_before: i32,
    pub evaluation_after: i32,
    #[serde(serialize_with = "serialize_chess_move", deserialize_with = "deserialize_chess_move")]
    pub best_move: ChessMove,
    pub best_move_eval: i32,
    pub quality: MoveQuality,
    pub centipawn_loss: i32,
    pub tactical_pattern: TacticalPattern,
    pub comment: String,
}

fn serialize_chess_move<S>(chess_move: &ChessMove, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("{}", chess_move))
}

fn deserialize_chess_move<'de, D>(deserializer: D) -> std::result::Result<ChessMove, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use std::str::FromStr;
    let s = String::deserialize(deserializer)?;
    // Parse the UCI move string (e.g., "e2e4")
    if s.len() >= 4 {
        let from = chess::Square::from_str(&s[0..2])
            .map_err(|e| serde::de::Error::custom(format!("Invalid from square: {}", e)))?;
        let to = chess::Square::from_str(&s[2..4])
            .map_err(|e| serde::de::Error::custom(format!("Invalid to square: {}", e)))?;
        let promotion = if s.len() == 5 {
            let promo_char = s.chars().nth(4).unwrap();
            Some(match promo_char.to_lowercase().next().unwrap() {
                'q' => chess::Piece::Queen,
                'r' => chess::Piece::Rook,
                'b' => chess::Piece::Bishop,
                'n' => chess::Piece::Knight,
                _ => return Err(serde::de::Error::custom("Invalid promotion piece")),
            })
        } else {
            None
        };
        Ok(ChessMove::new(from, to, promotion))
    } else {
        Err(serde::de::Error::custom("Move string too short"))
    }
}

pub struct GameAnalyzer;

impl GameAnalyzer {
    pub fn analyze_game(game: &ChessGame) -> Vec<MoveAnalysis> {
        let mut analyses = Vec::new();
        let mut board = Board::default();

        for (index, annotated_move) in game.move_history.iter().enumerate() {
            let analysis = Self::analyze_move(&board, annotated_move.chess_move, index);
            analyses.push(analysis);
            board = board.make_move_new(annotated_move.chess_move);
        }

        analyses
    }

    pub fn analyze_move(board: &Board, chess_move: ChessMove, move_number: usize) -> MoveAnalysis {
        let eval_before = Evaluator::evaluate_position(board);
        let new_board = board.make_move_new(chess_move);
        let eval_after = Evaluator::evaluate_position(&new_board);

        let best_move_eval = Evaluator::find_best_move(board);
        let (best_move, best_move_score) = match best_move_eval {
            Some(eval) => (eval.chess_move, eval.score),
            None => (chess_move, eval_after.score),
        };

        // Calculate centipawn loss (from the player's perspective)
        let centipawn_loss = (best_move_score - eval_after.score).abs();

        let quality = Self::determine_move_quality(centipawn_loss);
        let tactical_pattern = Self::detect_tactical_pattern(board, chess_move);
        let comment = Self::generate_comment(&quality, centipawn_loss, &tactical_pattern, chess_move == best_move);

        MoveAnalysis {
            move_number,
            chess_move,
            evaluation_before: eval_before.score,
            evaluation_after: eval_after.score,
            best_move,
            best_move_eval: best_move_score,
            quality,
            centipawn_loss,
            tactical_pattern,
            comment,
        }
    }

    fn determine_move_quality(centipawn_loss: i32) -> MoveQuality {
        match centipawn_loss {
            0..=25 => MoveQuality::Brilliant,
            26..=50 => MoveQuality::Great,
            51..=100 => MoveQuality::Good,
            101..=200 => MoveQuality::Inaccuracy,
            201..=400 => MoveQuality::Mistake,
            _ => MoveQuality::Blunder,
        }
    }

    fn detect_tactical_pattern(_board: &Board, _chess_move: ChessMove) -> TacticalPattern {
        // Simplified tactical pattern detection
        // In a real implementation, this would analyze the position for tactical motifs
        TacticalPattern::None
    }

    fn generate_comment(
        quality: &MoveQuality,
        centipawn_loss: i32,
        tactical_pattern: &TacticalPattern,
        is_best_move: bool,
    ) -> String {
        let mut comment = String::new();

        if is_best_move {
            comment.push_str("Best move! ");
        }

        match quality {
            MoveQuality::Brilliant => comment.push_str("Excellent move."),
            MoveQuality::Great => comment.push_str("Very good move."),
            MoveQuality::Good => comment.push_str("Good move."),
            MoveQuality::Inaccuracy => {
                comment.push_str(&format!("Inaccurate. Lost {} centipawns.", centipawn_loss));
            }
            MoveQuality::Mistake => {
                comment.push_str(&format!("Mistake! Lost {} centipawns.", centipawn_loss));
            }
            MoveQuality::Blunder => {
                comment.push_str(&format!("Blunder!! Lost {} centipawns.", centipawn_loss));
            }
        }

        if tactical_pattern != &TacticalPattern::None {
            comment.push_str(&format!(" Pattern: {:?}", tactical_pattern));
        }

        comment
    }

    pub fn identify_weaknesses(analyses: &[MoveAnalysis]) -> Vec<String> {
        let mut weaknesses = Vec::new();

        let total_moves = analyses.len();
        if total_moves == 0 {
            return weaknesses;
        }

        // Count different types of moves
        let blunders = analyses.iter().filter(|a| a.quality == MoveQuality::Blunder).count();
        let mistakes = analyses.iter().filter(|a| a.quality == MoveQuality::Mistake).count();
        let inaccuracies = analyses.iter().filter(|a| a.quality == MoveQuality::Inaccuracy).count();

        // Calculate average centipawn loss
        let avg_loss: i32 = analyses.iter().map(|a| a.centipawn_loss).sum::<i32>() / total_moves as i32;

        if blunders > total_moves / 10 {
            weaknesses.push("Frequent blunders - practice tactics and calculation".to_string());
        }

        if mistakes > total_moves / 5 {
            weaknesses.push("Many mistakes - slow down and calculate variations".to_string());
        }

        if inaccuracies > total_moves / 3 {
            weaknesses.push("Numerous inaccuracies - study positional chess".to_string());
        }

        if avg_loss > 100 {
            weaknesses.push("High average centipawn loss - improve move evaluation".to_string());
        }

        // Analyze game phases
        let opening_moves = &analyses[..analyses.len().min(10)];
        let opening_avg_loss: i32 = opening_moves.iter().map(|a| a.centipawn_loss).sum::<i32>()
            / opening_moves.len().max(1) as i32;

        if opening_avg_loss > 80 {
            weaknesses.push("Weak opening play - study opening principles".to_string());
        }

        if weaknesses.is_empty() {
            weaknesses.push("Overall solid play! Focus on maintaining consistency.".to_string());
        }

        weaknesses
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::Square;

    #[test]
    fn test_analyze_move() {
        let board = Board::default();
        let chess_move = ChessMove::new(Square::E2, Square::E4, None);
        let analysis = GameAnalyzer::analyze_move(&board, chess_move, 0);

        assert_eq!(analysis.move_number, 0);
        assert_eq!(analysis.chess_move, chess_move);
    }

    #[test]
    fn test_move_quality_determination() {
        assert_eq!(GameAnalyzer::determine_move_quality(10), MoveQuality::Brilliant);
        assert_eq!(GameAnalyzer::determine_move_quality(150), MoveQuality::Inaccuracy);
        assert_eq!(GameAnalyzer::determine_move_quality(500), MoveQuality::Blunder);
    }
}
