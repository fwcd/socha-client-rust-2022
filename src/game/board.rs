use std::{collections::HashMap, ops::Index};

use crate::util::{Element, SCError, SCResult};

use super::{Vec2, Piece, Move, Team};

pub const BOARD_SIZE: usize = 8;

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

    /// Checks whether a position in in-bounds.
    pub fn is_in_bounds(pos: Vec2) -> bool {
        pos.x >= 0 && pos.x < BOARD_SIZE as i32 && pos.y >= 0 && pos.y < BOARD_SIZE as i32
    }

    /// Fetches the x-index of the team's 'home' line.
    pub fn start_line(team: Team) -> i32 {
        team.index() * (BOARD_SIZE as i32 - 1)
    }

    // Partially translated from https://github.com/software-challenge/backend/blob/89407e5e2f76801ec8beb8f31412da218f5f70e5/plugin/src/main/kotlin/sc/plugin2022/Board.kt

    /// Applies a move to the board.
    pub fn perform(&mut self, m: Move) {
        if let Some(piece) = self.pieces.remove(&m.from()) {
            debug_assert!(Board::is_in_bounds(m.to()), "Move destination {} wasn't in bounds!", m.to());
            debug_assert!(piece.possible_directions().any(|v| v == m.delta()), "Move delta {} isn't in the allowed move for the piece {:?}!", m.delta(), piece);
            let new_piece = self.pieces.get(&m.to()).map(|&p| piece.capture(p)).unwrap_or(piece);
            self.pieces.insert(m.to(), new_piece);
        } else {
            panic!("Cannot perform empty move!");
        }
    }

    /// Checks whether the piece at the given position should be turned
    /// into an amber and, if so, removes it.
    pub fn check_amber(&mut self, pos: Vec2) -> usize {
        if let Some(piece) = self.pieces.get_mut(&pos) {
            let ambers = [
                piece.is_amber(),
                piece.piece_type().is_light() && pos.x == Self::start_line(piece.team().opponent()),
            ].into_iter().map(|b| if b { 1 } else { 0 }).sum();
            if ambers > 0 {
                self.pieces.remove(&pos);
            }
            ambers
        } else {
            0
        }
    }

    /// Checks whether the given piece can jump to the destination, not
    /// accounting for whether the move itself is valid.
    pub fn can_move(&self, piece: Piece, dest: Vec2) -> bool {
        Board::is_in_bounds(dest) && self.pieces.get(&dest).map(|p| p.team()) != Some(piece.team())
    }

    /// Fetches possible move deltas from the given position.
    pub fn possible_destinations_from(&self, pos: Vec2) -> Vec<Vec2> {
        if let Some(piece) = self.get(pos) {
            piece.possible_directions()
                .filter(|&v| self.can_move(piece, pos + v))
                .collect()
        } else {
            Vec::new()
        }
    }
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
