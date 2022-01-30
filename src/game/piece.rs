use crate::util::{Element, SCError, SCResult};

pub struct Team {
    // TODO
}

impl TryFrom<&Element> for Team {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        // TODO
        Ok(Team {})
    }
}
