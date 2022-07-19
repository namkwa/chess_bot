use super::{piececolor::PieceColor, piecename::PieceName};

#[derive(Copy, Clone)]
pub struct Piece {
    pub name: PieceName,
    pub color: PieceColor,
}

impl Piece {
    pub fn new(name: PieceName, color: PieceColor) -> Self {
        Piece { name, color }
    }
}
