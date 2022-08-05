use game::board::Board;
mod game;
fn main() {
    let mut board: Board = Board::new();
    board.execute_move((1, 3), (2, 3));
    board.execute_move((6, 4), (5, 4));
    board.execute_move((7, 5), (3, 1));
    board.execute_move((1, 2), (2, 2));
    board.compute_possible_moves();
    board.display_possible_moves();
    let result = board.compute_edge_cases(0, 4);
    println!("{}", board);
    println!("{} {:?}", result.3, result.2);
}
