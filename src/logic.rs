use log::info;

use crate::{client::SCClientDelegate, game::{Move, Team, State, Coords}};

/// An empty game logic structure that
/// implements the client delegate trait
/// and thus is responsible e.g. for picking
/// a move when requested.
pub struct OwnGameLogic;

impl SCClientDelegate for OwnGameLogic {
    fn request_move(&mut self, state: &State, my_team: Team) -> Move {
        info!("Requested move");

        // TODO
        Move::new(Coords::new(0, 0), Coords::new(1, 1))
    }
}
