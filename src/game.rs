//! Chess Game
//!
pub mod vectors;
pub mod color;
pub mod level;
pub mod piece;
pub mod history;
pub mod board;

pub mod prelude {
    pub use crate::game::vectors::Movement;
    pub use crate::game::color::Color;
    pub use crate::game::level::Level;
    pub use crate::game::piece::Piece;
    pub use crate::game::history::History;
    pub use crate::game::board::Board;
    pub use crate::error::Error;
}

use prelude::*;
use crate::error::Error::{self, *};

/// Convert chess notation into tuple (row, col)
///
/// ### Example
///
/// ```
/// use chess::game::convert;
/// assert_eq!(convert("a1")?, (0, 0));
/// # Ok::<(), chess::error::Error>(())
/// ```
pub fn convert(cell: &str) -> Result<(i8, i8), Error> {
    let col = match &cell[0..1] {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        "g" => 6,
        "h" => 7,
        _ => return Err(InvalidNotation("use proper notation, examples: 'a1' 'b2' 'h8'".to_owned())),
    };

    match (&cell[1..2]).parse::<i8>() {
        Ok(n) if n <= 8 && n > 0 => return Ok((n - 1, col)),
        _ => return Err(InvalidNotation("use proper notation, examples: 'a1' 'b2' 'h8'".to_owned()))
    };
}

/// Convert tuple (row, col) into chess notation
///
/// ### Examples
///
/// ```
/// use chess::game::invert;
/// assert_eq!(invert(0, 0)?, "a1");
/// # Ok::<(), chess::error::Error>(())
/// ```
pub fn invert(row: i8, col: i8) -> Result<String, Error> {
    let col = match col {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => return Err(InvalidNotation("col must be one of (01234567)".to_owned())),
    };

    if row > 7 || row < 0 {
        return Err(InvalidNotation("row must be one of (01234567)".to_owned()));
    }

    Ok(format!("{}{}", col, row + 1))
}

/// Get enemy color given color
///
/// ### Examples
///
/// ```
/// use chess::game::get_enemy_color;
/// use chess::game::prelude::*;
/// assert_eq!(get_enemy_color(Color::White), Color::Black);
/// ```
pub fn get_enemy_color(color: Color) -> Color {
    match color {
        Color::Black => Color::White,
        Color::White => Color::Black
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converting() -> Result<(), Error> {
        assert_eq!(convert("a1")?, (0, 0));
        assert_eq!(convert("h8")?, (7, 7));
        Ok(())
    }

    #[test]
    fn inverting() -> Result<(), Error> {
        assert_eq!(invert(0, 0)?, "a1");
        assert_eq!(invert(7, 7)?, "h8");
        Ok(())
    }

    #[test]
    fn enemy_color() {
        assert_eq!(get_enemy_color(Color::White), Color::Black);
        assert_eq!(get_enemy_color(Color::Black), Color::White);
    }
}
