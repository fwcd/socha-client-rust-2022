use std::{collections::HashMap, str::FromStr};

use crate::util::{Element, SCError, SCResult};

use super::{Board, Move, Team, Piece, Vec2};

pub const ROUND_LIMIT: usize = 30;

/// The state of the game at a point in time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    /// The game board.
    board: Board,
    /// The ambers per team.
    ambers: HashMap<Team, usize>,
    /// The turn of the game.
    turn: usize,
    /// The most recent move.
    last_move: Option<Move>,
    /// The starting team.
    start_team: Option<Team>,
}

impl State {
    /// The game board.
    #[inline]
    pub fn board(&self) -> &Board { &self.board }

    /// The ambers per team.
    #[inline]
    pub fn ambers(&self) -> &HashMap<Team, usize> { &self.ambers }

    /// The turn of the game.
    #[inline]
    pub fn turn(&self) -> usize { self.turn }

    /// Fetches the round, i.e. `(turn + 1) / 2`.
    #[inline]
    pub fn round(&self) -> usize { (self.turn + 1) / 2 }

    /// The most recent move, if available.
    #[inline]
    pub fn last_move(&self) -> Option<Move> { self.last_move }

    /// The starting team, if available.
    #[inline]
    pub fn start_team(&self) -> Option<Team> { self.start_team }

    /// The current team, computed from the starting team and the turn.
    pub fn current_team(&self) -> Option<Team> {
        let start_team = self.start_team?;
        Some(if self.turn % 2 == 0 { start_team } else { start_team.opponent() })
    }

    // Partially translated from https://github.com/software-challenge/backend/blob/89407e5e2f76801ec8beb8f31412da218f5f70e5/plugin/src/main/kotlin/sc/plugin2022/GameState.kt

    /// Fetches the current team's pieces.
    pub fn current_pieces<'a>(&'a self) -> impl Iterator<Item=(Vec2, Piece)> + 'a {
        let team = self.current_team();
        self.board.pieces()
            .iter()
            .filter(move |&(_, piece)| Some(piece.team()) == team)
            .map(|(&pos, &piece)| (pos, piece))
    }

    /// Fetches the possible moves.
    pub fn possible_moves(&self) -> Vec<Move> {
        self.current_pieces()
            .flat_map(|(pos, piece)| piece.possible_directions()
                .map(move |delta| Move::new(pos, pos + delta))
                .filter(move |m| self.board.get(m.to()).map(|p| p.team()) != Some(piece.team())))
            .collect()
    }

    /// Checks whether the game is over.
    pub fn is_over(&self) -> bool {
        self.turn % 2 == 0 && (self.round() > ROUND_LIMIT || self.ambers.iter().any(|(_, &v)| v >= 2))
    }

    /// Performs the given move.
    pub fn perform(&mut self, m: Move) {
        self.board.perform(m);
    }

    /// Fetches the child state after the given move.
    pub fn child(self, m: Move) -> State {
        let mut child = self.clone();
        child.perform(m);
        child
    }
}

impl TryFrom<&Element> for State {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(State {
            board: elem.child_by_name("board")?.try_into()?,
            ambers: elem
                .child_by_name("ambers")?
                .childs_by_name("entry")
                .map(|e| {
                    let team = Team::from_str(e.child_by_name("team")?.content())?;
                    let piece = usize::from_str(e.child_by_name("int")?.content())?;
                    Ok((team, piece))
                })
                .collect::<SCResult<_>>()?,
            turn: elem.attribute("turn")?.parse()?,
            last_move: elem.child_by_name("lastMove").ok().and_then(|m| m.try_into().ok()),
            start_team: elem.child_by_name("startTeam").ok().and_then(|t| t.content().parse().ok()),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{util::Element, game::{Board, State, Team}, hashmap};

    #[test]
    fn test_parsing() {
        assert_eq!(State::try_from(&Element::from_str(r#"
            <state turn="3">
                <board>
                    <pieces></pieces>
                </board>
                <ambers>
                    <entry>
                        <team>ONE</team>
                        <int>1</int>
                    </entry>
                    <entry>
                        <team>TWO</team>
                        <int>0</int>
                    </entry>
                </ambers>
            </state>
        "#).unwrap()).unwrap(), State {
            board: Board::empty(),
            ambers: hashmap![
                Team::One => 1usize,
                Team::Two => 0usize
            ],
            last_move: None,
            start_team: None,
            turn: 3,
        });
    }
}
