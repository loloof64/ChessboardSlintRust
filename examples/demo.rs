slint::include_modules!();

extern crate pleco;
use chessboard_slint::{get_piece_from_fen, is_white_turn_in_fen};
use pleco::Board;

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new().unwrap();
    let chess_logic = Board::start_pos();

    // Settings callbacks for the chess board
    app.global::<FenUtils>().on_get_piece_from_fen(|fen, cell_index| {
        get_piece_from_fen(fen, cell_index)
    });
    app.global::<FenUtils>().on_is_white_turn_in_fen(|fen| {
        is_white_turn_in_fen(fen)
    });

    // Settings callbacks for the current position
    app.on_get_current_position(move || {
        chess_logic.fen().into()
    });

    // Runs application
    app.run()
}
