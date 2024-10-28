use deadpool_postgres::Pool;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, Duration};
use tokio_postgres::NoTls;

use crate::db::models::chat_message::DirectChatMessage;
use crate::messages::websocket::DirectChatMessageResponse;
use crate::messages::websocket::WebsocketMessageResponse;

pub struct Storage {
    pub direct_chat_message: Vec<DirectChatMessageResponse>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            direct_chat_message: Vec::new(),
        }
    }
}

pub fn spawn_database_worker(pool: Pool<NoTls>) -> mpsc::UnboundedSender<WebsocketMessageResponse> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let storage = Arc::new(Mutex::new(Storage::new()));
    let receiver_storage = storage.clone();

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                WebsocketMessageResponse::DirectChatMessage(chat_message) => {
                    let mut storage = receiver_storage.lock().await;
                    storage.direct_chat_message.push(chat_message);
                }
                _ => {
                    println!("unhandled message received")
                }
            }
        }
    });

    let storage = storage.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(5000)).await;
            let client_connection = if let Ok(client_connection) = pool.get().await {
                client_connection
            } else {
                println!("error obtaing client connection in worker state");
                continue;
            };
            let mut storage = storage.lock().await;

            if let Err(err) = DirectChatMessage::insert_bulk(
                &client_connection,
                storage
                    .direct_chat_message
                    .drain(..)
                    .map(DirectChatMessage::from)
                    .collect::<Vec<DirectChatMessage>>()
                    .as_slice(),
            )
            .await
            {
                println!("Error inserting direct chat messages: {:?}", err);
            }
        }
    });

    tx
}
