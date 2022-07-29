use core::fmt;

use piece::piece::Piece;
use piece::piececolor::PieceColor::*;
use piece::piecemove::PieceMove;
use piece::piecename::PieceName::*;

use self::piece::piececolor::PieceColor;
pub mod piece;
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
    pub current_player: PieceColor,
    pub possible_white_moves: Vec<PieceMove>,
    pub possible_black_moves: Vec<PieceMove>,
}

impl Board {
    pub fn new() -> Self {
        let mut board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        board[1] = [
            Some(Piece::new(Pawn, White, false)),
            Some(Piece::new(Pawn, White, false)),
            Some(Piece::new(Pawn, White, false)),
            Some(Piece::new(Pawn, White, false)),
            Some(Piece::new(Pawn, White, false)),
            Some(Piece::new(Pawn, White, false)),
            Some(Piece::new(Pawn, White, false)),
            Some(Piece::new(Pawn, White, false)),
        ];

        board[6] = [
            Some(Piece::new(Pawn, Black, false)),
            Some(Piece::new(Pawn, Black, false)),
            Some(Piece::new(Pawn, Black, false)),
            Some(Piece::new(Pawn, Black, false)),
            Some(Piece::new(Pawn, Black, false)),
            Some(Piece::new(Pawn, Black, false)),
            Some(Piece::new(Pawn, Black, false)),
            Some(Piece::new(Pawn, Black, false)),
        ];

        board[0] = [
            Some(Piece::new(Rook, White, false)),
            Some(Piece::new(Knight, White, false)),
            Some(Piece::new(Bishop, White, false)),
            Some(Piece::new(Queen, White, false)),
            Some(Piece::new(King, White, false)),
            Some(Piece::new(Bishop, White, false)),
            Some(Piece::new(Knight, White, false)),
            Some(Piece::new(Rook, White, false)),
        ];
        board[7] = [
            Some(Piece::new(Rook, Black, false)),
            Some(Piece::new(Knight, Black, false)),
            Some(Piece::new(Bishop, Black, false)),
            Some(Piece::new(Queen, Black, false)),
            Some(Piece::new(King, Black, false)),
            Some(Piece::new(Bishop, Black, false)),
            Some(Piece::new(Knight, Black, false)),
            Some(Piece::new(Rook, Black, false)),
        ];
        let possible_white_moves: Vec<PieceMove> = Vec::new();
        let possible_black_moves: Vec<PieceMove> = Vec::new();
        Board {
            board,
            current_player: White,
            possible_white_moves,
            possible_black_moves,
        }
    }
    pub fn compute_possible_moves(&mut self) {
        let mut next_possible_moves: Vec<PieceMove> = Vec::new();
        for (i, line) in self.board.into_iter().enumerate() {
            for (j, piece) in line.into_iter().enumerate() {
                if piece.is_none() || piece.unwrap().color != self.current_player {
                    continue;
                }
                match piece.unwrap().name {
                    Rook => next_possible_moves
                        .append(&mut piece.unwrap().rook(self.board, i as i32, j as i32)),
                    Knight => next_possible_moves
                        .append(&mut piece.unwrap().knight(self.board, i as i32, j as i32)),
                    Bishop => next_possible_moves
                        .append(&mut piece.unwrap().bishop(self.board, i as i32, j as i32)),
                    Queen => next_possible_moves
                        .append(&mut piece.unwrap().queen(self.board, i as i32, j as i32)),
                    King => next_possible_moves
                        .append(&mut piece.unwrap().king(self.board, i as i32, j as i32)),
                    Pawn => next_possible_moves
                        .append(&mut piece.unwrap().pawn(self.board, i as i32, j as i32)),
                }
            }
        }
        self.possible_white_moves = next_possible_moves;
    }

    pub fn execute_move(&mut self, current_position: (usize, usize), destination: (usize, usize)) {
        let mut move_to_execute: PieceMove = PieceMove {
            piece: self.board[current_position.0][current_position.1].unwrap(),
            destination: destination,
            current_position: current_position,
            takes: false,
            puts_in_check: false,
        };
        move_to_execute.piece.has_moved = true;
        self.board[move_to_execute.destination.0][move_to_execute.destination.1] =
            Some(move_to_execute.piece);
        self.board[move_to_execute.current_position.0][move_to_execute.current_position.1] = None;
    }

    pub fn compute_legal_destinations(self, x: i32, y: i32) -> (Vec<(usize, usize)>, bool) {
        let mut destinations: Vec<(usize, usize)> = Vec::new();
        let mut is_checked: bool = false;
        let coord_moves: [(i32, i32); 8] = [
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];
        for (i, j) in coord_moves {
            for distance in 1..8 {
                let new_x: i32 = distance * (x + i);
                let new_y: i32 = distance * (y + j);
                let is_inbound: bool = new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8;
                let new_x: usize = new_x as usize;
                let new_y: usize = new_y as usize;
                if is_inbound && self.board[new_x][new_y].is_none() {
                    destinations.push((new_x, new_y));
                } else if is_inbound
                    && self.board[new_x][new_y].unwrap().color == self.current_player
                {
                } else if is_inbound
                    && self.board[new_x][new_y].unwrap().color != self.current_player
                {
                }
            }
        }
        return (destinations, is_checked);
    }

    pub fn display_possible_moves(&self) {
        let mut output: String = String::new();
        for piece_move in self.possible_white_moves.iter() {
            output.push('(');
            output.push_str(&piece_move.destination.0.to_string());
            output.push(',');
            output.push_str(&piece_move.destination.1.to_string());
            output.push(',');
            output.push_str(&piece_move.piece.name.to_string());
            output.push_str(") ")
        }
        println!("{}", output);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = String::new();
        output.push_str("\n-----------------------------------------\n");
        for line in self.board.into_iter().rev() {
            output.push('|');
            for piece in line {
                if piece.is_none() {
                    output.push_str("    |");
                    continue;
                }
                match (piece.unwrap().name, piece.unwrap().color) {
                    (Rook, White) => output.push_str(" ♖  |"),
                    (Rook, Black) => output.push_str(" ♜  |"),
                    (Knight, White) => output.push_str(" ♘  |"),
                    (Knight, Black) => output.push_str(" ♞  |"),
                    (Bishop, White) => output.push_str(" ♗  |"),
                    (Bishop, Black) => output.push_str(" ♝  |"),
                    (Queen, White) => output.push_str(" ♕  |"),
                    (Queen, Black) => output.push_str(" ♛  |"),
                    (King, White) => output.push_str(" ♔  |"),
                    (King, Black) => output.push_str(" ♚  |"),
                    (Pawn, White) => output.push_str(" ♙  |"),
                    (Pawn, Black) => output.push_str(" ♟  |"),
                }
            }
            output.push_str("\n-----------------------------------------\n");
        }
        write!(f, "{}", output)
    }
}
