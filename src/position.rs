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

#[derive(Debug, PartialEq)]
pub struct Position(pub Row, pub Column);

impl str::FromStr for Position {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 2 {
            match (s[0..1].parse::<Row>(), s[1..2].parse::<Column>()) {
                (Ok(row), Ok(column)) => Ok(Position(row, column)),
                (Err(_), Ok(_)) => Err(Error::RowPositionError(s.to_owned())),
                (Ok(_), Err(_)) => Err(Error::ColumnPositionError(s.to_owned())),
                (Err(_), Err(_)) => Err(Error::RowAndColumnPositionError(s.to_owned())),
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
    fn parse_position_errors() {
        let mut position = "D1";
        assert_eq!(
            position.parse::<Position>(),
            Err(Error::RowPositionError(position.to_owned()))
        );

        position = "A4";
        assert_eq!(
            position.parse::<Position>(),
            Err(Error::ColumnPositionError(position.to_owned()))
        );

        position = "D9";
        assert_eq!(
            position.parse::<Position>(),
            Err(Error::RowAndColumnPositionError(position.to_owned()))
        );

        position = "ABC";
        assert_eq!(
            position.parse::<Position>(),
            Err(Error::InvalidPositionStringLength(position.to_owned()))
        );
    }
}
