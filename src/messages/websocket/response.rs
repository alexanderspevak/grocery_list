use serde::{Deserialize, Serialize};

use crate::db::models::chat_message::DirectChatMessage;

use super::request::CreateGroupRequest;
use super::DirectChatMessageRequest;
use super::GroupChatMessageRequest;
use super::GroupId;
use super::WebsocketMessageRequest;
use chrono::Utc;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DirectChatMessageResponse {
    pub id: uuid::Uuid,
    pub sender_id: uuid::Uuid,
    pub receiver_id: uuid::Uuid,
    pub read: bool,
    pub message: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<DirectChatMessageRequest> for DirectChatMessageResponse {
    fn from(value: DirectChatMessageRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            sender_id: value.sender_id,
            receiver_id: value.receiver_id,
            read: false,
            message: value.message.clone(),
            created_at: Utc::now(),
        }
    }
}

impl From<DirectChatMessage> for DirectChatMessageResponse {
    fn from(value: DirectChatMessage) -> Self {
        Self {
            id: value.id,
            message: value.message,
            sender_id: value.sender_id,
            receiver_id: value.receiver_id,
            read: value.read,
            created_at: value.created_at,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GroupChatMessageResponse {
    pub id: uuid::Uuid,
    pub sender_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub message: String,
}

impl GroupId for GroupChatMessageResponse {
    fn get_group_id(&self) -> &uuid::Uuid {
        &self.group_id
    }
}

impl From<GroupChatMessageRequest> for GroupChatMessageResponse {
    fn from(value: GroupChatMessageRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            sender_id: value.sender_id,
            group_id: value.group_id,
            message: value.message.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddItemResponse {
    id: uuid::Uuid,
    product_id: uuid::Uuid,
    group_id: uuid::Uuid,
    product_unit: String,
    quantity: Option<f32>,
}

impl From<super::AddItemRequest> for AddItemResponse {
    fn from(value: super::AddItemRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            product_id: value.product_id,
            group_id: value.group_id,
            product_unit: value.product_unit,
            quantity: value.quantity,
        }
    }
}

impl GroupId for AddItemResponse {
    fn get_group_id(&self) -> &uuid::Uuid {
        &self.group_id
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddItemsResponse {
    pub sender_id: uuid::Uuid,
    pub items: Vec<AddItemResponse>,
    pub group_id: uuid::Uuid,
}

impl From<super::AddItemsRequest> for AddItemsResponse {
    fn from(value: super::AddItemsRequest) -> Self {
        Self {
            sender_id: value.sender_id,
            group_id: value.group_id,
            items: value.items.into_iter().map(AddItemResponse::from).collect(),
        }
    }
}

impl GroupId for AddItemsResponse {
    fn get_group_id(&self) -> &uuid::Uuid {
        &self.group_id
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateGroupResponse {
    pub group_id: uuid::Uuid,
    pub group_owner_id: uuid::Uuid,
    pub name: String,
}

impl From<CreateGroupRequest> for CreateGroupResponse {
    fn from(value: CreateGroupRequest) -> Self {
        Self {
            group_id: uuid::Uuid::new_v4(),
            group_owner_id: value.group_owner_id,
            name: value.name,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WebsocketMessageResponse {
    DirectChatMessage(DirectChatMessageResponse),
    GroupChatMessage(GroupChatMessageResponse),
    AddItems(AddItemsResponse),
    RemoveItems(super::RemoveItemsMessage),
    JoinGroup(super::JoinGroupRequest),
    ApproveJoin(super::ApproveJoin),
    CreateGroup(CreateGroupResponse),
}

impl From<WebsocketMessageRequest> for WebsocketMessageResponse {
    fn from(value: WebsocketMessageRequest) -> Self {
        match value {
            WebsocketMessageRequest::DirectChatMessage(msg) => {
                WebsocketMessageResponse::DirectChatMessage(DirectChatMessageResponse::from(msg))
            }
            WebsocketMessageRequest::GroupChatMessage(msg) => {
                WebsocketMessageResponse::GroupChatMessage(GroupChatMessageResponse::from(msg))
            }
            WebsocketMessageRequest::AddItemsRequest(msg) => {
                WebsocketMessageResponse::AddItems(AddItemsResponse::from(msg))
            }
            WebsocketMessageRequest::RemoveItems(msg) => WebsocketMessageResponse::RemoveItems(msg),
            WebsocketMessageRequest::JoinGroup(msg) => WebsocketMessageResponse::JoinGroup(msg),
            WebsocketMessageRequest::ApproveJoin(msg) => WebsocketMessageResponse::ApproveJoin(msg),
            WebsocketMessageRequest::CreateGroup(msg) => {
                WebsocketMessageResponse::CreateGroup(CreateGroupResponse::from(msg))
            }
        }
    }
}
