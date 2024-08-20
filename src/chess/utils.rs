use std::ops::{Add, Sub};

use super::{Chess, Piece, PieceKind, Side};

pub fn load_position_from_fen(fen: &str) -> Chess {
    let mut fen = fen.split(" ");
    let position_fen = fen.next().unwrap();
    let side_to_move_fen = fen.next().unwrap();

    let side_to_move = if side_to_move_fen == "w" {
        Side::White
    } else {
        Side::Black
    };

    let mut board: [Piece; 64] = [Piece::new(Side::None, PieceKind::Empty); 64];
    let mut aux: usize = 0;

    for c in position_fen.chars() {
        match c.to_string().parse::<usize>() {
            Ok(num) => {
                aux += num;
            }
            Err(..) => {
                board[aux] = get_piece_by_char(c);

                if c != '/' {
                    aux += 1
                };
            }
        };
    }

    Chess::new(board, side_to_move)
}

fn get_piece_by_char(c: char) -> Piece {
    match c {
        'P' => Piece::new(Side::White, PieceKind::Pawn),
        'N' => Piece::new(Side::White, PieceKind::Knight),
        'B' => Piece::new(Side::White, PieceKind::Bishop),
        'R' => Piece::new(Side::White, PieceKind::Rook),
        'Q' => Piece::new(Side::White, PieceKind::Queen),
        'K' => Piece::new(Side::White, PieceKind::King),
        'p' => Piece::new(Side::Black, PieceKind::Pawn),
        'n' => Piece::new(Side::Black, PieceKind::Knight),
        'b' => Piece::new(Side::Black, PieceKind::Bishop),
        'r' => Piece::new(Side::Black, PieceKind::Rook),
        'q' => Piece::new(Side::Black, PieceKind::Queen),
        'k' => Piece::new(Side::Black, PieceKind::King),
        _ => Piece::new(Side::None, PieceKind::Empty),
    }
}

pub fn get_move_by_semi_algebraic_notation(algebraic_notation: String) -> (usize, usize) {
    // e2e4
    let start = &algebraic_notation[..2]; // e2
    let target = &algebraic_notation[2..]; // e4

    (
        get_square_by_semi_algebraic_notation(start),
        get_square_by_semi_algebraic_notation(target),
    )
}

fn get_square_by_semi_algebraic_notation(algebraic_notation: &str) -> usize {
    let letter = &algebraic_notation[..1];
    let number = &algebraic_notation[1..];
    println!("{number}");
    let number: usize = number.parse().expect("Invalid algebraic notation");

    match letter {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        _ => 0,
    }
    .add((8 - number) * 8)
    .sub(1)
}
