use crate::util::{Element, SCError, SCResult};

pub struct State {
    // TODO
}

impl TryFrom<&Element> for State {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        // TODO
        Ok(State {})
    }
}
