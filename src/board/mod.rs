use core::fmt;
use std::collections::HashSet;

use piece::checksandpins::ChecksAndPins;
use piece::piece::Piece;
use piece::piececolor::PieceColor::*;
use piece::piecemove::PieceMove;
use piece::piecename::PieceName::*;
use piece::side::Side::Both;

use self::piece::castlingrights::CastlingRights;
use self::piece::piececolor::PieceColor;
use self::piece::side::Side;
pub mod piece;
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
    pub current_player: PieceColor,
    pub possible_moves: HashSet<PieceMove>,
    pub white_king_position: (usize, usize),
    pub black_king_position: (usize, usize),
    pub castling_rights: CastlingRights,
    pub en_passant: Option<(usize, usize)>,
}

impl Board {
    pub fn new() -> Self {
        let mut board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        board[1] = [
            Some(Piece::new(Pawn, White)),
            Some(Piece::new(Pawn, White)),
            Some(Piece::new(Pawn, White)),
            Some(Piece::new(Pawn, White)),
            Some(Piece::new(Pawn, White)),
            Some(Piece::new(Pawn, White)),
            Some(Piece::new(Pawn, White)),
            Some(Piece::new(Pawn, White)),
        ];

        board[6] = [
            Some(Piece::new(Pawn, Black)),
            Some(Piece::new(Pawn, Black)),
            Some(Piece::new(Pawn, Black)),
            Some(Piece::new(Pawn, Black)),
            Some(Piece::new(Pawn, Black)),
            Some(Piece::new(Pawn, Black)),
            Some(Piece::new(Pawn, Black)),
            Some(Piece::new(Pawn, Black)),
        ];

        board[0] = [
            Some(Piece::new(Rook, White)),
            Some(Piece::new(Knight, White)),
            Some(Piece::new(Bishop, White)),
            Some(Piece::new(Queen, White)),
            Some(Piece::new(King, White)),
            Some(Piece::new(Bishop, White)),
            Some(Piece::new(Knight, White)),
            Some(Piece::new(Rook, White)),
        ];
        board[7] = [
            Some(Piece::new(Rook, Black)),
            Some(Piece::new(Knight, Black)),
            Some(Piece::new(Bishop, Black)),
            Some(Piece::new(Queen, Black)),
            Some(Piece::new(King, Black)),
            Some(Piece::new(Bishop, Black)),
            Some(Piece::new(Knight, Black)),
            Some(Piece::new(Rook, Black)),
        ];
        let possible_moves: HashSet<PieceMove> = HashSet::new();
        Board {
            board,
            current_player: White,
            possible_moves,
            white_king_position: (0, 4),
            black_king_position: (7, 4),
            castling_rights: CastlingRights::new(),
            en_passant: None,
        }
    }

    pub fn from(fen: String) -> Self {
        let mut board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        let mut fen_splitted = fen.split_whitespace();
        let positions = fen_splitted.next().unwrap();
        let splitted_positions = positions.split("/");
        let mut white_king_positions: (usize, usize) = (0, 4);
        let mut black_king_positions: (usize, usize) = (7, 4);
        for (row, row_positions) in splitted_positions.into_iter().enumerate() {
            let row_index: usize = 7 - row;
            let mut col_index: usize = 0;
            for piece in row_positions.chars() {
                if piece.is_numeric() {
                    col_index += piece.to_digit(10).unwrap() as usize;
                    continue;
                }
                let piece_color = if piece.is_uppercase() { White } else { Black };
                match piece.to_ascii_lowercase() {
                    'k' => {
                        board[row_index][col_index] = Some(Piece::new(King, piece_color));
                        match piece_color {
                            White => white_king_positions = (row_index, col_index),
                            Black => black_king_positions = (row_index, col_index),
                        }
                    }

                    'q' => board[row_index][col_index] = Some(Piece::new(Queen, piece_color)),
                    'r' => board[row_index][col_index] = Some(Piece::new(Rook, piece_color)),
                    'b' => board[row_index][col_index] = Some(Piece::new(Bishop, piece_color)),
                    'n' => board[row_index][col_index] = Some(Piece::new(Knight, piece_color)),
                    'p' => board[row_index][col_index] = Some(Piece::new(Pawn, piece_color)),
                    _ => panic!(),
                }
                col_index += 1;
            }
        }
        let current_player = if fen_splitted.next() == Some("w") {
            White
        } else {
            Black
        };
        let mut castling_rights = CastlingRights::new();
        let castling_rights_splitted = fen_splitted.next().unwrap();
        println!("{}", castling_rights_splitted);
        if !castling_rights_splitted.contains("K") {
            castling_rights.remove_castling_rights(White, Side::King)
        }
        if !castling_rights_splitted.contains("k") {
            castling_rights.remove_castling_rights(Black, Side::King)
        }
        if !castling_rights_splitted.contains("Q") {
            castling_rights.remove_castling_rights(White, Side::Queen)
        }
        if !castling_rights_splitted.contains("q") {
            castling_rights.remove_castling_rights(Black, Side::Queen)
        }

        let possible_moves: HashSet<PieceMove> = HashSet::new();
        Board {
            board,
            current_player: current_player,
            possible_moves,
            white_king_position: white_king_positions,
            black_king_position: black_king_positions,
            castling_rights: castling_rights,
            en_passant: None,
        }
    }

