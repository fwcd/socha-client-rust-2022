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

impl From<Move> for Element {
    fn from(m: Move) -> Self {
        Element::new("data")
            .attribute("class", "move")
            .child(Element::new("from").attribute("x", m.from.x()).attribute("y", m.from.y()))
            .child(Element::new("to").attribute("x", m.to.x()).attribute("y", m.to.y()))
            .build()
    }
}
