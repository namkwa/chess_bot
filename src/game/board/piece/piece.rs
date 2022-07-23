use super::{
    piececolor::PieceColor, piececolor::PieceColor::*, piecemove::PieceMove, piecename::PieceName,
};

#[derive(Copy, Clone, PartialEq)]
pub struct Piece {
    pub name: PieceName,
    pub color: PieceColor,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(name: PieceName, color: PieceColor, has_moved: bool) -> Self {
        Piece {
            name,
            color,
            has_moved,
        }
    }

    pub fn rook(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> Vec<PieceMove> {
        let mut rook_moves: Vec<PieceMove> = Vec::new();
        let coord_moves: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];
        for (i, j) in coord_moves {
            for distance in 1..8 {
                let new_x: i32 = distance * (x + i);
                let new_y: i32 = distance * (y + j);
                let (piece_move, should_break): (Option<PieceMove>, bool) =
                    self.compute_move(new_x, new_y, board);
                if !piece_move.is_none() {
                    rook_moves.push(piece_move.unwrap());
                }
                if should_break {
                    break;
                }
            }
        }
        return rook_moves;
    }

    pub fn knight(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> Vec<PieceMove> {
        let mut knight_moves: Vec<PieceMove> = Vec::new();
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
            let (piece_move, should_break): (Option<PieceMove>, bool) =
                self.compute_move(new_x, new_y, board);
            if !piece_move.is_none() {
                knight_moves.push(piece_move.unwrap());
            }
            if should_break {
                break;
            }
        }
        return knight_moves;
    }

    pub fn bishop(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> Vec<PieceMove> {
        let mut bishop_moves: Vec<PieceMove> = Vec::new();
        let coord_moves: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        for (i, j) in coord_moves {
            for distance in 1..8 {
                let new_x: i32 = distance * (x + i);
                let new_y: i32 = distance * (y + j);
                let (piece_move, should_break): (Option<PieceMove>, bool) =
                    self.compute_move(new_x, new_y, board);
                if !piece_move.is_none() {
                    bishop_moves.push(piece_move.unwrap());
                }
                if should_break {
                    break;
                }
            }
        }
        return bishop_moves;
    }

    pub fn queen(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> Vec<PieceMove> {
        let mut queen_moves: Vec<PieceMove> = Vec::new();
        queen_moves.append(&mut self.rook(board, x, y));
        queen_moves.append(&mut self.bishop(board, x, y));
        return queen_moves;
    }

    pub fn king(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> Vec<PieceMove> {
        let mut king_moves: Vec<PieceMove> = Vec::new();
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
            let (piece_move, should_break): (Option<PieceMove>, bool) =
                self.compute_move(new_x, new_y, board);
            if !piece_move.is_none() {
                king_moves.push(piece_move.unwrap());
            }
            if should_break {
                break;
            }
        }
        return king_moves;
    }

    pub fn pawn(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> Vec<PieceMove> {
        let mut pawn_moves: Vec<PieceMove> = Vec::new();

        //checks if the pawn is black or white
        let direction: i32 = if self.color == White { 1 } else { -1 };
        let new_x: i32 = x + direction;
        if x == 1
            && board[new_x as usize][y as usize] == None
            && board[(new_x + direction) as usize][y as usize] == None
        {
            pawn_moves.push(PieceMove {
                piece: self,
                destination: ((new_x + direction) as usize, y as usize),
                takes: false,
                puts_in_check: false,
            })
        }
        if board[new_x as usize][y as usize] == None {
            pawn_moves.push(PieceMove {
                piece: self,
                destination: (new_x as usize, y as usize),
                takes: false,
                puts_in_check: false,
            })
        }
        return pawn_moves;
    }
    fn compute_move(
        self,
        new_x: i32,
        new_y: i32,
        board: [[Option<Piece>; 8]; 8],
    ) -> (Option<PieceMove>, bool) {
        let is_inbound: bool = new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8;
        let new_x: usize = new_x as usize;
        let new_y: usize = new_y as usize;
        let mut is_same_color: bool = false;
        if is_inbound && !(board[new_x][new_y] == None) {
            is_same_color = board[new_x][new_y].unwrap().color == self.color;
        }
        if is_inbound && board[new_x][new_y] == None {
            return (
                Some(PieceMove {
                    piece: self,
                    destination: (new_x, new_y),
                    takes: false,
                    puts_in_check: false,
                }),
                false,
            );
        } else if is_inbound && !is_same_color {
            return (
                Some(PieceMove {
                    piece: self,
                    destination: (new_x, new_y),
                    takes: true,
                    puts_in_check: false,
                }),
                true,
            );
        } else {
            return (None, true);
        }
    }
}
