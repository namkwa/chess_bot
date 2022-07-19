use piece::piececolor::PieceColor::*;
use piece::Piece;
mod piece;
pub struct Board {
    pub board: [[piece::Piece; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        let mut board: [[Piece; 8]; 8] = [[Piece {
            name: ' ',
            color: Neither,
        }; 8]; 8];
        for i in 0..7 {
            board[1][i].name = 'P';
            board[1][i].color = White;
            board[6][i].name = 'P';
            board[1][i].color = Black;
            if i == 0 || i == 7 {
                board[0][i].name = 'R';
                board[0][i].color = White;
                board[7][i].name = 'R';
                board[7][i].color = Black;
            } else if i == 1 || i == 6 {
                board[0][i].name = 'N';
                board[0][i].color = White;
                board[7][i].name = 'N';
                board[7][i].color = Black;
            } else if i == 2 || i == 5 {
                board[0][i].name = 'B';
                board[0][i].color = White;
                board[7][i].name = 'B';
                board[7][i].color = Black;
            } else if i == 3 {
                board[0][i].name = 'Q';
                board[0][i].color = White;
                board[7][i].name = 'Q';
                board[7][i].color = Black;
            } else if i == 4 {
                board[0][i].name = 'K';
                board[0][i].color = White;
                board[7][i].name = 'K';
                board[7][i].color = Black;
            }
        }
        Board { board: board }
    }
}
