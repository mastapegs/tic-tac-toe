use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    RowPositionError(String),
    ColumnPositionError(String),
    CombinedPositionError(String),
    InvalidPositionStringLength(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RowPositionError(character) => {
                write!(f, "Invalid Row Char: {}", character)
            }
            Error::ColumnPositionError(character) => {
                write!(f, "Invalid Column Char: {}", character)
            }
            Error::CombinedPositionError(character) => {
                write!(f, "Invalid Characters: {}", character)
            }
            Error::InvalidPositionStringLength(character) => {
                write!(f, "Need exactly 2 characters, received: {}", character)
            }
        }
    }
}

impl std::error::Error for Error {}
