use std::{collections::HashMap, str::FromStr};

use crate::util::{Element, SCError, SCResult};

use super::{Board, Move, Team};

/// The state of the game at a point in time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    /// The game board.
    board: Board,
    /// The most recent move.
    last_move: Move,
    /// The ambers per team.
    ambers: HashMap<Team, usize>,
}

impl TryFrom<&Element> for State {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(State {
            board: elem.child_by_name("board")?.try_into()?,
            last_move: elem.child_by_name("lastMove")?.try_into()?,
            ambers: elem
                .child_by_name("ambers")?
                .childs_by_name("entry")
                .map(|e| {
                    let team = Team::from_str(e.child_by_name("team")?.content())?;
                    let piece = usize::from_str(e.child_by_name("int")?.content())?;
                    Ok((team, piece))
                })
                .collect::<SCResult<_>>()?,
        })
    }
}
