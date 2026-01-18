use anyhow::Result;
use chess::{Board, ChessMove, Color};
use chess_core::ChessGame;
use chess_engine::Evaluator;
use crate::ui;
use std::str::FromStr;

pub async fn play_game(player_color: Color) -> Result<ChessGame> {
    let mut game = ChessGame::new(player_color);

    ui::print_header(&format!("Starting New Game - You are {:?}", player_color));

    loop {
        ui::print_board(&game.board, player_color);

        if game.is_finished() {
            match &game.state {
                chess_core::GameState::Checkmate(winner) => {
                    if *winner == player_color {
                        ui::print_success("Checkmate! You win!");
                    } else {
                        ui::print_error("Checkmate! You lose!");
                    }
                }
                chess_core::GameState::Stalemate => {
                    ui::print_info("Stalemate! It's a draw.");
                }
                _ => {
                    ui::print_info("Game over!");
                }
            }
            break;
        }

        let current_turn = game.current_turn();

        if current_turn == player_color {
            // Player's turn
            ui::print_info(&format!("Your turn ({:?})", player_color));

            let legal_moves = game.legal_moves();
            println!("Legal moves: {}", legal_moves.len());

            let user_move = loop {
                let input = ui::get_user_input("Enter your move (e.g., e2e4) or 'hint' for help:");

                if input.to_lowercase() == "hint" {
                    if let Some(best_move) = Evaluator::find_best_move(&game.board) {
                        ui::print_info(&format!(
                            "Hint: Consider {} (eval: {})",
                            best_move.chess_move, best_move.score
                        ));
                    }
                    continue;
                }

                if input.to_lowercase() == "quit" {
                    ui::print_warning("Quitting game...");
                    return Ok(game);
                }

                match parse_move(&input, &game.board) {
                    Ok(chess_move) => {
                        if legal_moves.contains(&chess_move) {
                            break chess_move;
                        } else {
                            ui::print_error("Illegal move! Try again.");
                        }
                    }
                    Err(e) => {
                        ui::print_error(&format!("Invalid move format: {}", e));
                    }
                }
            };

            game.make_move(user_move)?;
        } else {
            // Computer's turn
            ui::print_info("Computer is thinking...");

            if let Some(best_move) = Evaluator::find_best_move(&game.board) {
                ui::print_info(&format!(
                    "Computer plays: {} (eval: {})",
                    best_move.chess_move, best_move.score
                ));
                game.make_move(best_move.chess_move)?;
            } else {
                ui::print_error("Computer has no legal moves!");
                break;
            }
        }
    }

    Ok(game)
}

fn parse_move(input: &str, board: &Board) -> Result<ChessMove> {
    // Try to parse as UCI format (e.g., "e2e4")
    if input.len() >= 4 {
        let from_str = &input[0..2];
        let to_str = &input[2..4];

        let from = chess::Square::from_str(from_str)
            .map_err(|e| anyhow::anyhow!("Invalid from square: {:?}", e))?;
        let to = chess::Square::from_str(to_str)
            .map_err(|e| anyhow::anyhow!("Invalid to square: {:?}", e))?;

        // Check for promotion
        let promotion = if input.len() == 5 {
            let promo_char = input.chars().nth(4).unwrap();
            Some(match promo_char.to_lowercase().next().unwrap() {
                'q' => chess::Piece::Queen,
                'r' => chess::Piece::Rook,
                'b' => chess::Piece::Bishop,
                'n' => chess::Piece::Knight,
                _ => return Err(anyhow::anyhow!("Invalid promotion piece")),
            })
        } else {
            None
        };

        Ok(ChessMove::new(from, to, promotion))
    } else {
        Err(anyhow::anyhow!("Move must be at least 4 characters (e.g., e2e4)"))
    }
}
