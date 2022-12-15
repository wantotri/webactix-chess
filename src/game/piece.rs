//! Chess Piece

use std::fmt::Display;
use serde::{Deserialize, Serialize};

use super::{Color, Level};

/// Chess Piece
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Piece {
    pub level: Level,
    pub color: Color,
    pub icon: String,
    pub mv_unit: Option<i8>,
    pub moved: Option<bool>
}

impl Piece {
    /// Create new Piece instance
    pub fn new(level: Level, color: Color) -> Self {
        let icon = match color {
            Color::Black => match level {
                Level::Pawn => "♟",
                Level::Rook => "♜",
                Level::Bishop => "♝",
                Level::Knight => "♞",
                Level::Queen => "♛",
                Level::King => "♚",
            },
            Color::White => match level {
                Level::Pawn => "♙",
                Level::Rook => "♖",
                Level::Bishop => "♗",
                Level::Knight => "♘",
                Level::Queen => "♕",
                Level::King => "♔",
            },
        }.to_string();

        let (moved, mv_unit) = match level {
            Level::Pawn => match color {
                Color::Black => (Some(false), Some(-1)),
                Color::White => (Some(false), Some(1)),
            },
            Level::Rook | Level::King => (Some(false), None),
            _ => (None, None)
        };

        Self { level, color, icon, mv_unit, moved }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.icon)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn piece_default() {
        let p = Piece::default();
        assert_eq!(p.level, Level::Pawn);
        assert_eq!(p.color, Color::White);
        assert_eq!(p.icon, "");
        assert_eq!(p.moved, None);
        assert_eq!(p.mv_unit, None);
    }

    #[test]
    fn piece_new_pawn() {
        let p = Piece::new(Level::Pawn, Color::White);
        assert_eq!(p.level, Level::Pawn);
        assert_eq!(p.color, Color::White);
        assert_eq!(p.icon, "♙");
        assert_eq!(p.moved, Some(false));
        assert_eq!(p.mv_unit, Some(1));
    }

    #[test]
    fn piece_new_rook() {
        let p = Piece::new(Level::Rook, Color::White);
        assert_eq!(p.level, Level::Rook);
        assert_eq!(p.color, Color::White);
        assert_eq!(p.icon, "♖");
        assert_eq!(p.moved, Some(false));
        assert_eq!(p.mv_unit, None);
    }

    #[test]
    fn piece_new_bishop() {
        let p = Piece::new(Level::Bishop, Color::White);
        assert_eq!(p.level, Level::Bishop);
        assert_eq!(p.color, Color::White);
        assert_eq!(p.icon, "♗");
        assert_eq!(p.moved, None);
        assert_eq!(p.mv_unit, None);
    }
}