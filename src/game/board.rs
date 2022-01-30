use std::collections::HashMap;

use crate::util::{Element, SCError, SCResult};

use super::{Coords, Piece};

/// An 8x8 game board storing the pieces (8 pieces per team).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    /// The pieces on the board keyed by position.
    pieces: HashMap<Coords, Piece>,
}

impl Board {
    /// The pieces on the board.
    pub fn pieces(&self) -> &HashMap<Coords, Piece> { &self.pieces }
}

impl TryFrom<&Element> for Board {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Board {
            pieces: elem
                .child_by_name("pieces")?
                .childs_by_name("entry")
                .map(|e| {
                    let coords = Coords::try_from(e.child_by_name("coordinates")?)?;
                    let piece = Piece::try_from(e.child_by_name("piece")?)?;
                    Ok((coords, piece))
                })
                .collect::<SCResult<_>>()?,
        })
    }
}
