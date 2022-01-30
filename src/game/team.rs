use std::str::FromStr;
use std::fmt;

use crate::util::{SCError, SCResult};

pub enum Team {
    One,
    Two,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Team::One => write!(f, "ONE"),
            Team::Two => write!(f, "TWO"),
        }
    }
}

impl FromStr for Team {
    type Err = SCError;

    fn from_str(s: &str) -> SCResult<Self> {
        match s {
            "ONE" => Ok(Team::One),
            "TWO" => Ok(Team::Two),
            _ => Err(SCError::UnknownVariant(format!("Unknown team {}", s))),
        }
    }
}
