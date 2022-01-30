use crate::util::{Element, SCError, SCResult};

use super::{PieceType, Team, Vec2};

/// A placeable figure on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    /// Type of the (topmost) piece.
    piece_type: PieceType,
    /// Which team this piece belongs to.
    team: Team,
    /// Number of pieces in this castle.
    count: usize,
}

impl Piece {
    /// Creates a new piece.
    pub fn new(piece_type: PieceType, team: Team, count: usize) -> Self {
        Self { piece_type, team, count }
    }

    /// The type of the (topmost) piece.
    #[inline]
    pub fn piece_type(self) -> PieceType { self.piece_type }

    /// Which team this piece belongs to.
    #[inline]
    pub fn team(self) -> Team { self.team }

    /// Number of pieces in this castle.
    #[inline]
    pub fn count(self) -> usize { self.count }

    /// Returns whether the piece can be turned into an amber.
    #[inline]
    pub fn is_amber(self) -> bool { self.count >= 3 }

    /// A new piece that captures the other piece.
    pub fn capture(self, other: Self) -> Self {
        let mut captured = self;
        captured.count += other.count;
        captured
    }

    /// The directions this piece can move in.
    pub fn possible_directions(self) -> impl Iterator<Item=Vec2> {
        let direction = self.team.direction();
        self.piece_type.possible_directions()
            .into_iter()
            .map(move |v| Vec2::new(v.x * direction, v.y))
    }
}

impl TryFrom<&Element> for Piece {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Piece {
            piece_type: elem.attribute("type")?.parse()?,
            team: elem.attribute("team")?.parse()?,
            count: elem.attribute("count")?.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{util::Element, game::{Piece, PieceType, Team}};

    #[test]
    fn test_parsing() {
        assert_eq!(Piece::try_from(&Element::from_str(r#"
            <piece type="Herzmuschel" team="TWO" count="1" />
        "#).unwrap()).unwrap(), Piece {
            piece_type: PieceType::Herzmuschel,
            team: Team::Two,
            count: 1,
        });
    }
}
