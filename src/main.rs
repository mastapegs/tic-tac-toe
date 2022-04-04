#![deny(clippy::all)]
#![warn(clippy::pedantic)]
use tic_tac_toe::{Board, Row, Square};

fn main() {
    println!(
        "{}",
        Board {
            row1: Row(Square::Empty, Square::Empty, Square::X),
            row2: Row(Square::Empty, Square::X, Square::O),
            row3: Row(Square::O, Square::Empty, Square::Empty),
        }
    );
}
