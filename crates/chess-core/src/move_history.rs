use chess::ChessMove;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MoveQuality {
    Brilliant,    // !!
    Great,        // !
    Good,         // No annotation
    Inaccuracy,   // ?!
    Mistake,      // ?
    Blunder,      // ??
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotatedMove {
    #[serde(serialize_with = "serialize_chess_move", deserialize_with = "deserialize_chess_move")]
    pub chess_move: ChessMove,
    pub quality: Option<MoveQuality>,
    pub comment: Option<String>,
    pub evaluation: Option<f32>, // Centipawn evaluation after move
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
    let s = String::deserialize(deserializer)?;
    // Parse the UCI move string (e.g., "e2e4")
    if s.len() >= 4 {
        use std::str::FromStr;
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

impl AnnotatedMove {
    pub fn from_move(chess_move: ChessMove) -> Self {
        Self {
            chess_move,
            quality: None,
            comment: None,
            evaluation: None,
        }
    }

    pub fn with_quality(mut self, quality: MoveQuality) -> Self {
        self.quality = Some(quality);
        self
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn with_evaluation(mut self, evaluation: f32) -> Self {
        self.evaluation = Some(evaluation);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveHistory {
    moves: Vec<AnnotatedMove>,
}

impl MoveHistory {
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    pub fn add_move(&mut self, annotated_move: AnnotatedMove) {
        self.moves.push(annotated_move);
    }

    pub fn get_move(&self, index: usize) -> Option<&AnnotatedMove> {
        self.moves.get(index)
    }

    pub fn get_move_mut(&mut self, index: usize) -> Option<&mut AnnotatedMove> {
        self.moves.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &AnnotatedMove> {
        self.moves.iter()
    }

    pub fn last(&self) -> Option<&AnnotatedMove> {
        self.moves.last()
    }

    pub fn get_all_moves(&self) -> &[AnnotatedMove] {
        &self.moves
    }

    pub fn clear(&mut self) {
        self.moves.clear();
    }
}

impl Default for MoveHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::Square;

    #[test]
    fn test_move_history() {
        let mut history = MoveHistory::new();
        assert!(history.is_empty());

        let chess_move = ChessMove::new(Square::E2, Square::E4, None);
        let annotated = AnnotatedMove::from_move(chess_move)
            .with_quality(MoveQuality::Good)
            .with_evaluation(0.5);

        history.add_move(annotated);
        assert_eq!(history.len(), 1);
        assert!(history.last().is_some());
    }
}
