pub mod piececolor;
#[derive(Copy, Clone)]
pub struct Piece {
    pub name: char,
    pub color: piececolor::PieceColor,
}
