use std::{error, fmt::Error};

use board::Board;
mod board;
fn main() {
    let mut board: Board =
        Board::from("rnbq1bnr/ppppkppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR w - - 2 3".to_string());
    println!(
        "{} {:?}",
        board.current_player, board.castling_rights.can_black_castle_king_side
    );
    board.compute_possible_moves();
    board.display_possible_moves();
}
