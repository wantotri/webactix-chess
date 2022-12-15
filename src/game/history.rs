//! Moves History

use std::fmt::Display;
use super::Piece;
use serde::{Deserialize, Serialize};

/// Moves History
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct History {
    pub from: String,
    pub to: String,
    pub captured: Option<Piece>,
    pub has_moved: Option<bool>
}

impl Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.captured {
            Some(ref piece) => write!(f, "moves {} to {}, {}  captured", self.from, self.to, piece),
            None => write!(f, "moves {} to {}", self.from, self.to)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::prelude::{Level, Color};
    use super::*;

    #[test]
    fn print_history() {
        let mut his = History {
            from: "a2".to_string(),
            to: "a4".to_string(),
            captured: None,
            has_moved: Some(true)
        };
        assert_eq!(his.to_string(), "moves a2 to a4");

        let piece = Piece::new(Level::Pawn, Color::Black);
        his.captured = Some(piece);
        assert_eq!(his.to_string(), "moves a2 to a4, ♟  captured");
    }
}