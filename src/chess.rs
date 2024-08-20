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

    pub fn mov(&mut self, mov: (usize, usize)) {
        let (start, target) = mov;
        let start_piece = self.board[start];

        if start == target || !start_piece.is_side(self.side_to_move) || start_piece.is_empty() {
            println!("Invalid move.");
            return;
        }

        self.board[target] = start_piece;
        self.board[start] = Piece::new(Side::None, PieceKind::Empty);

        self.side_to_move = if let Side::White = self.side_to_move {
            Side::Black
        } else {
            Side::White
        };
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

    pub fn is_empty(&self) -> bool {
        return self.side == Side::None || self.kind == PieceKind::Empty;
    }

    pub fn is_side(&self, side: Side) -> bool {
        return self.side == side;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Empty,
}
