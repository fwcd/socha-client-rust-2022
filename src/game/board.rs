use std::{collections::HashMap, ops::Index};

use crate::util::{Element, SCError, SCResult};

use super::{Vec2, Piece};

/// An 8x8 game board storing the pieces (8 pieces per team).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    /// The pieces on the board keyed by position.
    pieces: HashMap<Vec2, Piece>,
}

impl Board {
    /// Creates a new empty board.
    pub fn empty() -> Self {
        Self { pieces: HashMap::new() }
    }

    /// Creates a new board with the given pieces.
    pub fn new(pieces: impl Into<HashMap<Vec2, Piece>>) -> Self {
        Self { pieces: pieces.into() }
    }

    /// The pieces on the board.
    pub fn pieces(&self) -> &HashMap<Vec2, Piece> { &self.pieces }

    /// Fetches a piece on the board.
    pub fn get(&self, pos: Vec2) -> Option<Piece> { self.pieces.get(&pos).cloned() }

    /// Fetches a piece on the board mutably.
    pub fn get_mut(&mut self, pos: Vec2) -> Option<&mut Piece> { self.pieces.get_mut(&pos) }
}

impl Index<Vec2> for Board {
    type Output = Piece;

    fn index(&self, index: Vec2) -> &Piece {
        &self.pieces[&index]
    }
}

impl TryFrom<&Element> for Board {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Board {
            pieces: elem
                .child_by_name("pieces")?
                .childs_by_name("entry")
                .map(|e| {
                    let coords = Vec2::try_from(e.child_by_name("coordinates")?)?;
                    let piece = Piece::try_from(e.child_by_name("piece")?)?;
                    Ok((coords, piece))
                })
                .collect::<SCResult<_>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{util::Element, game::{Piece, PieceType, Team, Board, Vec2}, hashmap};

    #[test]
    fn test_parsing() {
        assert_eq!(Board::try_from(&Element::from_str(r#"
            <board>
                <pieces>
                    <entry>
                        <coordinates x="0" y="0" />
                        <piece type="Herzmuschel" team="ONE" count="1" />
                    </entry>
                    <entry>
                        <coordinates x="1" y="0" />
                        <piece type="Robbe" team="TWO" count="1" />
                    </entry>
                </pieces>
            </board>
        "#).unwrap()).unwrap(), Board::new(hashmap![
            Vec2::new(0, 0) => Piece::new(PieceType::Herzmuschel, Team::One, 1),
            Vec2::new(1, 0) => Piece::new(PieceType::Robbe, Team::Two, 1)
        ]));
    }
}
