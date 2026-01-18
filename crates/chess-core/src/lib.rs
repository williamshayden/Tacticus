pub mod game;
pub mod position;
pub mod move_history;
pub mod error;

pub use game::{ChessGame, GameState};
pub use position::{Position, PositionAnalysis};
pub use move_history::{MoveHistory, AnnotatedMove, MoveQuality};
pub use error::{ChessError, Result};

// Re-export commonly used chess types
pub use chess::{Board, ChessMove, Color, Piece, Square, File, Rank};
