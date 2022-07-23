use core::fmt;

use piece::piece::Piece;
use piece::piececolor::PieceColor::*;
use piece::piecemove::PieceMove;
use piece::piecename::PieceName::*;

use self::piece::piececolor::PieceColor;
mod piece;
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
    pub current_player: PieceColor,
    pub possible_moves: Vec<PieceMove>,
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
        let possible_moves: Vec<PieceMove> = Vec::new();
        Board {
            board,
            current_player: White,
            possible_moves,
        }
    }
    pub fn compute_possible_moves(&mut self) {
        let mut next_possible_moves: Vec<PieceMove> = Vec::new();
        for (i, line) in self.board.into_iter().enumerate() {
            for (j, piece) in line.into_iter().enumerate() {
                if piece.is_none() {
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
        self.possible_moves = next_possible_moves;
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
