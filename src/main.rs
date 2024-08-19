mod chess;
mod ui;

use chess::utils::load_position_from_fen;
use ui::board::show_board;

fn main() {
    let chess = load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w");
    show_board(&chess.board);
}
