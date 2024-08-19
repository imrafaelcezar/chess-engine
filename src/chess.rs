pub mod utils;

pub struct Chess {
    pub board: [Piece; 64],
    pub side_to_move: Side,
}

impl Chess {
    pub fn new(board: [Piece; 64], side_to_move: Side) -> Chess {
        Chess {
            board,
            side_to_move,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
    None,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub side: Side,
    pub kind: PieceKind,
}

impl Piece {
    pub fn new(side: Side, kind: PieceKind) -> Piece {
        Piece { side, kind }
    }

    pub fn is_side(&self, side: Side) -> bool {
        return self.side == side;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Empty,
}
