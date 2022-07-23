use core::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceName {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

impl fmt::Display for PieceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieceName::Rook => write!(f, "Rook"),
            PieceName::Knight => write!(f, "Knight"),
            PieceName::Bishop => write!(f, "Bishop"),
            PieceName::Queen => write!(f, "Queen"),
            PieceName::King => write!(f, "King"),
            PieceName::Pawn => write!(f, "Pawn"),
        }
    }
}
