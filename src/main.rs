mod chess;
mod chess_utils;

fn main() {
    let chess =
        chess_utils::load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w");
    println!("{:?}", chess.board);
}
