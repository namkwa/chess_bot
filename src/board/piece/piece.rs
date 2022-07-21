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

    pub fn bishop(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> Vec<PieceMove> {
        let mut bishop_moves: Vec<PieceMove> = Vec::new();
        let coord_moves: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        for (i, j) in coord_moves {
            for distance in 1..8 {
                let new_x: i32 = distance * (x + i);
                let new_y: i32 = distance * (y + j);
                let is_inbound: bool = new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8;
                let mut is_same_color: bool = false;
                let new_usized_x: usize = new_x as usize;
                let new_usized_y: usize = new_y as usize;
                if is_inbound && !(board[new_usized_x][new_usized_y] == None) {
                    is_same_color = board[new_usized_x][new_usized_y].unwrap().color == self.color;
                }
                if is_inbound && board[new_usized_x][new_usized_y] == None {
                    bishop_moves.push(PieceMove {
                        piece: self,
                        destination: (new_usized_x, new_usized_y),
                        takes: false,
                        puts_in_check: false,
                    })
                } else if is_inbound && !is_same_color {
                    bishop_moves.push(PieceMove {
                        piece: self,
                        destination: (new_usized_x, new_usized_y),
                        takes: true,
                        puts_in_check: false,
                    });
                    break;
                } else {
                    break;
                }
            }
        }
        return bishop_moves;
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
            let is_inbound: bool = new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8;
            let new_usized_x: usize = new_x as usize;
            let new_usized_y: usize = new_y as usize;
            let mut is_same_color: bool = false;
            if is_inbound && !(board[new_usized_x][new_usized_y] == None) {
                is_same_color = board[new_usized_x][new_usized_y].unwrap().color == self.color;
            }
            if is_inbound && board[new_usized_x][new_usized_y] == None {
                knight_moves.push(PieceMove {
                    piece: self,
                    destination: (new_usized_x, new_usized_y),
                    takes: false,
                    puts_in_check: false,
                })
            } else if is_inbound && !is_same_color {
                knight_moves.push(PieceMove {
                    piece: self,
                    destination: (new_usized_x, new_usized_y),
                    takes: true,
                    puts_in_check: false,
                });
            }
        }
        return knight_moves;
    }

    pub fn rook(self, board: [[Option<Piece>; 8]; 8], x: i32, y: i32) -> Vec<PieceMove> {
        let mut rook_moves: Vec<PieceMove> = Vec::new();
        let coord_moves: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];
        for (i, j) in coord_moves {
            for distance in 1..8 {
                let new_x: i32 = distance * (x + i);
                let new_y: i32 = distance * (y + j);
                let is_inbound: bool = new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8;
                let new_usized_x: usize = new_x as usize;
                let new_usized_y: usize = new_y as usize;
                let mut is_same_color: bool = false;
                if is_inbound && !(board[new_usized_x][new_usized_y] == None) {
                    is_same_color = board[new_usized_x][new_usized_y].unwrap().color == self.color;
                }
                if is_inbound && board[new_usized_x][new_usized_y] == None {
                    rook_moves.push(PieceMove {
                        piece: self,
                        destination: (new_usized_x, new_usized_y),
                        takes: false,
                        puts_in_check: false,
                    })
                } else if is_inbound && !is_same_color {
                    rook_moves.push(PieceMove {
                        piece: self,
                        destination: (new_usized_x, new_usized_y),
                        takes: true,
                        puts_in_check: false,
                    });
                    break;
                } else {
                    break;
                }
            }
        }
        return rook_moves;
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
            let is_inbound: bool = new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8;
            let new_usized_x: usize = new_x as usize;
            let new_usized_y: usize = new_y as usize;
            let mut is_same_color: bool = false;
            if is_inbound && !(board[new_usized_x][new_usized_y] == None) {
                is_same_color = board[new_usized_x][new_usized_y].unwrap().color == self.color;
            }
            if is_inbound && board[new_usized_x][new_usized_y] == None {
                king_moves.push(PieceMove {
                    piece: self,
                    destination: (new_usized_x, new_usized_y),
                    takes: false,
                    puts_in_check: false,
                })
            } else if is_inbound && !is_same_color {
                king_moves.push(PieceMove {
                    piece: self,
                    destination: (new_usized_x, new_usized_y),
                    takes: true,
                    puts_in_check: false,
                });
                continue;
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
}
