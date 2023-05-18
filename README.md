# Chessboard Slint

A chessboard component for Slint Ui Toolkit.

# Usage

## Warning

You must not forget to set the callbacks `get_piece_from_fen` and `is_white_turn_in_fen` in the chessboard component.

Don't worry, there's already an implementation ready for those two callbacks in this crate. 

You can have a look at the **ui/demo.slint** and the **examples/demo.rs** files for an example of how to proceed.

## Demo

You can run the demo with `cargo run --example demo`.

## CREDITS

Chess vectors have been taken from [Wikimedia Commons](https://commons.wikimedia.org/wiki/Category:SVG_chess_pieces).
