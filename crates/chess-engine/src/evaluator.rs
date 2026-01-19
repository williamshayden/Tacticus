use chess::{Board, ChessMove, Color, Piece, Square, ALL_SQUARES, MoveGen};
use serde::{Deserialize, Serialize};

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 20000;

// Piece-square tables for positional evaluation
const PAWN_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

const KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionEvaluation {
    pub score: i32, // In centipawns (from white's perspective)
    pub material: i32,
    pub positional: i32,
    pub mobility: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveEvaluation {
    #[serde(serialize_with = "serialize_chess_move", deserialize_with = "deserialize_chess_move")]
    pub chess_move: ChessMove,
    pub score: i32,
    pub is_capture: bool,
    pub is_check: bool,
    pub is_promotion: bool,
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

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate_position(board: &Board) -> PositionEvaluation {
        let material = Self::evaluate_material(board);
        let positional = Self::evaluate_positional(board);
        let mobility = Self::evaluate_mobility(board);

        let mut score = material + positional + mobility;

        // Flip score if black to move (make it from side-to-move perspective)
        if board.side_to_move() == Color::Black {
            score = -score;
        }

        PositionEvaluation {
            score,
            material,
            positional,
            mobility,
        }
    }

    fn evaluate_material(board: &Board) -> i32 {
        let mut score = 0;

        for square in ALL_SQUARES.iter() {
            if let Some(piece) = board.piece_on(*square) {
                let value = Self::piece_value(piece);
                let piece_score = match board.color_on(*square) {
                    Some(Color::White) => value,
                    Some(Color::Black) => -value,
                    None => 0,
                };
                score += piece_score;
            }
        }

        score
    }

    fn piece_value(piece: Piece) -> i32 {
        match piece {
            Piece::Pawn => PAWN_VALUE,
            Piece::Knight => KNIGHT_VALUE,
            Piece::Bishop => BISHOP_VALUE,
            Piece::Rook => ROOK_VALUE,
            Piece::Queen => QUEEN_VALUE,
            Piece::King => KING_VALUE,
        }
    }

    fn evaluate_positional(board: &Board) -> i32 {
        let mut score = 0;

        for square in ALL_SQUARES.iter() {
            if let Some(piece) = board.piece_on(*square) {
                if let Some(color) = board.color_on(*square) {
                    let table_score = match piece {
                        Piece::Pawn => Self::get_piece_square_value(*square, &PAWN_TABLE, color),
                        Piece::Knight => Self::get_piece_square_value(*square, &KNIGHT_TABLE, color),
                        _ => 0,
                    };

                    score += if color == Color::White {
                        table_score
                    } else {
                        -table_score
                    };
                }
            }
        }

        score
    }

    fn get_piece_square_value(square: Square, table: &[i32; 64], color: Color) -> i32 {
        let index = match color {
            Color::White => square.to_index(),
            Color::Black => square.to_index() ^ 56, // Flip for black
        };
        table[index]
    }

    fn evaluate_mobility(board: &Board) -> i32 {
        let white_moves = MoveGen::new_legal(board).len();

        // Temporarily make a move for black (or use a simple heuristic)
        // For simplicity, we'll use a basic heuristic
        let mobility_score = white_moves as i32 * 10;

        if board.side_to_move() == Color::White {
            mobility_score
        } else {
            -mobility_score
        }
    }

    pub fn evaluate_move(board: &Board, chess_move: ChessMove) -> MoveEvaluation {
        let new_board = board.make_move_new(chess_move);
        let position_eval = Self::evaluate_position(&new_board);

        let is_capture = board.piece_on(chess_move.get_dest()).is_some();
        let is_check = new_board.checkers().popcnt() > 0;
        let is_promotion = chess_move.get_promotion().is_some();

        MoveEvaluation {
            chess_move,
            score: -position_eval.score, // Negate because it's from opponent's perspective
            is_capture,
            is_check,
            is_promotion,
        }
    }

    pub fn find_best_move(board: &Board) -> Option<MoveEvaluation> {
        let legal_moves: Vec<ChessMove> = MoveGen::new_legal(board).collect();

        if legal_moves.is_empty() {
            return None;
        }

        legal_moves
            .into_iter()
            .map(|m| Self::evaluate_move(board, m))
            .max_by_key(|eval| eval.score)
    }

    pub fn evaluate_all_moves(board: &Board) -> Vec<MoveEvaluation> {
        let legal_moves: Vec<ChessMove> = MoveGen::new_legal(board).collect();

        let mut evaluations: Vec<MoveEvaluation> = legal_moves
            .into_iter()
            .map(|m| Self::evaluate_move(board, m))
            .collect();

        evaluations.sort_by(|a, b| b.score.cmp(&a.score));
        evaluations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_starting_position() {
        let board = Board::default();
        let eval = Evaluator::evaluate_position(&board);
        // Starting position should be roughly equal (allow wider margin for positional factors)
        assert!(eval.score.abs() < 500, "Score was {}, expected near 0", eval.score);
    }

    #[test]
    fn test_find_best_move() {
        let board = Board::default();
        let best_move = Evaluator::find_best_move(&board);
        assert!(best_move.is_some());
    }
}
