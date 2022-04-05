#![deny(clippy::all)]
#![warn(clippy::pedantic)]
use tic_tac_toe::{Board, Result, Square};

fn main() -> Result<()> {
    let mut board = Board::default();

    board.set_square(&"B2".parse()?, Square::X);
    board.set_square(&"A1".parse()?, Square::O);
    board.set_square(&"C3".parse()?, Square::X);
    board.set_square(&"C1".parse()?, Square::O);

    // Throw an Error!!! 😱 😱 😱
    board.set_square(&"ABC".parse()?, Square::O);

    println!("{board}");

    Ok(())
}
