use deadpool_postgres::Pool;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio_postgres::NoTls;

use crate::db::models;
use crate::messages::websocket::{AddItemsResponse, ApproveJoin, GroupId};
use crate::messages::{
    websocket::{DirectChatMessage, WebsocketMessage},
    workers::WorkerMessage,
};
use crate::{constants, db};

pub struct ActiveUser {
    pub groups: Vec<uuid::Uuid>,
    pub websocket_session: actix_ws::Session,
}

pub fn spawn_message_worker(
    database_sender: mpsc::UnboundedSender<WebsocketMessage>,
    pool: Pool<NoTls>,
) -> mpsc::UnboundedSender<WorkerMessage> {
    let (tx, mut rx) = mpsc::unbounded_channel::<WorkerMessage>();
    let mut user_state: HashMap<uuid::Uuid, ActiveUser> = HashMap::new();

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Some(websocket_message) = msg.pass_to_db() {
                database_sender
                    .send(websocket_message)
                    .expect(constants::FAILED_TO_SEND_MESSAGE_TO_STATE_WORKER);
            }

            match msg {
                WorkerMessage::WebsocketMessage(client_message) => {
                    match &client_message {
                        WebsocketMessage::DirectChatMessage(chat_message) => {
                            send_direct_chat_message(&mut user_state, chat_message).await
                        }
                        WebsocketMessage::GroupChatMessage(group_chat_message) => {
                            send_group_message(&mut user_state, group_chat_message).await;
                        }
                        WebsocketMessage::AddItemsRequest(add_items_request) => {
                            let add_items_response: AddItemsResponse =
                                add_items_request.clone().into();
                            database_sender
                                .send(WebsocketMessage::AddItemsResponse(
                                    add_items_response.clone(),
                                ))
                                .expect(constants::FAILED_TO_SEND_MESSAGE_TO_STATE_WORKER);
                            send_group_message(&mut user_state, &add_items_response).await;
                        }
                        WebsocketMessage::RemoveItems(remove_items) => {
                            send_group_message(&mut user_state, remove_items).await;
                        }

                        WebsocketMessage::JoinGroup(join_group) => {
                            if let Some(active_user) =
                                user_state.get_mut(&join_group.group_owner_id)
                            {
                                if send_message(&join_group, active_user).await.is_err() {
                                    user_state.remove(&join_group.sender_id);
                                }
                            }
                        }

                        WebsocketMessage::ApproveJoin(approve_join) => {
                            if !is_approver_valid(&pool, approve_join).await {
                                return;
                            };

                            if let Some(candidate_active_user) =
                                user_state.get_mut(&approve_join.candidate_id)
                            {
                                if send_message(&approve_join, candidate_active_user)
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
                        WebsocketMessage::CreateGroup(_) => {}
                        WebsocketMessage::AddItemsResponse(_) => {}
                    };
                }
                WorkerMessage::ClientShutdown(id) => {
                    user_state.remove(&id);
                    println!("Shutdown received for ID: {}", id);
                }
                WorkerMessage::ClientLogin(id, session) => {
                    insert_active_user_to_user_state(&mut user_state, id, session, &pool).await;
                }
            }
        }
    });

    tx
}

async fn send_message<T>(message: &T, user: &mut ActiveUser) -> Result<(), actix_ws::Closed>
where
    T: Sized + Serialize,
{
    user.websocket_session
        .text(serde_json::to_string(message).expect("Failed to serialize websocket"))
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
    direct_chat_message: &DirectChatMessage,
) {
    let user = if let Some(user) = user_state.get_mut(&direct_chat_message.reciever_id) {
        user
    } else {
        return;
    };

    if send_message(&direct_chat_message, user).await.is_err() {
        user_state.remove(&direct_chat_message.reciever_id);
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
