use crate::util::{Element, SCResult, SCError};

/// A message from the server.
pub enum Event {
    /// Notifies the client that they successfully joined a room.
    Joined { room_id: String },
    /// Notifies the client that they left a room.
    Left { room_id: String },
}

impl TryFrom<Element> for Event {
    type Error = SCError;

    fn try_from(elem: Element) -> SCResult<Self> {
        match elem.name() {
            "joined" => Ok(Self::Joined { room_id: elem.attribute("roomId")?.to_owned() }),
            "left" => Ok(Self::Left { room_id: elem.attribute("roomId")?.to_owned() }),
            _ => Err(SCError::UnknownTag(elem)),
        }
    }
}
