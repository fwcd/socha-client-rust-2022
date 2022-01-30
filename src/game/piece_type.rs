use std::str::FromStr;
use std::fmt;

use crate::util::{SCError, SCResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    /// Moves only diagonally forwards.
    Herzmuschel,
    /// Moves only to adjacent fields.
    Moewe,
    /// Moves only diagonally or forwards.
    Seestern,
    /// Like a knight in chess. Only non-light figure.
    Robbe,
}

impl PieceType {
    /// Checks whether a piece is lightweight. Only the 'robbe' is non-light.
    pub fn is_light(self) -> bool {
        !matches!(self, Self::Robbe)
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieceType::Herzmuschel => write!(f, "Herzmuschel"),
            PieceType::Moewe => write!(f, "Moewe"),
            PieceType::Seestern => write!(f, "Seestern"),
            PieceType::Robbe => write!(f, "Robbe"),
        }
    }
}

impl FromStr for PieceType {
    type Err = SCError;

    fn from_str(s: &str) -> SCResult<Self> {
        match s {
            "Herzmuschel" => Ok(Self::Herzmuschel),
            "Moewe" => Ok(Self::Moewe),
            "Seestern" => Ok(Self::Seestern),
            "Robbe" => Ok(Self::Robbe),
            _ => Err(SCError::UnknownVariant(format!("Unknown piece type {}", s))),
        }
    }
}
