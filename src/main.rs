pub fn main() {
    let (prove_validate_move, verify_validate_move) = guest::build_validate_move();

    // FEN representation of a chessboard in its initial state
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

    // SAN representation Queen's pawn opening
    let san = "d4".to_string();

    let (output, proof) = prove_validate_move(fen, san);
    let is_valid = verify_validate_move(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);
}
