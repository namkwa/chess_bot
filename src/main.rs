use game::board::piece::piecemove::PieceMove;
use game::board::Board;
mod game;
fn main() {
    let mut board: Board = Board::new();
    board.execute_move((0, 1), (2, 0));
    board.compute_possible_moves();
    board.display_possible_moves();
    println!("{}", board);
}
