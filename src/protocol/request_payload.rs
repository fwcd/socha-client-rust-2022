use crate::util::Element;

/// The actual data of a message to the server.
#[derive(Debug, Clone)]
pub enum RequestPayload {
    MoveRequest,
}

impl From<RequestPayload> for Element {
    fn from(payload: RequestPayload) -> Self {
        let mut element = Element::new("data");

        element = match payload {
            RequestPayload::MoveRequest => element
                .attribute("class", "moveRequest"),
        };

        element.build()
    }
}
