use response::GroupChatMessageResponse;
use serde::{Deserialize, Serialize};

mod request;
mod response;

pub use request::AddItemRequest;
pub use request::AddItemsRequest;
pub use request::DirectChatMessageRequest;
pub use request::GroupChatMessageRequest;

pub use request::WebsocketMessageRequest;

pub use response::AddItemsResponse;
pub use response::CreateGroupResponse;
pub use response::DirectChatMessageResponse;
pub use response::WebsocketMessageResponse;

pub trait GroupId {
    fn get_group_id(&self) -> &uuid::Uuid;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApproveJoin {
    pub candidate_id: uuid::Uuid,
    pub sender_id: uuid::Uuid,
    pub approved: bool,
    pub group_id: uuid::Uuid,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JoinGroupRequest {
    pub sender_id: uuid::Uuid,
    pub group_owner_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RemoveItemsMessage {
    pub sender_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub items: Vec<uuid::Uuid>,
}

impl GroupId for RemoveItemsMessage {
    fn get_group_id(&self) -> &uuid::Uuid {
        &self.group_id
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WebsocketMessage {
    Request(WebsocketMessageRequest),
    Response(WebsocketMessageResponse),
}

impl From<DirectChatMessageResponse> for WebsocketMessage {
    fn from(value: DirectChatMessageResponse) -> Self {
        Self::Response(WebsocketMessageResponse::DirectChatMessage(value))
    }
}

impl From<GroupChatMessageResponse> for WebsocketMessage {
    fn from(value: GroupChatMessageResponse) -> Self {
        Self::Response(WebsocketMessageResponse::GroupChatMessage(value))
    }
}

impl From<AddItemsResponse> for WebsocketMessage {
    fn from(value: AddItemsResponse) -> Self {
        Self::Response(WebsocketMessageResponse::AddItems(value))
    }
}

impl From<RemoveItemsMessage> for WebsocketMessage {
    fn from(value: RemoveItemsMessage) -> Self {
        Self::Response(WebsocketMessageResponse::RemoveItems(value))
    }
}

impl From<JoinGroupRequest> for WebsocketMessage {
    fn from(value: JoinGroupRequest) -> Self {
        Self::Response(WebsocketMessageResponse::JoinGroup(value))
    }
}

impl From<ApproveJoin> for WebsocketMessage {
    fn from(value: ApproveJoin) -> Self {
        Self::Response(WebsocketMessageResponse::ApproveJoin(value))
    }
}

impl From<CreateGroupResponse> for WebsocketMessage {
    fn from(value: CreateGroupResponse) -> Self {
        Self::Response(WebsocketMessageResponse::CreateGroup(value))
    }
}
