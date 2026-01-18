use colored::*;
use chess::{Board, Color};

pub fn print_board(board: &Board, player_color: Color) {
    println!();

    let ranks: Vec<usize> = if player_color == Color::White {
        (0..8).rev().collect()
    } else {
        (0..8).collect()
    };

    for rank in ranks {
        print!("{}  ", rank + 1);

        let files: Vec<usize> = if player_color == Color::White {
            (0..8).collect()
        } else {
            (0..8).rev().collect()
        };

        for file in files {
            let square = chess::Square::make_square(
                chess::Rank::from_index(rank),
                chess::File::from_index(file),
            );

            let piece_char = if let Some(piece) = board.piece_on(square) {
                let color = board.color_on(square).unwrap();
                let symbol = match piece {
                    chess::Piece::Pawn => "♙",
                    chess::Piece::Knight => "♘",
                    chess::Piece::Bishop => "♗",
                    chess::Piece::Rook => "♖",
                    chess::Piece::Queen => "♕",
                    chess::Piece::King => "♔",
                };

                if color == Color::White {
                    symbol.white().to_string()
                } else {
                    symbol.blue().to_string()
                }
            } else {
                ".".to_string()
            };

            print!("{} ", piece_char);
        }
        println!();
    }

    print!("\n   ");
    let file_labels = if player_color == Color::White {
        vec!["a", "b", "c", "d", "e", "f", "g", "h"]
    } else {
        vec!["h", "g", "f", "e", "d", "c", "b", "a"]
    };

    for label in file_labels {
        print!("{} ", label);
    }
    println!("\n");
}

pub fn print_header(text: &str) {
    println!("\n{}", "=".repeat(60).bright_cyan());
    println!("{}", text.bright_cyan().bold());
    println!("{}\n", "=".repeat(60).bright_cyan());
}

pub fn print_success(text: &str) {
    println!("{} {}", "✓".green().bold(), text.green());
}

pub fn print_error(text: &str) {
    println!("{} {}", "✗".red().bold(), text.red());
}

pub fn print_info(text: &str) {
    println!("{} {}", "ℹ".blue().bold(), text);
}

pub fn print_warning(text: &str) {
    println!("{} {}", "⚠".yellow().bold(), text.yellow());
}

pub fn get_user_input(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{} ", prompt.cyan());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
