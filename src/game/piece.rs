use crate::util::{Element, SCError, SCResult};

use super::{PieceType, Team};

/// A placeable figure on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    /// Type of the (topmost) piece.
    r#type: PieceType,
    /// Which team this piece belongs to.
    team: Team,
    /// Number of pieces in this rook.
    count: usize,
}

impl TryFrom<&Element> for Piece {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Piece {
            r#type: elem.attribute("type")?.parse()?,
            team: elem.attribute("team")?.parse()?,
            count: elem.attribute("count")?.parse()?,
        })
    }
}
