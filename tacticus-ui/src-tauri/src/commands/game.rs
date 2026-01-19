use chess::{Board, ChessMove, Color, MoveGen, Piece, Square};
use chess_engine::Evaluator;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub fen: String,
    pub turn: String,
    pub is_checkmate: bool,
    pub is_stalemate: bool,
    pub is_check: bool,
    pub legal_moves: Vec<String>,
    pub last_move: Option<String>,
    pub evaluation: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveResult {
    pub success: bool,
    pub new_state: Option<GameState>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineMove {
    pub uci: String,
    pub san: String,
    pub evaluation: f32,
}

fn board_to_game_state(board: &Board, last_move: Option<String>) -> GameState {
    let legal_moves: Vec<String> = MoveGen::new_legal(board)
        .map(|m| format!("{}", m))
        .collect();
    
    let eval = Evaluator::evaluate_position(board);
    let is_check = *board.checkers() != chess::EMPTY;
    let is_checkmate = legal_moves.is_empty() && is_check;
    let is_stalemate = legal_moves.is_empty() && !is_check;
    
    GameState {
        fen: format!("{}", board),
        turn: if board.side_to_move() == Color::White { "white".to_string() } else { "black".to_string() },
        is_checkmate,
        is_stalemate,
        is_check,
        legal_moves,
        last_move,
        evaluation: eval.score as f32 / 100.0,
    }
}

#[tauri::command]
pub fn get_initial_position() -> GameState {
    let board = Board::default();
    board_to_game_state(&board, None)
}

#[tauri::command]
pub fn get_legal_moves(fen: String) -> Result<Vec<String>, String> {
    let board = Board::from_str(&fen).map_err(|e| format!("Invalid FEN: {}", e))?;
    let moves: Vec<String> = MoveGen::new_legal(&board)
        .map(|m| format!("{}", m))
        .collect();
    Ok(moves)
}

#[tauri::command]
pub fn make_move(fen: String, uci_move: String) -> MoveResult {
    let board = match Board::from_str(&fen) {
        Ok(b) => b,
        Err(e) => return MoveResult {
            success: false,
            new_state: None,
            error: Some(format!("Invalid FEN: {}", e)),
        },
    };
    
    // Parse the UCI move
    if uci_move.len() < 4 {
        return MoveResult {
            success: false,
            new_state: None,
            error: Some("Invalid move format".to_string()),
        };
    }
    
    let from = match Square::from_str(&uci_move[0..2]) {
        Ok(sq) => sq,
        Err(_) => return MoveResult {
            success: false,
            new_state: None,
            error: Some("Invalid source square".to_string()),
        },
    };
    
    let to = match Square::from_str(&uci_move[2..4]) {
        Ok(sq) => sq,
        Err(_) => return MoveResult {
            success: false,
            new_state: None,
            error: Some("Invalid destination square".to_string()),
        },
    };
    
    let promotion = if uci_move.len() == 5 {
        match uci_move.chars().nth(4).unwrap() {
            'q' => Some(Piece::Queen),
            'r' => Some(Piece::Rook),
            'b' => Some(Piece::Bishop),
            'n' => Some(Piece::Knight),
            _ => None,
        }
    } else {
        None
    };
    
    let chess_move = ChessMove::new(from, to, promotion);
    
    // Verify the move is legal
    let legal_moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();
    if !legal_moves.contains(&chess_move) {
        return MoveResult {
            success: false,
            new_state: None,
            error: Some("Illegal move".to_string()),
        };
    }
    
    let new_board = board.make_move_new(chess_move);
    
    MoveResult {
        success: true,
        new_state: Some(board_to_game_state(&new_board, Some(uci_move))),
        error: None,
    }
}

#[tauri::command]
pub fn get_engine_move(fen: String, engine_elo: i32) -> Result<EngineMove, String> {
    let board = Board::from_str(&fen).map_err(|e| format!("Invalid FEN: {}", e))?;
    
    // Get the best move (we'll add ELO-based move selection later)
    let best = Evaluator::find_best_move(&board)
        .ok_or_else(|| "No legal moves available".to_string())?;
    
    // For now, we return the best move. Later we'll add randomization based on ELO
    // Lower ELO = more likely to pick suboptimal moves
    let _strength_factor = (engine_elo as f32 / 2000.0).min(1.0);
    
    Ok(EngineMove {
        uci: format!("{}", best.chess_move),
        san: format!("{}", best.chess_move), // TODO: Convert to SAN
        evaluation: best.score as f32 / 100.0,
    })
}

#[tauri::command]
pub fn evaluate_position(fen: String) -> Result<f32, String> {
    let board = Board::from_str(&fen).map_err(|e| format!("Invalid FEN: {}", e))?;
    let eval = Evaluator::evaluate_position(&board);
    Ok(eval.score as f32 / 100.0)
}

#[tauri::command]
pub fn get_position_from_fen(fen: String) -> Result<GameState, String> {
    let board = Board::from_str(&fen).map_err(|e| format!("Invalid FEN: {}", e))?;
    Ok(board_to_game_state(&board, None))
}
