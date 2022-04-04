#![deny(clippy::all)]
#![warn(clippy::pedantic)]
use tic_tac_toe::Square;

fn main() {
    println!("{}", Square::X);
    println!("{}", Square::O);
    println!("{}", Square::Empty);
}
