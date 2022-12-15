use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::game::{board::Board, piece::Piece, color};

/// Statuses for the game: `Waiting`, `Playing`, and `GameOver`
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum Status {
    #[default]
    Waiting,
    Playing,
    GameOver
}

impl Status {
    /// Parse `Status` as `&str`
    pub fn as_str(&self) -> &str {
        match *self {
            Status::Waiting => "waiting",
            Status::Playing => "playing",
            Status::GameOver => "game over"
        }
    }
}

/// Winners for the game: `NotDecided`, `Draw`, `White`, and `Black`
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum Winner {
    #[default]
    NotDecided,
    Draw,
    White,
    Black
}

impl Winner {
    /// Convert `Color` to `Winner`
    pub fn from_color(color: Color) -> Winner {
        match color {
            Color::Black => Winner::Black,
            Color::White => Winner::White
        }
    }
}

/// Player's Color: `White` or `Black`
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black
}

impl Color {
    /// Get the opposite `Color`
    pub fn opposite(&self) -> Color {
        match *self {
            Color::Black => Color::White,
            Color::White => Color::Black
        }
    }

    /// Parse `Color` as `&str`
    pub fn as_str(&self) -> &str {
        match *self {
            Color::White => "white",
            Color::Black => "black"
        }
    }

    /// Parse `model::Color` as `game::color::Color`
    pub fn as_color(&self) -> color::Color {
        match *self {
            Color::Black => color::Color::Black,
            Color::White => color::Color::White
        }
    }
}

/// Chess `Game` Struct
#[derive(Debug)]
pub struct Game {
    pub players: HashMap<Uuid, Color>,
    pub status: Status,
    pub winner: Winner,
    pub board: Board<Piece>
}

impl Game {
    /// Create new `Game` instance
    pub fn new() -> Game {
        Game {
            players: HashMap::new(),
            status: Status::default(),
            winner: Winner::default(),
            board: Board::new()
        }
    }
}
