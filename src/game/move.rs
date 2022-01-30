use std::fmt;

use crate::util::{Element, SCError, SCResult};

use super::Vec2;

/// An action in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    from: Vec2,
    to: Vec2,
}

impl Move {
    #[inline]
    pub fn new(from: Vec2, to: Vec2) -> Self {
        Self { from, to }
    }

    #[inline]
    pub fn from(self) -> Vec2 { self.from }

    #[inline]
    pub fn to(self) -> Vec2 { self.to }

    #[inline]
    pub fn delta(self) -> Vec2 { self.to - self.from }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
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
            .child(Element::new("from").attribute("x", m.from.x).attribute("y", m.from.y))
            .child(Element::new("to").attribute("x", m.to.x).attribute("y", m.to.y))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{util::Element, game::{Move, Vec2}};

    #[test]
    fn test_parsing() {
        assert_eq!(Move::try_from(&Element::from_str(r#"
            <data class="move">
                <from x="3" y="4" />
                <to x="5" y="9" />
            </data>
        "#).unwrap()).unwrap(), Move {
            from: Vec2::new(3, 4),
            to: Vec2::new(5, 9),
        });
    }
}
