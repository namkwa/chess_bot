use crate::board::Board;

use super::{
    piececolor::PieceColor, piececolor::PieceColor::*, piecemove::PieceMove, piecename::PieceName,
};
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    pub name: PieceName,
    pub color: PieceColor,
}

impl Piece {
    pub fn new(name: PieceName, color: PieceColor) -> Self {
        Piece { name, color }
    }

    pub fn rook(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> HashSet<PieceMove> {
        let mut rook_moves: HashSet<PieceMove> = HashSet::new();
        let coord_moves: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];
        for (i, j) in coord_moves {
            for distance in 1..8 {
                let new_x: i32 = distance * i + x;
                let new_y: i32 = distance * j + y;
                let (piece_move, should_break): (Option<PieceMove>, bool) =
                    self.compute_move(new_x, new_y, x as usize, y as usize, board);
                if !piece_move.is_none() {
                    rook_moves.insert(piece_move.unwrap());
                }
                if should_break {
                    break;
                }
            }
        }
        return rook_moves;
    }

    pub fn knight(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> HashSet<PieceMove> {
        let mut knight_moves: HashSet<PieceMove> = HashSet::new();
        let coord_moves: [(i32, i32); 8] = [
            (2, -1),
            (2, 1),
            (-2, -1),
            (-2, 1),
            (-1, 2),
            (1, 2),
            (-1, -2),
            (1, -2),
        ];
        for (i, j) in coord_moves {
            let new_x: i32 = x + i;
            let new_y: i32 = y + j;
            let (piece_move, _): (Option<PieceMove>, bool) =
                self.compute_move(new_x, new_y, x as usize, y as usize, board);
            if !piece_move.is_none() {
                knight_moves.insert(piece_move.unwrap());
            }
        }
        return knight_moves;
    }

    pub fn bishop(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> HashSet<PieceMove> {
        let mut bishop_moves: HashSet<PieceMove> = HashSet::new();
        let coord_moves: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        for (i, j) in coord_moves {
            for distance in 1..8 {
                let new_x: i32 = distance * i + x;
                let new_y: i32 = distance * j + y;
                let (piece_move, should_break): (Option<PieceMove>, bool) =
                    self.compute_move(new_x, new_y, x as usize, y as usize, board);
                if !piece_move.is_none() {
                    bishop_moves.insert(piece_move.unwrap());
                }
                if should_break {
                    break;
                }
            }
        }
        return bishop_moves;
    }

    pub fn queen(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> HashSet<PieceMove> {
        let mut queen_moves: HashSet<PieceMove> = HashSet::new();
        queen_moves.extend(&mut self.rook(board, x, y).iter());
        queen_moves.extend(&mut self.bishop(board, x, y).iter());
        return queen_moves;
    }

    pub fn king(self, board: &mut Board, x: i32, y: i32) -> HashSet<PieceMove> {
        let mut king_moves: HashSet<PieceMove> = HashSet::new();
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
            let new_x: i32 = x + i;
            let new_y: i32 = y + j;
            let king_position: (usize, usize) = if self.color == White {
                board.white_king_position
            } else {
                board.black_king_position
            };
            let (piece_move, _): (Option<PieceMove>, bool) =
                self.compute_move(new_x, new_y, x as usize, y as usize, board.board);
            if !piece_move.is_none()
                && !board
                    .compute_edge_cases(
                        king_position.0.try_into().unwrap(),
                        king_position.1.try_into().unwrap(),
                    )
                    .1
            {
                king_moves.insert(piece_move.unwrap());
            }
        }
        return king_moves;
    }

    pub fn pawn(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> HashSet<PieceMove> {
        let mut pawn_moves: HashSet<PieceMove> = HashSet::new();

        let new_x: i32 = x + 1;
        let new_y: i32 = y;
        let (piece_move, _): (Option<PieceMove>, bool) =
            self.compute_move(new_x, new_y, x as usize, y as usize, board);
        if !piece_move.is_none() {
            pawn_moves.insert(piece_move.unwrap());
        }
        return pawn_moves;
    }

    fn compute_move(
        self,
        new_x: i32,
        new_y: i32,
        old_x: usize,
        old_y: usize,
        board: [[Option<Piece>; 8]; 8],
    ) -> (Option<PieceMove>, bool) {
        let is_inbound: bool = new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8;
        let new_x: usize = new_x as usize;
        let new_y: usize = new_y as usize;
        if is_inbound && board[new_x][new_y].is_none() {
            return (
                Some(PieceMove {
                    piece: self,
                    destination: (new_x, new_y),
                    current_position: (old_x, old_y),
                }),
                false,
            );
        } else if is_inbound && board[new_x][new_y].unwrap().color != self.color {
            return (
                Some(PieceMove {
                    piece: self,
                    destination: (new_x, new_y),
                    current_position: (old_x, old_y),
                }),
                true,
            );
        } else {
            return (None, true);
        }
    }
}
