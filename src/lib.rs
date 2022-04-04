#![deny(clippy::all)]
#![warn(clippy::pedantic)]
mod board;
mod square;

pub use board::{Board, Row};
pub use square::Square;
