use std::collections::HashMap;

use crate::util::{Element, SCError, SCResult};

use super::{Coords, Piece};

/// An 8x8 game board storing the pieces (8 pieces per team).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    /// The pieces on the board keyed by position.
    pieces: HashMap<Coords, Piece>,
}

impl Board {
    /// Creates a new empty board.
    pub fn empty() -> Self {
        Self { pieces: HashMap::new() }
    }

    /// Creates a new board with the given pieces.
    pub fn new(pieces: impl Into<HashMap<Coords, Piece>>) -> Self {
        Self { pieces: pieces.into() }
    }

    /// The pieces on the board.
    pub fn pieces(&self) -> &HashMap<Coords, Piece> { &self.pieces }
}

impl TryFrom<&Element> for Board {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Board {
            pieces: elem
                .child_by_name("pieces")?
                .childs_by_name("entry")
                .map(|e| {
                    let coords = Coords::try_from(e.child_by_name("coordinates")?)?;
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

    use crate::{util::Element, game::{Piece, PieceType, Team, Board, Coords}, hashmap};

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
            Coords::new(0, 0) => Piece::new(PieceType::Herzmuschel, Team::One, 1),
            Coords::new(1, 0) => Piece::new(PieceType::Robbe, Team::Two, 1)
        ]));
    }
}
