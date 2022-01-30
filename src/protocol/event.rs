use crate::util::{Element, SCResult, SCError};

/// A message from the server.
pub enum Event {
    /// Notifies the client about a successful join.
    Joined { room_id: String },
}

impl TryFrom<Element> for Event {
    type Error = SCError;

    fn try_from(elem: Element) -> SCResult<Self> {
        match elem.name() {
            "joined" => Ok(Self::Joined {
                room_id: elem.attribute("roomId")?.to_owned()
            }),
            name => Err(SCError::UnknownTag(name.to_owned())),
        }
    }
}
