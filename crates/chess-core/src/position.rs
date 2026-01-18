use chess::{Board, Color, Piece, Square, ALL_SQUARES};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(serialize_with = "serialize_board", deserialize_with = "deserialize_board")]
    pub board: Board,
}

fn serialize_board<S>(board: &Board, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("{}", board))
}

fn deserialize_board<'de, D>(deserializer: D) -> std::result::Result<Board, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Board::from_str(&s).map_err(serde::de::Error::custom)
}

impl Position {
    pub fn new(board: Board) -> Self {
        Self { board }
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        Board::from_str(fen)
            .map(|board| Self { board })
            .map_err(|e| format!("Invalid FEN: {}", e))
    }

    pub fn material_count(&self, color: Color) -> i32 {
        let mut count = 0;
        for square in ALL_SQUARES.iter() {
            if let Some(piece) = self.board.piece_on(*square) {
                if self.board.color_on(*square) == Some(color) {
                    count += Self::piece_value(piece);
                }
            }
        }
        count
    }

    fn piece_value(piece: Piece) -> i32 {
        match piece {
            Piece::Pawn => 1,
            Piece::Knight => 3,
            Piece::Bishop => 3,
            Piece::Rook => 5,
            Piece::Queen => 9,
            Piece::King => 0, // King has no material value
        }
    }

    pub fn material_balance(&self) -> i32 {
        self.material_count(Color::White) - self.material_count(Color::Black)
    }

    pub fn piece_count(&self, color: Color, piece: Piece) -> u8 {
        let mut count = 0;
        for square in ALL_SQUARES.iter() {
            if self.board.piece_on(*square) == Some(piece)
                && self.board.color_on(*square) == Some(color)
            {
                count += 1;
            }
        }
        count
    }

    pub fn is_endgame(&self) -> bool {
        // Simple heuristic: endgame if queens are off or total material is low
        let white_queens = self.piece_count(Color::White, Piece::Queen);
        let black_queens = self.piece_count(Color::Black, Piece::Queen);
        let total_material = self.material_count(Color::White) + self.material_count(Color::Black);

        (white_queens == 0 && black_queens == 0) || total_material < 20
    }

    pub fn analyze(&self) -> PositionAnalysis {
        PositionAnalysis {
            material_balance: self.material_balance(),
            is_endgame: self.is_endgame(),
            white_material: self.material_count(Color::White),
            black_material: self.material_count(Color::Black),
            side_to_move: self.board.side_to_move(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionAnalysis {
    pub material_balance: i32,
    pub is_endgame: bool,
    pub white_material: i32,
    pub black_material: i32,
    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub side_to_move: Color,
}

fn serialize_color<S>(color: &Color, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(match color {
        Color::White => "White",
        Color::Black => "Black",
    })
}

fn deserialize_color<'de, D>(deserializer: D) -> std::result::Result<Color, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "White" => Ok(Color::White),
        "Black" => Ok(Color::Black),
        _ => Err(serde::de::Error::custom("Invalid color")),
    }
}

impl PositionAnalysis {
    pub fn advantage(&self) -> &str {
        if self.material_balance > 3 {
            "White has significant advantage"
        } else if self.material_balance > 0 {
            "White has slight advantage"
        } else if self.material_balance < -3 {
            "Black has significant advantage"
        } else if self.material_balance < 0 {
            "Black has slight advantage"
        } else {
            "Equal position"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_position() {
        let position = Position::new(Board::default());
        assert_eq!(position.material_balance(), 0);
        assert!(!position.is_endgame());
    }

    #[test]
    fn test_material_count() {
        let position = Position::new(Board::default());
        // Starting position: 8 pawns + 2 knights + 2 bishops + 2 rooks + 1 queen
        // = 8*1 + 2*3 + 2*3 + 2*5 + 1*9 = 8 + 6 + 6 + 10 + 9 = 39
        assert_eq!(position.material_count(Color::White), 39);
        assert_eq!(position.material_count(Color::Black), 39);
    }
}
