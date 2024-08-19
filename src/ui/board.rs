use crate::chess::{Piece, PieceKind, Side};

pub fn show_board(board: &[Piece; 64]) {
    for i in 0..8 {
        for j in 0..8 {
            let piece_index = j + i * 8;
            let piece = board[piece_index];
            let square_background = get_square_background_by_index(i, j);
            print!("{} \x1b[1;32m{} \x1b[0m", square_background, get_char_by_piece(&piece));
        }

        println!("");
    }
}

fn get_square_background_by_index(x: usize, y: usize) -> String {
    if (x + y) % 2 == 0 {
        return String::from("\x1b[47m");
    }

    return String::from("\x1b[40m");
}

fn get_char_by_piece(piece: &Piece) -> String {
    let mut piece_char = match piece.kind {
        PieceKind::Pawn => 'p',
        PieceKind::Knight => 'n',
        PieceKind::Bishop => 'b',
        PieceKind::Rook=> 'r',
        PieceKind::Queen => 'q',
        PieceKind::King => 'k',
        PieceKind::Empty => ' ',
    }.to_string();

    if piece.is_side(Side::White) {
        piece_char = piece_char.to_uppercase().to_string();
    }

    piece_char
}
