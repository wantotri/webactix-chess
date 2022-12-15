//! Chess Piece Color

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Chess Piece Color
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::Black => write!(f, "Black"),
            Color::White => write!(f, "White")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_color() {
        assert_eq!(Color::White.to_string(), "White");
        assert_eq!(Color::Black.to_string(), "Black");
    }
}