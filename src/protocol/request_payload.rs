use crate::{util::Element, game::Move};

/// The data of a room message to the server.
#[derive(Debug, Clone)]
pub enum RequestPayload {
    /// A move to be performed.
    Move(Move),
}

impl From<RequestPayload> for Element {
    fn from(payload: RequestPayload) -> Self {
        let mut element = Element::new("data");

        element = match payload {
            RequestPayload::Move(m) => element
                .attribute("class", "move")
                .child(m),
        };

        element.build()
    }
}
