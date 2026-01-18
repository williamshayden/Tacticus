use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChessError {
    #[error("Invalid move: {0}")]
    InvalidMove(String),

    #[error("Game already finished")]
    GameFinished,

    #[error("Invalid position: {0}")]
    InvalidPosition(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, ChessError>;
