use deadpool_postgres::Pool;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio_postgres::NoTls;

use crate::db;
use crate::db::models;
use crate::messages::websocket::{
    ApproveJoin, DirectChatMessageResponse, GroupId, WebsocketMessage, WebsocketMessageResponse,
};
use crate::messages::workers::WorkerMessageRequest;

pub struct ActiveUser {
    pub groups: Vec<uuid::Uuid>,
    pub websocket_session: actix_ws::Session,
}

pub fn spawn_message_worker(
    database_sender: mpsc::UnboundedSender<WebsocketMessageResponse>,
    pool: Pool<NoTls>,
) -> mpsc::UnboundedSender<WorkerMessageRequest> {
    let (tx, mut rx) = mpsc::unbounded_channel::<WorkerMessageRequest>();
    let mut user_state: HashMap<uuid::Uuid, ActiveUser> = HashMap::new();

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                WorkerMessageRequest::WebsocketMessage(websocket_message) => {
                    let websocket_response_message =
                        WebsocketMessageResponse::from(websocket_message);

                    match &websocket_response_message {
                        WebsocketMessageResponse::DirectChatMessage(chat_message) => {
                            send_direct_chat_message(&mut user_state, chat_message).await
                        }
                        WebsocketMessageResponse::GroupChatMessage(group_chat_message) => {
                            send_group_message(&mut user_state, group_chat_message).await;
                        }
                        WebsocketMessageResponse::AddItems(add_items_response) => {
                            send_group_message(&mut user_state, add_items_response).await;
                        }
                        WebsocketMessageResponse::RemoveItems(remove_items) => {
                            send_group_message(&mut user_state, remove_items).await;
                        }
                        WebsocketMessageResponse::JoinGroup(join_group) => {
                            if let Some(active_user) =
                                user_state.get_mut(&join_group.group_owner_id)
                            {
                                let websocket_message: WebsocketMessage = join_group.clone().into();
                                if send_message(&websocket_message, active_user).await.is_err() {
                                    user_state.remove(&join_group.sender_id);
                                }
                            }
                        }
                        WebsocketMessageResponse::ApproveJoin(approve_join) => {
                            if !is_approver_valid(&pool, approve_join).await {
                                return;
                            };

                            if let Some(candidate_active_user) =
                                user_state.get_mut(&approve_join.candidate_id)
                            {
                                if send_message(&approve_join.clone().into(), candidate_active_user)
                                    .await
                                    .is_err()
                                {
                                    user_state.remove(&approve_join.candidate_id);
                                    continue;
                                }

                                if approve_join.approved {
                                    candidate_active_user.groups.push(approve_join.group_id)
                                }
                            }
                        }
                        WebsocketMessageResponse::CreateGroup(create_group) => {
                            if let Some(active_user) =
                                user_state.get_mut(&create_group.group_owner_id)
                            {
                                if send_message(&create_group.clone().into(), active_user)
                                    .await
                                    .is_err()
                                {
                                    user_state.remove(&create_group.group_owner_id);
                                    continue;
                                }
                            }
                        }
                    }
                    database_sender
                        .send(websocket_response_message)
                        .expect("Failed to send message to database worker");
                }
                WorkerMessageRequest::ClientShutdown(id) => {
                    user_state.remove(&id);
                    println!("Shutdown received for ID: {}", id);
                }
                WorkerMessageRequest::ClientLogin(id, session) => {
                    insert_active_user_to_user_state(&mut user_state, id, session.clone(), &pool)
                        .await;
                }
            }
        }
    });

    tx
}

async fn send_message(
    message: &WebsocketMessage,
    user: &mut ActiveUser,
) -> Result<(), actix_ws::Closed> {
    user.websocket_session
        .text(serde_json::to_string(message).expect("Failed to serialize websocket message"))
        .await
}

async fn send_group_message<T>(
    user_state: &mut HashMap<uuid::Uuid, ActiveUser>,
    group_chat_message: &T,
) where
    T: Sized + Serialize + GroupId,
{
    let mut failures = vec![];
    for (id, session) in get_group_users(user_state, group_chat_message.get_group_id()) {
        let serialized_message = serde_json::to_string(group_chat_message)
            .expect("Failed to serialize group chat message");

        if session.text(serialized_message).await.is_err() {
            failures.push(id.clone());
        }
    }

    for failure in failures {
        println!("Failed to send websocket message. Session closing to ");
        user_state.remove(&failure);
    }
}

async fn send_direct_chat_message(
    user_state: &mut HashMap<uuid::Uuid, ActiveUser>,
    direct_chat_message: &DirectChatMessageResponse,
) {
    let user = if let Some(user) = user_state.get_mut(&direct_chat_message.receiver_id) {
        user
    } else {
        return;
    };

    if send_message(&direct_chat_message.clone().into(), user)
        .await
        .is_err()
    {
        user_state.remove(&direct_chat_message.receiver_id);
    }
}

fn get_group_users<'a>(
    user_state: &'a mut HashMap<uuid::Uuid, ActiveUser>,
    group_id: &uuid::Uuid,
) -> Vec<(&'a uuid::Uuid, &'a mut actix_ws::Session)> {
    let mut sessions = vec![];
    for (id, active_user) in user_state {
        if active_user.groups.contains(group_id) {
            sessions.push((id, &mut active_user.websocket_session));
        }
    }

    sessions
}

async fn insert_active_user_to_user_state(
    user_state: &mut HashMap<uuid::Uuid, ActiveUser>,
    id: uuid::Uuid,
    session: actix_ws::Session,
    pool: &Pool<NoTls>,
) {
    let client_connection = if let Ok(client_connection) = pool.get().await {
        client_connection
    } else {
        println!("error obtaing client connection in worker state");
        return;
    };

    let group_ids = if let Ok(group_ids) =
        db::models::user::User::get_group_ids(&id, &client_connection).await
    {
        group_ids
    } else {
        println!("error obtaing group ids in worker state");
        return;
    };

    user_state.insert(
        id,
        ActiveUser {
            groups: group_ids,
            websocket_session: session,
        },
    );
}

async fn is_approver_valid(pool: &Pool<NoTls>, approve_join_message: &ApproveJoin) -> bool {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(error) => {
            println!(
                "Error obtaining database client in message worker: {}",
                error
            );
            return false;
        }
    };
    let maybe_group = match models::Group::get_by_id(&approve_join_message.group_id, &client).await
    {
        Ok(maybe_group) => maybe_group,
        Err(error) => {
            println!(
                "Error obtaining database client in message worker: {}",
                error
            );
            return false;
        }
    };

    let db_group = if let Some(group) = maybe_group {
        group
    } else {
        println!("Group from ApproveJoin message not found");
        return false;
    };

    if db_group.created_by_user != approve_join_message.sender_id {
        println!("Invalid approve join request");
        true
    } else {
        false
    }
}
