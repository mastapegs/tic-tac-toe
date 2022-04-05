use crate::{position, Position, Square};
use std::{fmt, result};

#[derive(Debug, PartialEq, Default, Copy, Clone)]
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        write!(f, "{}\n{}\n{}", self.row1, self.row2, self.row3)
    }
}

impl Board {
    #[allow(dead_code)]
    fn get_square(&self, position: &Position) -> Square {
        match position {
            Position(position::Row::A, position::Column::One) => self.row1.0,
            Position(position::Row::A, position::Column::Two) => self.row1.1,
            Position(position::Row::A, position::Column::Three) => self.row1.2,

            Position(position::Row::B, position::Column::One) => self.row2.0,
            Position(position::Row::B, position::Column::Two) => self.row2.1,
            Position(position::Row::B, position::Column::Three) => self.row2.2,

            Position(position::Row::C, position::Column::One) => self.row3.0,
            Position(position::Row::C, position::Column::Two) => self.row3.1,
            Position(position::Row::C, position::Column::Three) => self.row3.2,
        }
    }

    pub fn set_square(&mut self, position: &Position, square: Square) {
        *self = match position {
            Position(position::Row::A, position::Column::One) => Self {
                row1: Row {
                    0: square,
                    ..self.row1
                },
                ..*self
            },
            Position(position::Row::A, position::Column::Two) => Self {
                row1: Row {
                    1: square,
                    ..self.row1
                },
                ..*self
            },
            Position(position::Row::A, position::Column::Three) => Self {
                row1: Row {
                    2: square,
                    ..self.row1
                },
                ..*self
            },

            Position(position::Row::B, position::Column::One) => Self {
                row2: Row {
                    0: square,
                    ..self.row2
                },
                ..*self
            },
            Position(position::Row::B, position::Column::Two) => Self {
                row2: Row {
                    1: square,
                    ..self.row2
                },
                ..*self
            },
            Position(position::Row::B, position::Column::Three) => Self {
                row2: Row {
                    2: square,
                    ..self.row2
                },
                ..*self
            },

            Position(position::Row::C, position::Column::One) => Self {
                row3: Row {
                    0: square,
                    ..self.row3
                },
                ..*self
            },
            Position(position::Row::C, position::Column::Two) => Self {
                row3: Row {
                    1: square,
                    ..self.row3
                },
                ..*self
            },
            Position(position::Row::C, position::Column::Three) => Self {
                row3: Row {
                    2: square,
                    ..self.row3
                },
                ..*self
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Result;

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
    fn test_get_square() -> Result<()> {
        let board = Board {
            row1: Row(Square::Empty, Square::Empty, Square::X),
            row2: Row(Square::Empty, Square::X, Square::O),
            row3: Row(Square::O, Square::Empty, Square::Empty),
        };

        assert_eq!(board.get_square(&"A1".parse()?), Square::Empty);
        assert_eq!(board.get_square(&"B3".parse()?), Square::O);
        assert_eq!(board.get_square(&"B2".parse()?), Square::X);
        assert_eq!(board.get_square(&"C3".parse()?), Square::Empty);

        Ok(())
    }

    #[test]
    fn test_set_square() -> Result<()> {
        let mut test_board = Board::default();

        test_board.set_square(&"B2".parse()?, Square::X);
        assert_eq!(
            test_board,
            Board {
                row1: Row::default(),
                row2: Row(Square::Empty, Square::X, Square::Empty),
                row3: Row::default(),
            }
        );

        test_board.set_square(&"C1".parse()?, Square::O);
        assert_eq!(
            test_board,
            Board {
                row1: Row::default(),
                row2: Row(Square::Empty, Square::X, Square::Empty),
                row3: Row(Square::O, Square::Empty, Square::Empty),
            }
        );

        test_board.set_square(&"A3".parse()?, Square::X);
        assert_eq!(
            test_board,
            Board {
                row1: Row(Square::Empty, Square::Empty, Square::X),
                row2: Row(Square::Empty, Square::X, Square::Empty),
                row3: Row(Square::O, Square::Empty, Square::Empty),
            }
        );

        Ok(())
    }
}
