//! Chess Board

use std::{fmt::Display, io::{self, Write}};
use std::collections::HashMap;
use super::{
    Movement,
    Color,
    Level,
    Piece,
    History,
    convert,
    invert,
    get_enemy_color
};
use crate::error::Error::{self, *};
use serde::{Deserialize, Serialize};

/// Chess Board
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Board<T> {
    pub cells: Vec<Vec<Option<T>>>,
    pub history: Vec<History>
}

impl<T: Display> Display for Board<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (_i, row) in self.cells.iter().enumerate().rev() {
            for cell in row.iter() {
                if let Some(piece) = cell {
                    write!(f, "{} ", piece)?;
                } else {
                    write!(f, "⬛ ")?;
                };
            };
            write!(f, "\n")?;
        };
        Ok(())
    }
}

impl Board<Piece> {
    /// Create a new chess board already filled with pieces
    pub fn new() -> Self {
        Self {
            cells: vec![
                vec![
                    Some(Piece::new(Level::Rook, Color::White)),
                    Some(Piece::new(Level::Knight, Color::White)),
                    Some(Piece::new(Level::Bishop, Color::White)),
                    Some(Piece::new(Level::Queen, Color::White)),
                    Some(Piece::new(Level::King, Color::White)),
                    Some(Piece::new(Level::Bishop, Color::White)),
                    Some(Piece::new(Level::Knight, Color::White)),
                    Some(Piece::new(Level::Rook, Color::White)),
                ],
                vec![Some(Piece::new(Level::Pawn, Color::White)); 8],
                vec![None; 8],
                vec![None; 8],
                vec![None; 8],
                vec![None; 8],
                vec![Some(Piece::new(Level::Pawn, Color::Black)); 8],
                vec![
                    Some(Piece::new(Level::Rook, Color::Black)),
                    Some(Piece::new(Level::Knight, Color::Black)),
                    Some(Piece::new(Level::Bishop, Color::Black)),
                    Some(Piece::new(Level::Queen, Color::Black)),
                    Some(Piece::new(Level::King, Color::Black)),
                    Some(Piece::new(Level::Bishop, Color::Black)),
                    Some(Piece::new(Level::Knight, Color::Black)),
                    Some(Piece::new(Level::Rook, Color::Black)),
                ],
            ],
            history: vec![]
        }
    }

    /// Print chess board for debuging purposes
    pub fn print(&self) -> std::fmt::Result {
        for (i, row) in self.cells.iter().enumerate().rev() {
            print!("{} ", i+1);
            for cell in row.iter() {
                if let Some(piece) = cell {
                    print!("{} ", piece);
                } else {
                    print!("⬛");
                };
            };
            print!("\n");
        };
        print!("  a b c d e f g h\n");
        io::stdout().flush().unwrap();
        Ok(())
    }

    /// Get a piece on the board
    ///
    /// ### Examples
    ///
    /// ```
    /// # use chess::game::prelude::*;
    /// let board = Board::new();
    /// let piece = board.get("a1")?;
    ///
    /// assert_eq!(piece.unwrap().color, Color::White);
    /// assert_eq!(piece.unwrap().level, Level::Rook);
    /// # assert!(board.get("a3")?.is_none());
    /// # Ok::<(), Error>(())
    /// ```
    pub fn get(&self, cell: &str) -> Result<Option<Piece>, Error> {
        let (row, col) = convert(cell)?;
        Ok(self.cells[row as usize][col as usize].clone())
    }

