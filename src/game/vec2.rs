use std::{fmt, ops::{Add, Sub, Mul, Div, DivAssign, MulAssign}};

use crate::util::{Element, SCError, SCResult};

/// A position on the board or 2D vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    #[inline]
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    #[inline]
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn x(self) -> usize { self.x }

    #[inline]
    pub fn y(self) -> usize { self.y }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<usize> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vec2> for usize {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self * rhs.x, self * rhs.y)
    }
}

impl MulAssign<usize> for Vec2 {
    fn mul_assign(&mut self, rhs: usize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<usize> for Vec2 {
    type Output = Self;

    fn div(self, rhs: usize) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<usize> for Vec2 {
    fn div_assign(&mut self, rhs: usize) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl TryFrom<&Element> for Vec2 {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Vec2::new(elem.attribute("x")?.parse()?, elem.attribute("y")?.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{util::Element, game::Vec2};

    #[test]
    fn test_parsing() {
        assert_eq!(Vec2::try_from(&Element::from_str(r#"
            <coords x="23" y="0" />
        "#).unwrap()).unwrap(), Vec2::new(23, 0));
    }
}
