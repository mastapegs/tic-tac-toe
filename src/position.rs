use crate::{Error, Result};
use std::str;

#[derive(Debug, PartialEq)]
pub enum Row {
    A,
    B,
    C,
}

impl str::FromStr for Row {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match &*s.to_ascii_uppercase() {
            "A" => Ok(Row::A),
            "B" => Ok(Row::B),
            "C" => Ok(Row::C),
            _ => Err(Error::RowPositionError(s.to_owned())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Column {
    One,
    Two,
    Three,
}

impl str::FromStr for Column {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "1" => Ok(Column::One),
            "2" => Ok(Column::Two),
            "3" => Ok(Column::Three),
            _ => Err(Error::ColumnPositionError(s.to_owned())),
        }
    }
}

/// A1 | A2 | A3
/// B1 | B2 | B3
/// C1 | C2 | C3
#[derive(Debug, PartialEq)]
pub struct Position(pub Row, pub Column);

impl str::FromStr for Position {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 2 {
            let characters = s.split("").collect::<Vec<&str>>();
            let row = characters[1].parse::<Row>();
            let column = characters[2].parse::<Column>();

            match (row, column) {
                (Ok(row), Ok(column)) => Ok(Position(row, column)),
                _ => Err(Error::CombinedPositionError(s.to_owned())),
            }
        } else {
            Err(Error::InvalidPositionStringLength(s.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_row() {
        assert_eq!("A".parse(), Ok(Row::A));
        assert_eq!("B".parse(), Ok(Row::B));
        assert_eq!("c".parse(), Ok(Row::C));
    }

    #[test]
    fn parse_row_error() {
        fn assert_row_error(char: &str) {
            assert_eq!(
                char.parse::<Row>(),
                Err(Error::RowPositionError(char.to_owned()))
            );
        }

        assert_row_error("D");
        assert_row_error("E");
        assert_row_error("z");
        assert_row_error("1");
    }

    #[test]
    fn parse_column() {
        assert_eq!("1".parse(), Ok(Column::One));
        assert_eq!("2".parse(), Ok(Column::Two));
        assert_eq!("3".parse(), Ok(Column::Three));
    }

    #[test]
    fn parse_column_error() {
        fn assert_column_error(char: &str) {
            assert_eq!(
                char.parse::<Column>(),
                Err(Error::ColumnPositionError(char.to_owned()))
            );
        }

        assert_column_error("4");
        assert_column_error("5");
        assert_column_error("6");
        assert_column_error("z");
        assert_column_error("A");
        assert_column_error("5");
    }

    #[test]
    fn parse_position() {
        assert_eq!("A1".parse(), Ok(Position(Row::A, Column::One)));
        assert_eq!("B3".parse(), Ok(Position(Row::B, Column::Three)));
        assert_eq!("C2".parse(), Ok(Position(Row::C, Column::Two)));
    }

    #[test]
    fn parse_position_error() {
        fn assert_position_combination_error(char: &str) {
            assert_eq!(
                char.parse::<Position>(),
                Err(Error::CombinedPositionError(char.to_owned()))
            );
        }

        fn assert_position_length_error(char: &str) {
            assert_eq!(
                char.parse::<Position>(),
                Err(Error::InvalidPositionStringLength(char.to_owned()))
            );
        }

        assert_position_combination_error("D1");
        assert_position_combination_error("A4");
        assert_position_combination_error("B9");

        assert_position_length_error("ABC");
        assert_position_length_error("123");
        assert_position_length_error("Helicopter");
    }
}
