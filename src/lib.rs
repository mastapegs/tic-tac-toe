#![deny(clippy::all)]
#![warn(clippy::pedantic)]
mod board;
mod error;
pub mod position;
mod square;

pub use board::{Board, Row};
pub use error::Error;
pub use position::Position;
pub use square::Square;

pub type Result<T> = std::result::Result<T, error::Error>;
