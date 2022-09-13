use std::{error, fmt::Error};

use board::Board;
mod board;
fn main() {
    let mut board: Board =
        Board::from("rnbqk1nr/pppp1ppp/8/2b1p3/4P3/8/PPPPKPPP/RNBQ1BNR w kq - 2 3".to_string());
    println!(
        "{} {:?}",
        board.current_player, board.castling_rights.can_black_castle_king_side
    );
    board.compute_possible_moves();
    board.display_possible_moves();
    println!("{}", board);
}
