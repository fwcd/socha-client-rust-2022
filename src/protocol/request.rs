use crate::util::Element;

const GAME_TYPE: &str = "swc_2022_ostseeschach";

/// A message from the client.
#[derive(Debug, Clone)]
pub enum Request {
    /// Joins an abitrary open game.
    Join,
    /// Joins the room with the given id.
    JoinRoom { room_id: String },
    /// Joins a reserved place in a planned match with
    /// a reservation code.
    JoinPrepared { reservation_code: String },
}

impl From<Request> for Element {
    fn from(req: Request) -> Self {
        match req {
            Request::Join => Element::new("join").attribute("gameType", GAME_TYPE).build(),
            Request::JoinRoom { room_id } => Element::new("joinRoom").attribute("roomId", room_id).build(),
            Request::JoinPrepared { reservation_code } => Element::new("joinPrepared").attribute("reservationCode", reservation_code).build(),
        }
    }
}
