use std::{str::FromStr, iter::once};
use std::fmt;

use crate::util::{SCError, SCResult};

use super::{Vec2, CARDINALS, DIAGONALS};

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
    #[inline]
    pub fn is_light(self) -> bool {
        !matches!(self, Self::Robbe)
    }

    /// The directions in which this piece is allowed to move.
    pub fn possible_directions(self) -> Vec<Vec2> {
        match self {
            Self::Herzmuschel => vec![Vec2::new(1, 1), Vec2::new(1, -1)],
            Self::Moewe => CARDINALS.into_iter().collect(),
            Self::Seestern => DIAGONALS.into_iter().chain(once(Vec2::new(1, 0))).collect(),
            Self::Robbe => DIAGONALS.into_iter().flat_map(|v| [
                Vec2::new(2 * v.x(), v.y()),
                Vec2::new(v.x(), 2 * v.y()),
            ].into_iter()).collect(),
        }
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
