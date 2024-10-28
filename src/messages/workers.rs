use super::websocket::{WebsocketMessage, WebsocketMessageRequest};

pub enum WorkerMessage {
    WebsocketMessage(WebsocketMessage),
    ClientShutdown(uuid::Uuid),
    ClientLogin(uuid::Uuid, actix_ws::Session),
}

pub enum WorkerMessageRequest {
    WebsocketMessage(WebsocketMessageRequest),
    ClientShutdown(uuid::Uuid),
    ClientLogin(uuid::Uuid, actix_ws::Session),
}

impl std::fmt::Debug for WorkerMessageRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkerMessageRequest::WebsocketMessage(message) => {
                write!(f, "WorkerMessage::WebsocketMessage({:?})", message)
            }
            WorkerMessageRequest::ClientShutdown(uuid) => {
                write!(f, "WorkerMessage::ClientShutdown({})", uuid)
            }
            WorkerMessageRequest::ClientLogin(uuid, _session) => {
                write!(f, "WorkerMessage::ClientLogin({}, Session)", uuid)
            }
        }
    }
}
