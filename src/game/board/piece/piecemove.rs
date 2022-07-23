use super::piece::Piece;
pub struct PieceMove {
    pub piece: Piece,
    pub destination: (usize, usize),
    pub takes: bool,
    pub puts_in_check: bool,
}
