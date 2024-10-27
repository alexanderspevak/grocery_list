use serde::{Deserialize, Serialize};
use uuid::uuid;

pub trait GroupId {
    fn get_group_id(&self) -> &uuid::Uuid;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddItemRequest {
    product_id: uuid::Uuid,
    group_id: uuid::Uuid,
    product_unit: String,
    quantity: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddItemResponse {
    id: uuid::Uuid,
    product_id: uuid::Uuid,
    group_id: uuid::Uuid,
    product_unit: String,
    quantity: Option<f32>,
}

impl From<AddItemRequest> for AddItemResponse {
    fn from(value: AddItemRequest) -> Self {
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
pub struct DirectChatMessage {
    pub sender_id: uuid::Uuid,
    pub reciever_id: uuid::Uuid,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GroupChatMessage {
    pub sender_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub message: String,
}
impl GroupId for GroupChatMessage {
    fn get_group_id(&self) -> &uuid::Uuid {
        &self.group_id
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddItemsRequest {
    pub sender_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub items: Vec<AddItemRequest>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddItemsResponse {
    pub sender_id: uuid::Uuid,
    pub items: Vec<AddItemResponse>,
    pub group_id: uuid::Uuid,
}

impl From<AddItemsRequest> for AddItemsResponse {
    fn from(value: AddItemsRequest) -> Self {
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
pub struct JoinGroup {
    pub sender_id: uuid::Uuid,
    pub group_owner_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApproveJoin {
    pub candidate_id: uuid::Uuid,
    pub sender_id: uuid::Uuid,
    pub approved: bool,
    pub group_id: uuid::Uuid,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateGroup {
    pub group_owner_id: uuid::Uuid,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WebsocketMessage {
    DirectChatMessage(DirectChatMessage),
    GroupChatMessage(GroupChatMessage),
    AddItemsRequest(AddItemsRequest),
    AddItemsResponse(AddItemsResponse),
    RemoveItems(RemoveItemsMessage),
    JoinGroup(JoinGroup),
    ApproveJoin(ApproveJoin),
    CreateGroup(CreateGroup),
}

impl WebsocketMessage {
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
            WebsocketMessage::GroupChatMessage(msg) => msg.sender_id,
            WebsocketMessage::AddItemsRequest(msg) => msg.sender_id,
            WebsocketMessage::RemoveItems(msg) => msg.sender_id,
            WebsocketMessage::JoinGroup(msg) => msg.sender_id,
            WebsocketMessage::ApproveJoin(msg) => msg.sender_id,
            WebsocketMessage::AddItemsResponse(msg) => msg.sender_id,
            WebsocketMessage::DirectChatMessage(msg) => msg.sender_id,
            WebsocketMessage::CreateGroup(msg) => msg.group_owner_id,
        }
    }
}
