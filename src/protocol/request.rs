use crate::util::Element;

/// A message from the client.
enum Request {
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
            Request::Join => Element::new("join").build(),
            Request::JoinRoom { room_id } => Element::new("joinRoom").attribute("roomId", room_id).build(),
            Request::JoinPrepared { reservation_code } => Element::new("joinPrepared").attribute("reservationCode", reservation_code).build(),
        }
    }
}
