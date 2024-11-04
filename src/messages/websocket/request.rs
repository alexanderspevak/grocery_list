use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddItemRequest {
    pub product_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub product_unit: String,
    pub quantity: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DirectChatMessageRequest {
    pub sender_id: uuid::Uuid,
    pub receiver_id: uuid::Uuid,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GroupChatMessageRequest {
    pub sender_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddItemsRequest {
    pub sender_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub items: Vec<AddItemRequest>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WebsocketMessageRequest {
    DirectChatMessage(DirectChatMessageRequest),
    GroupChatMessage(GroupChatMessageRequest),
    AddItemsRequest(AddItemsRequest),
    RemoveItems(super::RemoveItemsMessage),
    JoinGroup(super::JoinGroupRequest),
    ApproveJoin(super::ApproveJoin),
}

impl WebsocketMessageRequest {
    //     pub fn group_id(&self) -> Option<uuid::Uuid> {
    //         match self {
    //             WebsocketMessage::GroupChatMessage(msg) => Some(msg.group_id),
    //             WebsocketMessage::(msg) => Some(msg.group_id),
    //             WebsocketMessage::RemoveItems(msg) => Some(msg.group_id),
    //             WebsocketMessage::JoinGroup(msg) => Some(msg.group_id),
    //             WebsocketMessage::ApproveJoin(msg) => Some(msg.group_id),
    //             WebsocketMessage::DirectChatMessage(_) => None,
    //         }
    //     }

    pub fn sender_id(&self) -> uuid::Uuid {
        match self {
            WebsocketMessageRequest::GroupChatMessage(msg) => msg.sender_id,
            WebsocketMessageRequest::AddItemsRequest(msg) => msg.sender_id,
            WebsocketMessageRequest::RemoveItems(msg) => msg.sender_id,
            WebsocketMessageRequest::JoinGroup(msg) => msg.sender_id,
            WebsocketMessageRequest::ApproveJoin(msg) => msg.sender_id,
            WebsocketMessageRequest::DirectChatMessage(msg) => msg.sender_id,
        }
    }
}
