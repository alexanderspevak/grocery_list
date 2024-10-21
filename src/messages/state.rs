use std::fmt;

use actix_ws::Session;

use super::websocket::WebsocketMessage;

pub struct StateMessage {
    pub websocket_sender: Session,
    pub message: WebsocketMessage,
}

impl fmt::Debug for StateMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.message)
    }
}

impl From<(Session, WebsocketMessage)> for StateMessage {
    fn from(value: (Session, WebsocketMessage)) -> StateMessage {
        StateMessage {
            websocket_sender: value.0,
            message: value.1,
        }
    }
}