    pub fn execute_move(&mut self, current_position: (usize, usize), destination: (usize, usize)) {
        let mut move_to_execute: PieceMove = PieceMove {
            piece: self.board[current_position.0][current_position.1].unwrap(),
            destination: destination,
            current_position: current_position,
        };
        if move_to_execute.piece.name == King {
            _ = &mut self
                .castling_rights
                .remove_castling_rights(move_to_execute.piece.color, Both);
            match move_to_execute.piece.color {
                White => self.white_king_position = move_to_execute.destination,
                Black => self.black_king_position = move_to_execute.destination,
            }
        } else if move_to_execute.piece.name == Rook {
            match current_position {
                (0, 0) => self
                    .castling_rights
                    .remove_castling_rights(White, Side::Queen),
                (0, 7) => self
                    .castling_rights
                    .remove_castling_rights(White, Side::King),
                (7, 0) => self
                    .castling_rights
                    .remove_castling_rights(Black, Side::Queen),
                (7, 7) => self
                    .castling_rights
                    .remove_castling_rights(Black, Side::King),
                _ => (),
            }
        }
        self.board[move_to_execute.destination.0][move_to_execute.destination.1] =
            Some(move_to_execute.piece);
        self.board[move_to_execute.current_position.0][move_to_execute.current_position.1] = None;
    }

