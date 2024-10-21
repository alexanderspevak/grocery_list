use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatMessage {
    pub sender_id: String,
    pub reciever_id: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupChatMessage {
    pub sender_id: String,
    pub group_id: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddItemsMessage {
    pub sender_id: String,
    pub group_id: String,
    pub items: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RemoveItemsMessage {
    pub sender_id: String,
    pub group_id: String,
    pub items: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct JoinGroup {
    pub sender_id: String,
    pub group_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApproveJoin {
    pub approved: bool,
    pub group_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateGroup {
    pub group_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum WebsocketMessage {
    ChatMessage(ChatMessage),
    GroupChatMessage(GroupChatMessage),
    AddItems(AddItemsMessage),
    RemoveItems(RemoveItemsMessage),
    JoinGroup(JoinGroup),
    ApproveJoin(ApproveJoin),
}
