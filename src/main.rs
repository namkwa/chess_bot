mod board;
fn main() {
    let board: board::Board = board::Board::new();
    println!("{}", board.board[0][0].name);
}
