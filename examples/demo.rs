use chessboard_slint::{get_piece_from_fen, is_white_turn_in_fen};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new().unwrap();
    app.global::<FenUtils>().on_get_piece_from_fen(|fen, cell_index| {
        get_piece_from_fen(fen, cell_index)
    });
    app.global::<FenUtils>().on_is_white_turn_in_fen(|fen| {
        is_white_turn_in_fen(fen)
    });
    app.run()
}
