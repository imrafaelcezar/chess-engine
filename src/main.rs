mod chess;
mod ui;

use chess::utils;

fn main() {
    let mut chess = utils::load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w");

    loop {
        ui::clear();
        ui::show_board(&chess.board);
        ui::show_side_to_move(&chess.side_to_move);
        let mov = ui::ask_for_move();
        let mov = utils::get_move_by_semi_algebraic_notation(mov);
        chess.mov(mov);
    }
}
