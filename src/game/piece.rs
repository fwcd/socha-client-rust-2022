use crate::util::{Element, SCError, SCResult};

/// A placeable figure on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    // TODO
}

impl TryFrom<&Element> for Piece {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        // TODO
        Ok(Piece {})
    }
}
