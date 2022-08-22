use super::piececolor::PieceColor;
use super::piececolor::PieceColor::Black;
use super::piececolor::PieceColor::White;
use super::side::Side;
use super::side::Side::Both;
use super::side::Side::King;
use super::side::Side::Queen;

pub struct CastlingRights {
    pub can_white_castle_queen_side: bool,
    pub can_white_castle_king_side: bool,
    pub can_black_castle_queen_side: bool,
    pub can_black_castle_king_side: bool,
}

impl CastlingRights {
    pub fn new() -> Self {
        CastlingRights {
            can_white_castle_queen_side: true,
            can_white_castle_king_side: true,
            can_black_castle_queen_side: true,
            can_black_castle_king_side: true,
        }
    }

    pub fn remove_castling_rights(&mut self, piece_color: PieceColor, side: Side) {
        match (piece_color, side) {
            (White, Both) => {
                self.can_white_castle_king_side = false;
                self.can_white_castle_queen_side = false;
            }
            (Black, Both) => {
                self.can_black_castle_king_side = false;
                self.can_black_castle_queen_side = false;
            }
            (Black, Queen) => self.can_black_castle_queen_side = false,
            (White, Queen) => self.can_white_castle_queen_side = false,
            (Black, King) => self.can_black_castle_king_side = false,
            (White, King) => self.can_white_castle_king_side = false,
        }
    }
}
