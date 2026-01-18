use chess::{Board, BoardStatus, ChessMove, Color};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::error::{ChessError, Result};
use crate::move_history::{MoveHistory, AnnotatedMove};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    InProgress,
    Checkmate(Color), // Winner
    Stalemate,
    DrawByRepetition,
    DrawByInsufficientMaterial,
    DrawBy50MoveRule,
}

// Custom serialization for GameState
impl Serialize for GameState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("GameState", 2)?;
        match self {
            GameState::InProgress => {
                state.serialize_field("type", "InProgress")?;
            }
            GameState::Checkmate(color) => {
                state.serialize_field("type", "Checkmate")?;
                state.serialize_field("winner", if *color == Color::White { "White" } else { "Black" })?;
            }
            GameState::Stalemate => {
                state.serialize_field("type", "Stalemate")?;
            }
            GameState::DrawByRepetition => {
                state.serialize_field("type", "DrawByRepetition")?;
            }
            GameState::DrawByInsufficientMaterial => {
                state.serialize_field("type", "DrawByInsufficientMaterial")?;
            }
            GameState::DrawBy50MoveRule => {
                state.serialize_field("type", "DrawBy50MoveRule")?;
            }
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for GameState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct GameStateVisitor;

        impl<'de> Visitor<'de> for GameStateVisitor {
            type Value = GameState;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a GameState object")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GameState, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_str = None;
                let mut winner = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "type" => {
                            type_str = Some(map.next_value::<String>()?);
                        }
                        "winner" => {
                            winner = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            let _ = map.next_value::<serde_json::Value>()?;
                        }
                    }
                }

                let type_str = type_str.ok_or_else(|| de::Error::missing_field("type"))?;

                match type_str.as_str() {
                    "InProgress" => Ok(GameState::InProgress),
                    "Checkmate" => {
                        let winner_str = winner.ok_or_else(|| de::Error::missing_field("winner"))?;
                        let color = match winner_str.as_str() {
                            "White" => Color::White,
                            "Black" => Color::Black,
                            _ => return Err(de::Error::custom("Invalid winner color")),
                        };
                        Ok(GameState::Checkmate(color))
                    }
                    "Stalemate" => Ok(GameState::Stalemate),
                    "DrawByRepetition" => Ok(GameState::DrawByRepetition),
                    "DrawByInsufficientMaterial" => Ok(GameState::DrawByInsufficientMaterial),
                    "DrawBy50MoveRule" => Ok(GameState::DrawBy50MoveRule),
                    _ => Err(de::Error::custom("Invalid GameState type")),
                }
            }
        }

        deserializer.deserialize_struct("GameState", &["type", "winner"], GameStateVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChessGame {
    pub id: Option<u64>,
    #[serde(serialize_with = "serialize_board", deserialize_with = "deserialize_board")]
    pub board: Board,
    pub move_history: MoveHistory,
    pub state: GameState,
    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub player_color: Color,
    pub created_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
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

impl ChessGame {
    pub fn new(player_color: Color) -> Self {
        Self {
            id: None,
            board: Board::default(),
            move_history: MoveHistory::new(),
            state: GameState::InProgress,
            player_color,
            created_at: Utc::now(),
            finished_at: None,
        }
    }

    pub fn from_board(board: Board, player_color: Color) -> Self {
        let state = Self::determine_state(&board);
        Self {
            id: None,
            board,
            move_history: MoveHistory::new(),
            state,
            player_color,
            created_at: Utc::now(),
            finished_at: None,
        }
    }

    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<()> {
        if self.state != GameState::InProgress {
            return Err(ChessError::GameFinished);
        }

        let legal_moves: Vec<ChessMove> = chess::MoveGen::new_legal(&self.board).collect();

        if !legal_moves.contains(&chess_move) {
            return Err(ChessError::InvalidMove(format!(
                "Move {} is not legal in current position",
                chess_move
            )));
        }

        self.board = self.board.make_move_new(chess_move);
        self.move_history.add_move(AnnotatedMove::from_move(chess_move));
        self.state = Self::determine_state(&self.board);

        if self.state != GameState::InProgress {
            self.finished_at = Some(Utc::now());
        }

        Ok(())
    }

    pub fn legal_moves(&self) -> Vec<ChessMove> {
        chess::MoveGen::new_legal(&self.board).collect()
    }

    pub fn current_turn(&self) -> Color {
        self.board.side_to_move()
    }

    pub fn is_finished(&self) -> bool {
        self.state != GameState::InProgress
    }

    fn determine_state(board: &Board) -> GameState {
        match board.status() {
            BoardStatus::Checkmate => {
                // The side to move is checkmated, so the other side won
                let winner = !board.side_to_move();
                GameState::Checkmate(winner)
            }
            BoardStatus::Stalemate => GameState::Stalemate,
            BoardStatus::Ongoing => GameState::InProgress,
        }
    }

    pub fn get_fen(&self) -> String {
        format!("{}", self.board)
    }

    pub fn from_fen(fen: &str, player_color: Color) -> Result<Self> {
        let board = Board::from_str(fen)
            .map_err(|e| ChessError::ParseError(format!("Invalid FEN: {}", e)))?;
        Ok(Self::from_board(board, player_color))
    }
}

impl Default for ChessGame {
    fn default() -> Self {
        Self::new(Color::White)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess::Square;

    #[test]
    fn test_new_game() {
        let game = ChessGame::new(Color::White);
        assert_eq!(game.state, GameState::InProgress);
        assert_eq!(game.current_turn(), Color::White);
        assert!(!game.is_finished());
    }

    #[test]
    fn test_make_move() {
        let mut game = ChessGame::new(Color::White);
        let e2 = Square::E2;
        let e4 = Square::E4;
        let chess_move = ChessMove::new(e2, e4, None);

        assert!(game.make_move(chess_move).is_ok());
        assert_eq!(game.move_history.len(), 1);
        assert_eq!(game.current_turn(), Color::Black);
    }

    #[test]
    fn test_invalid_move() {
        let mut game = ChessGame::new(Color::White);
        // Try to move a piece that can't move this way
        let e2 = Square::E2;
        let e5 = Square::E5;
        let chess_move = ChessMove::new(e2, e5, None);

        assert!(game.make_move(chess_move).is_err());
    }
}
