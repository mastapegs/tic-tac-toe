use std::fmt;

use crate::{Position, Square};

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
    fn get_square(&self, position: &Position) -> Square {
        todo!()
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
}
