use std::fmt;

use crate::util::{Element, SCError, SCResult};

/// A position on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    #[inline]
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn x(&self) -> usize { self.x }

    #[inline]
    pub fn y(&self) -> usize { self.y }
}

impl TryFrom<&Element> for Coords {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Coords::new(elem.attribute("x")?.parse()?, elem.attribute("y")?.parse()?))
    }
}
