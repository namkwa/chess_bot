use super::piece::Piece;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PieceMove {
    pub piece: Piece,
    pub destination: (usize, usize),
    pub current_position: (usize, usize),
    pub takes: bool,
    pub puts_in_check: bool,
}
