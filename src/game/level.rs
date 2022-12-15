//! Chess Piece Level

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Chess Piece Level
///
/// Consist of Pawn, Rook, Knight, Bishop, Queen, and King
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Level {
    #[default]
    Pawn,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Level::Pawn => write!(f, "Pawn"),
            Level::Rook => write!(f, "Rook"),
            Level::Knight => write!(f, "Knight"),
            Level::Bishop => write!(f, "Bishop"),
            Level::Queen => write!(f, "Queen"),
            Level::King => write!(f, "King")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_level() {
        assert_eq!(Level::Pawn.to_string(), "Pawn");
        assert_eq!(Level::Rook.to_string(), "Rook");
        assert_eq!(Level::Knight.to_string(), "Knight");
        assert_eq!(Level::Bishop.to_string(), "Bishop");
        assert_eq!(Level::Queen.to_string(), "Queen");
        assert_eq!(Level::King.to_string(), "King");
    }
}
