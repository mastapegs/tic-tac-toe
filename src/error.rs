use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    RowPositionError(String),
    ColumnPositionError(String),
    RowAndColumnPositionError(String),
    InvalidPositionStringLength(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RowPositionError(character) => {
                write!(f, "Invalid Row Char. Received: {}", character)
            }
            Error::ColumnPositionError(character) => {
                write!(f, "Invalid Column Char. Recieved: {}", character)
            }
            Error::RowAndColumnPositionError(character) => {
                write!(
                    f,
                    "Both Invalid Row and Invalid Column characters. Received: {}",
                    character
                )
            }
            Error::InvalidPositionStringLength(character) => {
                write!(
                    f,
                    "Needs exactly 2 valid position characters, Received: {}",
                    character
                )
            }
        }
    }
}

impl std::error::Error for Error {}