    /// Set a piece on the board
    ///
    /// ### Examples
    ///
    /// ```
    /// # use chess::game::prelude::*;
    /// let mut board = Board::new();
    /// let white_queen = Piece::new(Level::Queen, Color::White);
    /// board.set("a1", Some(white_queen));
    ///
    /// # let piece = board.get("a1")?;
    /// # assert_eq!(piece.unwrap().color, Color::White);
    /// # assert_eq!(piece.unwrap().level, Level::Queen);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn set(&mut self, cell: &str, piece: Option<Piece>) -> Result<(), Error> {
        let (row, col) = convert(cell)?;
        self.cells[row as usize][col as usize] = piece;
        Ok(())
    }

    /// Get moves history
    pub fn get_history(&self) -> &Vec<History> {
        &self.history
    }

    /// Write moves history
    fn write_history(
        &mut self,
        from: String,
        to: String,
        piece: Option<Piece>,
        has_moved: Option<bool>
    ) -> Result<(), Error> {
        Ok(self.history.push(History { from, to, captured: piece, has_moved }))
    }

    /// Show captured piece by color
    pub fn get_captured(&self, color: Color) -> Result<Vec<Piece>, Error> {
        let mut captured_piece = vec![];
        for hist in self.history.iter() {
            if let Some(piece) = hist.captured.clone() {
                if piece.color == color {
                    captured_piece.push(piece);
                }
            }
        }
        Ok(captured_piece)
    }

    /// Get all possible moves for a pawn
    fn get_possible_moves_for_pawn(&self, cell: &str) -> Result<Vec<String>, Error>  {
        let piece = self.get(cell)?.unwrap();
        let (row, col) = convert(cell)?;
        let mut pos_mv = vec![];
        let mvu = piece.mv_unit.unwrap();

        // pawn is on the edge of the board, it has no possible moves
        if (row + mvu) > 7 || (row + mvu) < 0 {
            return Ok(pos_mv)
        }

        // Check blocked
        let pos1 = invert(row + mvu, col)?;
        if let None = self.get(&pos1)? {
            pos_mv.push(pos1);

            // Check if it has moved
            if !piece.moved.unwrap() {
                let pos2 = invert(row + (2 * mvu), col)?;
                pos_mv.push(pos2);
            }
        }

        // Check if there is a piece in the "attack zone"
        if (col + 1) <= 7 {
            let pos3 = invert(row + mvu, col + 1)?;
            if let Some(p) = self.get(&pos3)? {
                if p.color != piece.color { pos_mv.push(pos3) }
            }
        }

        if (col - 1) >= 0 {
            let pos4 = invert(row + mvu, col - 1)?;
            if let Some(p) = self.get(&pos4)? {
                if p.color != piece.color { pos_mv.push(pos4) }
            }
        }

        pos_mv.sort();
        Ok(pos_mv)
    }

    /// Get all possible moves for a piece given the cell location
    ///
    /// ### Examples
    ///
    /// ```
    /// # use chess::game::prelude::*;
    /// let board = Board::new();
    /// let white_pawn = board.get_possible_moves("a2")?;
    /// let black_knight = board.get_possible_moves("b8")?;
    ///
    /// assert_eq!(white_pawn, ["a3", "a4"]);
    /// assert_eq!(black_knight, ["a6", "c6"]);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn get_possible_moves(&self, cell: &str) -> Result<Vec<String>, Error> {
        let piece = self.get(cell)?.unwrap();
        let (row, col) = convert(cell)?;
        let mut pos_move = vec![];
        let loop_vector ;

        match piece.level {
            Level::Pawn => return self.get_possible_moves_for_pawn(cell),
            _ => loop_vector = Movement::new(piece.level).vectors
        };

        for item in loop_vector.iter() {
            for (x, y) in item.iter()  {
                // If out of board, break
                if (row + y) > 7 || (row + y) < 0
                || (col + x) > 7 || (col + x) < 0 { break }

                // check if next cell empty, if it isn't, check the color
                let pos = invert(row + y, col + x)?;
                if let Some(p) = self.get(&pos)? {
                    if p.color == piece.color { break }
                    else {
                        pos_move.push(pos);
                        break;
                    }
                } else {
                    // the next cell is empty, push pos then continue looping
                    pos_move.push(pos);
                }
            }
        }

        pos_move.sort();
        Ok(pos_move)
    }

    /// Get possible moves for a cell return it as a string
    pub fn get_possible_moves_as_string(&self, cell: &str) -> String {
        match self.get_possible_moves(cell) {
            Ok(vek) => vek.join(" "),
            Err(_) => return "".to_owned()
        }
    }

    /// Get all possible moves for all pieces by color
    ///
    /// ### Examples
    ///
    /// ```
    /// # use chess::game::prelude::*;
    /// let mut board = Board::new();
    /// board.moves_piece("e2", "e4")?;
    /// let paz = board.get_possible_moves_by_color(Color::White)?;
    ///
    /// assert_eq!(*paz.get("c2").unwrap(), ["c3", "c4"]);
    /// assert_eq!(*paz.get("d1").unwrap(), ["e2", "f3", "g4", "h5"]);
    /// assert_eq!(*paz.get("e1").unwrap(), ["e2"]);
    /// assert_eq!(*paz.get("f1").unwrap(), ["a6", "b5", "c4", "d3", "e2"]);
    /// assert_eq!(*paz.get("g1").unwrap(), ["e2", "f3", "h3"]);
    /// assert_eq!(paz.get("h1"), None);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn get_possible_moves_by_color(&self, color: Color) -> Result<HashMap<String, Vec<String>>, Error> {
        let all_pos = self.get_pieces_positions_by_color(color)?;
        let mut possible_moves = HashMap::new();

        for pos in all_pos.iter() {
            let pm = self.get_possible_moves(pos)?;
            if pm.len() != 0 {
                possible_moves.insert(pos.to_owned(), pm);
            }
        }

        Ok(possible_moves)
    }

    /// Moves a piece on the board
    ///
    /// ### Examples
    ///
    /// ```
    /// # use chess::game::prelude::*;
    /// let mut board = Board::new();
    /// board.moves_piece("a2", "a4")?;
    ///
    /// assert_eq!(board.get("a4")?.unwrap().level, Level::Pawn);
    /// assert_eq!(board.get("a4")?.unwrap().color, Color::White);
    /// assert!(board.get("a2")?.is_none());
    /// # Ok::<(), Error>(())
    /// ```
    pub fn moves_piece(&mut self, src_cell: &str, des_cell: &str) -> Result<String, Error> {
        let mut src_piece = self.get(src_cell)?.unwrap();
        let des_piece = self.get(des_cell)?;
        let mut has_moved = None;

        if !self.get_possible_moves(src_cell)?.iter().any(|s| { s == &des_cell }) {
            return Err(IllegalMoves(format!("can't move {} to {}", src_cell, des_cell)));
        }

        // change moved to true for pawn, rook, or a king
        match &src_piece.level {
            Level::Pawn | Level::Rook | Level::King =>
                if !src_piece.moved.unwrap() {
                    src_piece.moved = Some(true);
                    has_moved = Some(true);
                },
            _ => ()
        }

        // do the actual moves
        self.set(des_cell, Some(src_piece.clone()))?;
        self.set(src_cell, None)?;

        // write the moves to board history
        self.write_history(src_cell.to_owned(), des_cell.to_owned(), des_piece.clone(), has_moved)?;

        match des_piece {
            Some(piece) => Ok(format!("Moving {} {} from {} to {}, captured {} {}", src_piece.color, src_piece.level, src_cell, des_cell, piece.color, piece.level)),
            None => Ok(format!("Moving {} {} from {} to {}", src_piece.color, src_piece.level, src_cell, des_cell))
        }
    }

    /// Undo the last moves in board history
    ///
    /// ### Examples
    ///
    /// ```
    /// # use chess::game::prelude::*;
    /// let mut board = Board::new();
    /// board.moves_piece("a2", "a4")?;
    /// board.undo_moves()?;
    ///
    /// assert_eq!(board.get("a2")?.unwrap().level, Level::Pawn);
    /// assert_eq!(board.get("a2")?.unwrap().color, Color::White);
    /// assert!(board.get("a4")?.is_none());
    /// # Ok::<(), Error>(())
    /// ```
    pub fn undo_moves(&mut self) -> Result<String, Error> {
        if self.history.len() == 0 {
            return Err(GameError("Already the oldest state.".to_owned()));
        }

        let his = self.history.pop().unwrap();
        let mut piece = self.get(&his.to)?.unwrap();

        match his.has_moved {
            Some(_val) => piece.moved = Some(false),
            None => ()
        }

        self.set(&his.from, Some(piece))?;
        self.set(&his.to, his.captured)?;

        Ok(format!("Undo the moves from {} to {}", his.from, his.to))
    }

    /// Pawn Promotion
    ///
    /// Pawn promotion occurs when a pawn reaches the farthest rank
    /// from its original square—the eighth rank for White and
    /// first rank for Black. When this happens, the player can
    /// replace the pawn for a queen, a rook, a bishop, or a knight.
    ///
    pub fn promote(&mut self, cell: &str, promotion_level: Level) -> Result<String, Error> {
        let piece = self.get(cell)?.unwrap();

        match piece.level {
            Level::Pawn => (),
            _ => return Err(PromotionError("Only a pawn can be promoted.".to_owned()))
        }

        let (row, _col) = convert(cell)?;
        match piece.color {
            Color::White => if row != 7 { return Err(PromotionError("Can't promote this pawn yet".to_owned())) },
            Color::Black => if row != 0 { return Err(PromotionError("Can't promote this pawn yet".to_owned())) }
        }

        match promotion_level {
            Level::Pawn => return Err(PromotionError("Can't promote a pawn to a pawn.".to_owned())),
            Level::King => return Err(PromotionError("Can't promote a pawn to a king.".to_owned())),
            _ => ()
        }

        let promotion_piece = Piece::new(promotion_level, piece.color);
        self.set(cell, Some(promotion_piece))?;

        Ok(format!("Promoted to {:?}", promotion_level))
    }

    /// Castling
    ///
    /// The rules for castling
    /// * castling is only possible if neither the king nor the rook has moved
    /// * there must not be any pieces between the king and the rook
    /// * the king may not be in check
    /// * the square the king goes to and any intervening squares may not be under attack
    /// * however, there is nothing to prevent castling if the rook is under attack
    ///
    pub fn castling(&mut self, king_cell: &str, rook_cell: &str) -> Result<String, Error> {
        let mut king = self.get(king_cell)?.unwrap();
        let mut rook = self.get(rook_cell)?.unwrap();
        let (king_row, king_col) = convert(king_cell)?;
        let (_rook_row, rook_col) = convert(rook_cell)?;
        let enemy_color = get_enemy_color(king.color);

        if king.color != rook.color {
            return Err(CastlingError("King and Rook have different color".to_owned()))
        }

        if self.is_king_checked(king.color)? {
            return Err(CastlingError("King is in check.".to_owned()));
        }

        if king.moved.unwrap() || rook.moved.unwrap() {
            return Err(CastlingError("King or Rook has already moved".to_owned()))
        }

        let (start, end, wr, wk, br, bk);
        if king_col < rook_col {
            (start, end, wr, wk, br, bk) = (king_col, rook_col, "f1", "g1", "f8", "g8");
        } else {
            (start, end, wr, wk, br, bk) = (rook_col, king_col, "d1", "c1", "d8", "c8");
        }

        let paz = self.get_possible_attack_by_color(enemy_color)?;
        for y in (start + 1)..end {
            if let Some(_piece) = self.get(&invert(king_row, y)?)? {
                return Err(CastlingError("Can't do castling, the path is blocked".to_owned()));
            }
            for (_key, val) in paz.iter() {
                if val.iter().any(|p| { p == &invert(king_row, y).unwrap() }) {
                    return Err(CastlingError("Can't do castling, the path is under attack".to_owned()));
                }
            }
        }

        king.moved = Some(true);
        rook.moved = Some(true);
        match king.color {
            Color::White => {
                // self.moves_piece(rook_cell, wr).unwrap();
                self.set(wr, Some(rook))?;
                self.set(rook_cell, None)?;
                self.set(wk, Some(king))?;
                self.set(king_cell, None)?;
            },
            Color::Black => {
                // self.moves_piece(rook_cell, br).unwrap();
                self.set(br, Some(rook))?;
                self.set(rook_cell, None)?;
                self.set(bk, Some(king))?;
                self.set(king_cell, None)?;
            }
        }

        self.write_history(king_cell.to_string(), rook_cell.to_string(), None, Some(true))?;
        Ok(format!("Castling {} and {}", king_cell, rook_cell))
    }

    /// Get all pieces position on the board by its color
    pub fn get_pieces_positions_by_color(&self, color: Color) -> Result<Vec<String>, Error> {
        let mut all_pos = vec![];

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(piece) = cell {
                    if piece.color == color {
                        let pos = invert(y as i8, x as i8)?;
                        all_pos.push(pos);
                    }
                }
            }
        }

        Ok(all_pos)
    }

    /// Get possible attack zone for a pawn
    fn get_possible_attack_for_pawn(&self, cell: &str) -> Result<Vec<String>, Error> {
        let piece = self.get(cell)?.unwrap();
        let (row, col) = convert(cell)?;
        let mut att = vec![];
        let mvu = piece.mv_unit.unwrap();

        // pawn is on the edge of the board, it has no possible attacks
        if (row + mvu) > 7 || (row + mvu) < 0 {
            return Ok(att)
        }

        // Check if there is a piece in the "attack zone"
        if (col + 1) <= 7 {
            let pos3 = invert(row + mvu, col + 1)?;
            if let Some(p) = self.get(&pos3)? {
                if p.color != piece.color { att.push(pos3) }
            } else {
                att.push(pos3);
            }
        }

        if (col - 1) >= 0 {
            let pos4 = invert(row + mvu, col - 1)?;
            if let Some(p) = self.get(&pos4)? {
                if p.color != piece.color { att.push(pos4) }
            } else {
                att.push(pos4);
            }
        }

        att.sort();
        Ok(att)
    }

    /// Get all under attack cells by piece color
    ///
    /// ### Examples
    ///
    /// ```
    /// # use chess::game::prelude::*;
    /// let mut board = Board::new();
    /// board.moves_piece("e2", "e4")?;
    /// let paz = board.get_possible_attack_by_color(Color::White)?;
    ///
    /// assert_eq!(*paz.get("c2").unwrap(), ["b3", "d3"]);
    /// assert_eq!(*paz.get("d1").unwrap(), ["e2", "f3", "g4", "h5"]);
    /// assert_eq!(*paz.get("e1").unwrap(), ["e2"]);
    /// assert_eq!(*paz.get("f1").unwrap(), ["a6", "b5", "c4", "d3", "e2"]);
    /// assert_eq!(*paz.get("g1").unwrap(), ["e2", "f3", "h3"]);
    /// assert_eq!(paz.get("h1"), None);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn get_possible_attack_by_color(&self, color: Color) -> Result<HashMap<String, Vec<String>>, Error> {
        let all_pos = self.get_pieces_positions_by_color(color)?;
        let mut attack_pos = HashMap::new();
        let mut paz: Vec<String>;

        for pos in all_pos.iter() {
            paz = match self.get(&pos)?.unwrap().level {
                Level::Pawn => self.get_possible_attack_for_pawn(pos)?,
                _ => self.get_possible_moves(pos)?
            };
            if paz.len() != 0 {
                attack_pos.insert(pos.to_owned(), paz);
            }
        }

        Ok(attack_pos)
    }

    /// Get king position given the color
    ///
    /// ### Examples
    ///
    /// ```
    /// # use chess::game::prelude::*;
    /// let board = Board::new();
    /// assert_eq!(board.get_king_position(Color::White)?, "e1");
    /// assert_eq!(board.get_king_position(Color::Black)?, "e8");
    /// # Ok::<(), Error>(())
    /// ```
    pub fn get_king_position(&self, color: Color) -> Result<String, Error> {
        for pos in self.get_pieces_positions_by_color(color)?.iter() {
            if self.get(pos)?.unwrap().level == Level::King {
                return Ok(pos.to_owned());
            }
        }
        Err(GameError("King is not found.".to_owned()))
    }

    /// Checking if the king is under attack (check)
    ///
    /// ### Examples
    ///
    /// ```
    /// # use chess::game::prelude::*;
    /// let mut board = Board::new();
    /// board.moves_piece("e2", "e4")?;
    /// board.moves_piece("f7", "f6")?;
    /// board.moves_piece("d1", "h5")?;
    ///
    /// assert_eq!(board.is_king_checked(Color::Black)?, true);
    /// assert_eq!(board.is_king_checked(Color::White)?, false);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn is_king_checked(&self, king_color: Color) -> Result<bool, Error> {
        let enemy_color = get_enemy_color(king_color);
        let king_pos = self.get_king_position(king_color)?;

        for (_key, val) in self.get_possible_attack_by_color(enemy_color)? {
            if val.iter().any(|pos| { pos == &king_pos }) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Checking if checkmate
    pub fn is_checkmate(&mut self, king_color: Color) -> Result<bool, Error> {
        if !self.is_king_checked(king_color)? {
            return Ok(false)
        }

        for (piece, pos_moves) in self.get_possible_moves_by_color(king_color)?.iter() {
            for pos in pos_moves.iter() {
                self.moves_piece(piece, pos)?;
                if !self.is_king_checked(king_color)? {
                    self.undo_moves()?;
                    return Ok(false);
                }
                self.undo_moves()?;
            }
        }

        Ok(true)
    }

    /// Check is there a safe move(s) given the color
    pub fn has_safe_moves(&mut self, color: Color) -> Result<bool, Error> {
        for (piece, pos_moves) in self.get_possible_moves_by_color(color)?.iter() {
            for pos in pos_moves.iter() {
                self.moves_piece(piece, pos)?;
                if !self.is_king_checked(color)? {
                    self.undo_moves()?;
                    return Ok(true);
                }
                self.undo_moves()?;
            }
        }
        Ok(false)
    }

    /// Check if it draw (no more possible moves)
    pub fn is_draw(&mut self, color: Color) -> Result<bool, Error> {
        if self.is_king_checked(color)? { return Ok(false) }
        if self.has_safe_moves(color)? { return Ok(false) }
        Ok(true)
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn board_possible_moves_pawn() -> Result<(), Error> {
        let board = Board::new();
        let moves = board.get_possible_moves_for_pawn("a2");
        assert_eq!(moves?, ["a3", "a4"]);
        Ok(())
    }

    #[test]
    fn board_possible_moves_knight() -> Result<(), Error> {
        let board = Board::new();
        let moves = board.get_possible_moves("b1");
        assert_eq!(moves?, ["a3", "c3"]);
        Ok(())
    }

    #[test]
    fn board_possible_moves_rook() -> Result<(), Error> {
        let mut board = Board::new();
        board.moves_piece("a2", "a4")?;
        let moves = board.get_possible_moves("a1");
        assert_eq!(moves?, ["a2", "a3"]);
        Ok(())
    }

    #[test]
    fn board_possible_moves_bishop() -> Result<(), Error> {
        let mut board = Board::new();
        board.moves_piece("e2", "e4")?;
        let moves = board.get_possible_moves("f1");
        assert_eq!(moves?, ["a6", "b5", "c4", "d3", "e2"]);
        Ok(())
    }

    #[test]
    fn board_possible_moves_queen() -> Result<(), Error> {
        let mut board = Board::new();
        board.moves_piece("d2", "d4")?;
        board.moves_piece("e2", "e4")?;
        let moves = board.get_possible_moves("d1");
        assert_eq!(moves?, ["d2", "d3", "e2", "f3", "g4", "h5"]);
        Ok(())
    }

    #[test]
    fn board_possible_moves_king() -> Result<(), Error> {
        let mut board = Board::new();
        board.moves_piece("d2", "d4")?;
        board.moves_piece("e2", "e4")?;
        let moves = board.get_possible_moves("e1");
        assert_eq!(moves?, ["d2", "e2"]);
        Ok(())
    }

    #[test]
    fn board_possible_moves_by_color() -> Result<(), Error>  {
        let mut board = Board::new();
        board.moves_piece("e2", "e4")?;
        let paz = board.get_possible_moves_by_color(Color::White)?;
        assert_eq!(*paz.get("c2").unwrap(), ["c3", "c4"]);
        assert_eq!(*paz.get("d1").unwrap(), ["e2", "f3", "g4", "h5"]);
        assert_eq!(*paz.get("e1").unwrap(), ["e2"]);
        assert_eq!(*paz.get("f1").unwrap(), ["a6", "b5", "c4", "d3", "e2"]);
        assert_eq!(*paz.get("g1").unwrap(), ["e2", "f3", "h3"]);
        assert_eq!(paz.get("h1"), None);
        Ok(())
    }

    #[test]
    fn board_moves_piece() -> Result<(), Error> {
        let mut board = Board::new();
        board.moves_piece("a2", "a4")?;
        assert_eq!(board.get("a4")?.unwrap().level, Level::Pawn);
        assert_eq!(board.get("a4")?.unwrap().color, Color::White);
        assert!(board.get("a2")?.is_none());
        assert_eq!(board.history.len(), 1);
        assert_eq!(board.history[0].from, "a2");
        assert_eq!(board.history[0].to, "a4");
        assert!(board.history[0].captured.is_none());
        assert!(board.history[0].has_moved.unwrap());
        Ok(())
    }

    #[test]
    fn board_undo_moves() -> Result<(), Error> {
        let mut board = Board::new();
        board.moves_piece("a2", "a4")?;
        board.undo_moves()?;
        let piece = board.get("a2")?.unwrap();
        assert_eq!(piece.level, Level::Pawn);
        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.moved, Some(false));
        assert!(board.get("a4")?.is_none());
        assert_eq!(board.history.len(), 0);
        Ok(())
    }

    #[test]
    fn board_possible_attack_by_color() -> Result<(), Error> {
        let mut board = Board::new();
        board.moves_piece("e2", "e4")?;
        let paz = board.get_possible_attack_by_color(Color::White)?;
        assert_eq!(*paz.get("c2").unwrap(), ["b3", "d3"]);
        assert_eq!(*paz.get("d1").unwrap(), ["e2", "f3", "g4", "h5"]);
        assert_eq!(*paz.get("e1").unwrap(), ["e2"]);
        assert_eq!(*paz.get("f1").unwrap(), ["a6", "b5", "c4", "d3", "e2"]);
        assert_eq!(*paz.get("g1").unwrap(), ["e2", "f3", "h3"]);
        assert_eq!(paz.get("h1"), None);
        Ok(())
    }

    #[test]
    fn board_get_king_position() -> Result<(), Error> {
        let board = Board::new();
        assert_eq!(board.get_king_position(Color::White)?, "e1");
        assert_eq!(board.get_king_position(Color::Black)?, "e8");
        Ok(())
    }

    #[test]
    fn board_is_king_checked() -> Result<(), Error> {
        let mut board = Board::new();
        board.moves_piece("e2", "e4")?;
        board.moves_piece("f7", "f6")?;
        board.moves_piece("d1", "h5")?;
        assert_eq!(board.is_king_checked(Color::Black)?, true);
        assert_eq!(board.is_king_checked(Color::White)?, false);
        Ok(())
    }

    #[test]
    fn board_is_checkmate() -> Result<(), Error> {
        let mut board = Board::new();
        board.moves_piece("e2", "e4")?;
        board.moves_piece("e7", "e5")?;
        board.moves_piece("d1", "f3")?;
        board.moves_piece("b8", "c6")?;
        board.moves_piece("f1", "c4")?;
        board.moves_piece("f8", "c5")?;
        assert!(!board.is_checkmate(Color::Black)?);
        board.moves_piece("f3", "f7")?;
        assert!(board.is_checkmate(Color::Black)?);
        Ok(())
    }

    #[test]
    fn board_castling() -> Result<(), Error> {
        let mut board = Board::new();
        board.moves_piece("e2", "e4")?;
        board.moves_piece("e7", "e5")?;
        board.moves_piece("g1", "f3")?;
        board.moves_piece("b8", "c6")?;
        board.moves_piece("f1", "c4")?;
        board.moves_piece("f8", "c5")?;
        board.castling("e1", "h1")?;
        assert!(board.get("e1")?.is_none());
        assert_eq!(board.get("f1")?.unwrap().level, Level::Rook);
        assert_eq!(board.get("g1")?.unwrap().level, Level::King);
        assert!(board.get("h1")?.is_none());
        Ok(())
    }

    #[test]
    fn board_promote() -> Result<(), Error> {
        let mut board = Board::new();
        board.moves_piece("h2", "h4")?;
        board.moves_piece("h4", "h5")?;
        board.moves_piece("h5", "h6")?;
        board.moves_piece("h6", "g7")?;
        board.moves_piece("g7", "f8")?;
        board.promote("f8", Level::Queen)?;
        assert_eq!(board.get("f8")?.unwrap().level, Level::Queen);
        assert_eq!(board.get("f8")?.unwrap().color, Color::White);
        Ok(())
    }

}
