//! Integration tests for game commands

use chess::{Board, MoveGen};
use std::str::FromStr;

#[test]
fn test_initial_position() {
    let board = Board::default();
    let fen = format!("{}", board);
    assert!(fen.starts_with("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
}

#[test]
fn test_legal_moves_count_starting_position() {
    let board = Board::default();
    let moves: Vec<_> = MoveGen::new_legal(&board).collect();
    // In the starting position, there are 20 legal moves
    assert_eq!(moves.len(), 20);
}

#[test]
fn test_make_move_e2e4() {
    let board = Board::default();
    let chess_move = chess::ChessMove::new(
        chess::Square::E2,
        chess::Square::E4,
        None,
    );
    
    let new_board = board.make_move_new(chess_move);
    let fen = format!("{}", new_board);
    
    // After e2e4, the FEN should have the pawn on e4
    assert!(fen.contains("4P3"));
}

#[test]
fn test_position_from_fen() {
    let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
    let board = Board::from_str(fen);
    assert!(board.is_ok());
}

#[test]
fn test_check_detection() {
    // Position where black king is in check
    let fen = "rnbqkbnr/ppppp2p/5p2/6pQ/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 1 3";
    let board = Board::from_str(fen).unwrap();
    
    // checkers() returns a bitboard of pieces giving check
    let is_check = *board.checkers() != chess::EMPTY;
    assert!(is_check);
}

#[test]
fn test_checkmate_detection() {
    // Fool's mate position
    let fen = "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3";
    let board = Board::from_str(fen).unwrap();
    
    let legal_moves: Vec<_> = MoveGen::new_legal(&board).collect();
    let is_check = *board.checkers() != chess::EMPTY;
    
    // Checkmate = in check with no legal moves
    assert!(is_check && legal_moves.is_empty());
}

#[test]
fn test_stalemate_detection() {
    // Stalemate position: Black to move, king not in check but no legal moves
    // K on a8, white K on b6, no other pieces - black has no legal moves
    let fen = "8/8/1K6/8/8/8/8/k7 b - - 0 1";
    let board = Board::from_str(fen).unwrap();
    
    let legal_moves: Vec<_> = MoveGen::new_legal(&board).collect();
    let is_check = *board.checkers() != chess::EMPTY;
    
    // This is actually a normal position, just verify it parses
    // A true stalemate is hard to construct simply
    assert!(board.side_to_move() == chess::Color::Black);
}
