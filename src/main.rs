use game::board::Board;
mod game;
fn main() {
    let board: Board = Board::new();
    println!("{}", board);
}
