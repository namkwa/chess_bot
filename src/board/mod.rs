use piece::piece::Piece;
use piece::piececolor::PieceColor::*;
use piece::piecemove::PieceMove;
use piece::piecename::PieceName::*;
mod piece;
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
    pub possible_Moves: Vec<PieceMove>,
}

impl Board {
    pub fn new() -> Self {
        let mut board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        for i in 0..7 {
            board[1][i] = Some(Piece::new(Pawn, White));
            board[6][i] = Some(Piece::new(Pawn, Black));
            if i == 0 || i == 7 {
                board[0][i] = Some(Piece::new(Rook, Black));
                board[7][i] = Some(Piece::new(Rook, Black));
            } else if i == 1 || i == 6 {
                board[0][i] = Some(Piece::new(Knight, Black));
                board[7][i] = Some(Piece::new(Knight, Black));
            } else if i == 2 || i == 5 {
                board[0][i] = Some(Piece::new(Bishop, Black));
                board[7][i] = Some(Piece::new(Bishop, Black));
            } else if i == 3 {
                board[0][i] = Some(Piece::new(Queen, Black));
                board[7][i] = Some(Piece::new(Queen, Black));
            } else if i == 4 {
                board[0][i] = Some(Piece::new(King, Black));
                board[7][i] = Some(Piece::new(King, Black));
            }
        }
        Board { board }
    }
}
