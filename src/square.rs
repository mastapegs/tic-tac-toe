use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Square {
    Empty,
    X,
    O,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Square::Empty => write!(f, "_"),
            Square::X => write!(f, "X"),
            Square::O => write!(f, "O"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn display_trait_test() {
        assert_eq!("X", format!("{}", Square::X));
        assert_eq!("O", format!("{}", Square::O));
        assert_eq!("_", format!("{}", Square::Empty));
    }
}
