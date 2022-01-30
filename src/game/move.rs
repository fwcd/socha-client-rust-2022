use crate::util::{Element, SCError, SCResult};

use super::Coords;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    from: Coords,
    to: Coords,
}

impl TryFrom<&Element> for Move {
    type Error = SCError;

    fn try_from(element: &Element) -> SCResult<Self> {
        Ok(Move {
            from: element.child_by_name("from")?.try_into()?,
            to: element.child_by_name("to")?.try_into()?,
        })
    }
}
