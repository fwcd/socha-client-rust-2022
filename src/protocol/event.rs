use crate::util::{Element, SCResult, SCError};

/// A message from the server.
pub enum Event {
    Joined,
}

impl TryFrom<Element> for Event {
    type Error = SCError;

    fn try_from(elem: Element) -> SCResult<Self> {
        match elem.name() {
            "joined" => Ok(Self::Joined),
            name => Err(SCError::UnknownTag(name.to_owned())),
        }
    }
}
