use super::piece::Piece;
pub struct PieceMove {
    name_piece: Piece,
    destination: (i32, i32),
    takes: bool,
}
