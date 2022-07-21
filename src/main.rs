use board::Board;
mod board;
fn main() {
    let mut board: Board = Board::new();
    board.compute_possible_moves();
}
