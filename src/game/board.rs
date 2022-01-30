use crate::util::{Element, SCError, SCResult};

/// A game board storing the pieces.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    // TODO
}

impl TryFrom<&Element> for Board {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        // TODO
        Ok(Board {})
    }
}
