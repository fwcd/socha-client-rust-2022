use crate::util::{Element, SCError, SCResult};

use super::Coords;

/// An action in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    from: Coords,
    to: Coords,
}

impl Move {
    #[inline]
    pub fn new(from: Coords, to: Coords) -> Self {
        Self { from, to }
    }

    #[inline]
    pub fn from(self) -> Coords { self.from }

    #[inline]
    pub fn to(self) -> Coords { self.to }
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{util::Element, game::{Move, Coords}};

    #[test]
    fn test_parsing() {
        assert_eq!(Move::try_from(&Element::from_str(r#"
            <data class="move">
                <from x="3" y="4" />
                <to x="5" y="9" />
            </data>
        "#).unwrap()).unwrap(), Move {
            from: Coords::new(3, 4),
            to: Coords::new(5, 9),
        });
    }
}
