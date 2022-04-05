use crate::{position, Position, Square};
use std::fmt;

#[derive(Debug, PartialEq, Default)]
pub struct Row(pub Square, pub Square, pub Square);

impl fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} | {} | {}", self.0, self.1, self.2)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Board {
    pub row1: Row,
    pub row2: Row,
    pub row3: Row,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}\n{}\n{}", self.row1, self.row2, self.row3)
    }
}

impl Board {
    fn get_square(&self, position: &Position) -> &Square {
        match position {
            Position(position::Row::A, position::Column::One) => &self.row1.0,
            Position(position::Row::A, position::Column::Two) => &self.row1.1,
            Position(position::Row::A, position::Column::Three) => &self.row1.2,

            Position(position::Row::B, position::Column::One) => &self.row2.0,
            Position(position::Row::B, position::Column::Two) => &self.row2.1,
            Position(position::Row::B, position::Column::Three) => &self.row2.2,

            Position(position::Row::C, position::Column::One) => &self.row3.0,
            Position(position::Row::C, position::Column::Two) => &self.row3.1,
            Position(position::Row::C, position::Column::Three) => &self.row3.2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn row_display_trait() {
        assert_eq!("_ | _ | _", format!("{}", Row::default()));
        assert_eq!(
            "X | X | X",
            format!("{}", Row(Square::X, Square::X, Square::X))
        );
        assert_eq!(
            "_ | O | X",
            format!("{}", Row(Square::Empty, Square::O, Square::X))
        );
    }

    #[test]
    fn board_display_trait() {
        assert_eq!(
            "\
            _ | _ | _\n\
            _ | _ | _\n\
            _ | _ | _\
        ",
            format!("{}", Board::default())
        );

        assert_eq!(
            "\
            _ | _ | _\n\
            _ | X | _\n\
            _ | _ | _\
        ",
            format!(
                "{}",
                Board {
                    row1: Row::default(),
                    row2: Row(Square::Empty, Square::X, Square::Empty),
                    row3: Row::default(),
                }
            )
        );

        assert_eq!(
            "\
            _ | _ | X\n\
            _ | X | O\n\
            O | _ | _\
        ",
            format!(
                "{}",
                Board {
                    row1: Row(Square::Empty, Square::Empty, Square::X),
                    row2: Row(Square::Empty, Square::X, Square::O),
                    row3: Row(Square::O, Square::Empty, Square::Empty),
                }
            )
        );
    }

    #[test]
    fn test_get_square() {
        let board = Board {
            row1: Row(Square::Empty, Square::Empty, Square::X),
            row2: Row(Square::Empty, Square::X, Square::O),
            row3: Row(Square::O, Square::Empty, Square::Empty),
        };

        assert_eq!(
            board.get_square(&"A1".parse::<Position>().unwrap()),
            &Square::Empty
        );

        assert_eq!(
            board.get_square(&"B3".parse::<Position>().unwrap()),
            &Square::O
        );

        assert_eq!(
            board.get_square(&"B2".parse::<Position>().unwrap()),
            &Square::X
        );

        assert_eq!(
            board.get_square(&"C3".parse::<Position>().unwrap()),
            &Square::Empty
        );
    }
}
