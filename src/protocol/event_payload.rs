use std::fmt;

use crate::util::{Element, SCResult, SCError};

/// The actual data of a message from the server.
#[derive(Debug, Clone)]
pub enum EventPayload {
    /// A welcome message by the server.
    Welcome,
    /// A game state.
    /// TODO: Add the actual state.
    Memento,
    /// A request by the server to perform a move.
    MoveRequest,
}

impl fmt::Display for EventPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Welcome => write!(f, "Welcome"),
            Self::Memento => write!(f, "Memento"),
            Self::MoveRequest => write!(f, "MoveRequest"),
        }
    }
}

impl TryFrom<&Element> for EventPayload {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        match elem.attribute("class")? {
            "welcomeMessage" => Ok(Self::Welcome),
            "memento" => Ok(Self::Memento),
            "moveRequest" => Ok(Self::MoveRequest),
            _ => Err(SCError::UnknownElement(elem.clone())),
        }
    }
}
