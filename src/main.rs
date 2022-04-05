#![deny(clippy::all)]
#![warn(clippy::pedantic)]
use tic_tac_toe::{Board, Result, Square};

fn main() -> Result<()> {
    let mut board = Board::default();

    let moves = [
        ("B2", Square::X),
        ("A1", Square::O),
        ("C3", Square::X),
        ("C1", Square::O),
        ("B1", Square::X),
        ("B3", Square::O),
    ];

    for (position, square) in moves {
        board.set_square(&position.parse()?, square);
    }

    println!("{board}");

    Ok(())
}
