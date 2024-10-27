use super::websocket::WebsocketMessage;

pub enum WorkerMessage {
    WebsocketMessage(WebsocketMessage),
    ClientShutdown(uuid::Uuid),
    ClientLogin(uuid::Uuid, actix_ws::Session),
}

impl std::fmt::Debug for WorkerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkerMessage::WebsocketMessage(message) => {
                write!(f, "WorkerMessage::WebsocketMessage({:?})", message)
            }
            WorkerMessage::ClientShutdown(uuid) => {
                write!(f, "WorkerMessage::ClientShutdown({})", uuid)
            }
            WorkerMessage::ClientLogin(uuid, _session) => {
                write!(f, "WorkerMessage::ClientLogin({}, Session)", uuid)
            }
        }
    }
}

impl WorkerMessage {
    pub fn pass_to_db(&self) -> Option<WebsocketMessage> {
        match self {
            WorkerMessage::WebsocketMessage(websocket_message) => match websocket_message {
                WebsocketMessage::AddItemsRequest(_) => None,
                _ => Some(websocket_message.clone()),
            },
            _ => None,
        }
    }
}
