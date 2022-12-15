//! Custom Error for Chess Game

use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    InvalidNotation(String),
    PromotionError(String),
    CastlingError(String),
    IllegalMoves(String),
    GameError(String)
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidNotation(msg) => write!(f, "Invalid Chess Notation: {msg}"),
            Error::PromotionError(msg) => write!(f, "Promotion Error: {msg}"),
            Error::CastlingError(msg) => write!(f, "Castling Error: {msg}"),
            Error::IllegalMoves(msg) => write!(f, "Invalid Moves: {msg}"),
            Error::GameError(msg) => write!(f, "Game Error: {msg}")
        }
    }
}
