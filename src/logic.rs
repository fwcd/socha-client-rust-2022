use log::info;

use crate::{client::SCClientDelegate, game::{Move, Team, State, Vec2}};

/// An empty game logic structure that
/// implements the client delegate trait
/// and thus is responsible e.g. for picking
/// a move when requested.
pub struct OwnGameLogic;

impl SCClientDelegate for OwnGameLogic {
    fn request_move(&mut self, state: &State, my_team: Team) -> Move {
        info!("Requested move");

        // TODO
        Move::new(Vec2::new(0, 0), Vec2::new(1, 1))
    }
}