    pub fn look_for_checks_and_pins_by_rook(&self, x: i32, y: i32) -> ChecksAndPins {
        let mut destinations: HashSet<(usize, usize)> = HashSet::new();
        let mut current_positions: HashSet<(usize, usize)> = HashSet::new();
        let mut is_checked: bool = false;
        let mut is_pinned: bool = false;
        let coord_moves: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];
        for (i, j) in coord_moves {
            let mut has_met_same_color_piece: bool = false;
            let mut temp_destinations: HashSet<(usize, usize)> = HashSet::new();
            let mut temp_current_positions: HashSet<(usize, usize)> = HashSet::new();
            for distance in 1..8 {
                let new_x: i32 = distance * i + x;
                let new_y: i32 = distance * j + y;
                if !(new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8) {
                    break;
                }
                let new_x: usize = new_x as usize;
                let new_y: usize = new_y as usize;
                let square: Option<Piece> = self.board[new_x][new_y];
                if square.is_none() {
                    temp_destinations.insert((new_x, new_y));
                } else if square.unwrap().color == self.current_player && !has_met_same_color_piece
                {
                    has_met_same_color_piece = true;
                    temp_current_positions.insert((new_x, new_y));
                } else if square.unwrap().color == self.current_player && has_met_same_color_piece {
                    break;
                } else if square.unwrap().color != self.current_player
                    && has_met_same_color_piece
                    && (square.unwrap().name == Rook || square.unwrap().name == Queen)
                {
                    is_pinned = true;
                    current_positions.extend(temp_current_positions.iter());
                    break;
                } else if square.unwrap().color != self.current_player
                    && !has_met_same_color_piece
                    && (square.unwrap().name == Rook || square.unwrap().name == Queen)
                {
                    temp_destinations.insert((new_x, new_y));
                    is_checked = true;
                    destinations.extend(temp_destinations.iter());
                    break;
                } else {
                    break;
                }
            }
        }
        ChecksAndPins {
            destinations,
            is_checked,
            current_positions: current_positions,
            is_pinned: is_pinned,
        }
    }

    pub fn look_for_checks_by_knight(&self, x: i32, y: i32) -> ChecksAndPins {
        let mut destinations: HashSet<(usize, usize)> = HashSet::new();
        let mut is_checked: bool = false;
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
            let new_x: i32 = i + x;
            let new_y: i32 = j + y;
            let is_inbound: bool = new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8;
            let new_x: usize = new_x as usize;
            let new_y: usize = new_y as usize;
            if !is_inbound {
                break;
            }
            let square: Option<Piece> = self.board[new_x][new_y];
            if is_inbound
                && !self.board[new_x][new_y].is_none()
                && self.current_player != square.unwrap().color
                && square.unwrap().name == Knight
            {
                is_checked = true;
                destinations.insert((new_x, new_y));
            }
        }
        ChecksAndPins {
            destinations,
            is_checked,
            current_positions: HashSet::new(),
            is_pinned: false,
        }
    }

    pub fn look_for_checks_and_pins_by_bishop(&self, x: i32, y: i32) -> ChecksAndPins {
        let mut destinations: HashSet<(usize, usize)> = HashSet::new();
        let mut current_positions: HashSet<(usize, usize)> = HashSet::new();
        let mut is_checked: bool = false;
        let mut is_pinned: bool = false;
        let coord_moves: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        for (i, j) in coord_moves {
            let mut has_met_same_color_piece: bool = false;
            let mut temp_destinations: HashSet<(usize, usize)> = HashSet::new();
            let mut temp_current_positions: HashSet<(usize, usize)> = HashSet::new();
            for distance in 1..8 {
                let new_x: i32 = distance * i + x;
                let new_y: i32 = distance * j + y;
                let is_inbound: bool = new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8;
                if !is_inbound {
                    break;
                }
                let new_x: usize = new_x as usize;
                let new_y: usize = new_y as usize;
                let square: Option<Piece> = self.board[new_x][new_y];
                if is_inbound && self.board[new_x][new_y].is_none() {
                    temp_destinations.insert((new_x, new_y));
                } else if square.unwrap().color == self.current_player && !has_met_same_color_piece
                {
                    has_met_same_color_piece = true;
                    temp_current_positions.insert((new_x, new_y));
                } else if square.unwrap().color == self.current_player && has_met_same_color_piece {
                    break;
                } else if square.unwrap().color != self.current_player
                    && has_met_same_color_piece
                    && (square.unwrap().name == Bishop || square.unwrap().name == Queen)
                {
                    is_pinned = true;
                    current_positions.extend(temp_current_positions.iter());
                    break;
                } else if square.unwrap().color != self.current_player
                    && !has_met_same_color_piece
                    && (square.unwrap().name == Bishop || square.unwrap().name == Queen)
                {
                    temp_destinations.insert((new_x, new_y));
                    is_checked = true;
                    destinations.extend(temp_destinations.iter());
                    break;
                }
            }
        }
        ChecksAndPins {
            destinations,
            is_checked,
            current_positions: current_positions,
            is_pinned: is_pinned,
        }
    }

    pub fn look_for_checks_and_pins(&self, x: i32, y: i32) -> ChecksAndPins {
        let checks_and_pins_by_rook: ChecksAndPins = self.look_for_checks_and_pins_by_rook(x, y);
        let checks_by_knight: ChecksAndPins = self.look_for_checks_by_knight(x, y);
        let checks_and_pins_by_bishop: ChecksAndPins =
            self.look_for_checks_and_pins_by_bishop(x, y);

        let mut destinations: HashSet<(usize, usize)> = HashSet::new();
        destinations.extend(checks_and_pins_by_rook.destinations.iter());
        destinations.extend(checks_and_pins_by_bishop.destinations.iter());
        destinations.extend(checks_by_knight.destinations.iter());

        let is_checked: bool = checks_and_pins_by_rook.is_checked
            || checks_by_knight.is_checked
            || checks_and_pins_by_bishop.is_checked;

        let mut current_positions: HashSet<(usize, usize)> = HashSet::new();
        current_positions.extend(checks_and_pins_by_rook.current_positions.iter());
        current_positions.extend(checks_and_pins_by_bishop.current_positions.iter());

        let is_pinned: bool =
            checks_and_pins_by_rook.is_pinned || checks_and_pins_by_bishop.is_pinned;

        ChecksAndPins {
            destinations,
            is_checked,
            current_positions,
            is_pinned,
        }
    }

    pub fn compute_possible_moves(&mut self) {
        let mut next_possible_moves: HashSet<PieceMove> = HashSet::new();
        for (i, line) in self.board.into_iter().enumerate() {
            for (j, piece) in line.into_iter().enumerate() {
                if piece.is_none() || piece.unwrap().color != self.current_player {
                    continue;
                }
                match piece.unwrap().name {
                    Rook => next_possible_moves
                        .extend(&mut piece.unwrap().rook(self.board, i as i32, j as i32).iter()),
                    Knight => next_possible_moves
                        .extend(&mut piece.unwrap().knight(self.board, i as i32, j as i32).iter()),
                    Bishop => next_possible_moves
                        .extend(&mut piece.unwrap().bishop(self.board, i as i32, j as i32).iter()),
                    Queen => next_possible_moves
                        .extend(&mut piece.unwrap().queen(self.board, i as i32, j as i32).iter()),
                    King => next_possible_moves
                        .extend(&mut piece.unwrap().king(self, i as i32, j as i32).iter()),
                    Pawn => next_possible_moves
                        .extend(&mut piece.unwrap().pawn(self.board, i as i32, j as i32).iter()),
                }
            }
        }
        self.possible_moves = next_possible_moves;
    }

    pub fn compute_legal_moves(&mut self) {
        let king_position: (usize, usize) = if self.current_player == White {
            self.white_king_position
        } else {
            self.black_king_position
        };
        let _ = &mut self.compute_possible_moves();
        let checks_and_pins: ChecksAndPins = self.look_for_checks_and_pins(
            king_position.0.try_into().unwrap(),
            king_position.1.try_into().unwrap(),
        );
        let mut legal_moves: HashSet<PieceMove> = HashSet::new();
        if checks_and_pins.is_checked {
            for destination in checks_and_pins.destinations {
                legal_moves.extend(
                    (&self.possible_moves)
                        .into_iter()
                        .filter(|piece_move| piece_move.destination == destination),
                );
            }
        }
        if checks_and_pins.is_pinned && checks_and_pins.is_checked {
            for current_position in checks_and_pins.current_positions {
                legal_moves.retain(|piece_move| piece_move.current_position != current_position);
            }
        } else if checks_and_pins.is_pinned && !checks_and_pins.is_checked {
            for current_position in checks_and_pins.current_positions {
                legal_moves.extend(
                    (&self.possible_moves)
                        .into_iter()
                        .filter(|piece_move| piece_move.current_position != current_position),
                );
            }
        }
        self.possible_moves = legal_moves;
    }

    pub fn display_possible_moves(&self) {
        let mut output: String = String::new();
        for piece_move in self.possible_moves.iter() {
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::board::piece::checksandpins::ChecksAndPins;

    use super::Board;

    #[test]
    fn computes_correct_moves_bishop() {
        let mut board: Board = Board::new();
        board.execute_move((1, 3), (2, 3));
        board.execute_move((6, 4), (5, 4));
        board.execute_move((7, 5), (3, 1));
        let result = board.look_for_checks_and_pins_by_bishop(
            board.white_king_position.0.try_into().unwrap(),
            board.white_king_position.1.try_into().unwrap(),
        );
        assert!(
            ChecksAndPins {
                destinations: HashSet::from([(2, 2), (1, 3)]),
                is_checked: true,
                current_positions: HashSet::new(),
                is_pinned: false
            } == result
        );
    }
}
