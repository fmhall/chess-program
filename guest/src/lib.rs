#![no_main]

use chess::{Board, ChessMove};
use std::str::FromStr;

#[jolt::provable]
fn validate_move(fen: String, san: String) -> bool {
    
    
    // Generate the chessboard from the FEN input
    let b = Board::from_str(&fen).expect("valid FEN board");

    // Try to parse the SAN as a legal chess move
    let is_valid_move = ChessMove::from_san(&b, &san).is_ok();

    is_valid_move
}
