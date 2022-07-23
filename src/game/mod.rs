use self::board::Board;

pub mod board;

struct Game {
    pub board: Board,
}

impl Game {
    fn new(board: Board) -> Self {
        Game { board }
    }

    fn play() {
        let mut is_game_finished: bool = false;
        while !is_game_finished {}
    }
}
