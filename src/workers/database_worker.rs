use actix_ws::Closed;
use deadpool_postgres::Pool;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, Duration};
use tokio_postgres::NoTls;

use crate::db;
use crate::messages::{
    websocket::{DirectChatMessage, WebsocketMessage},
    workers::WorkerMessage,
};

pub struct Storage {
    pub chat_messages: Vec<DirectChatMessage>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            chat_messages: Vec::new(),
        }
    }
}

pub fn spawn_database_worker(pool: Pool<NoTls>) -> mpsc::UnboundedSender<WebsocketMessage> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let storage = Arc::new(Mutex::new(Storage::new()));
    let receiver_storage = storage.clone();

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                WebsocketMessage::DirectChatMessage(chat_message) => {
                    let mut storage = receiver_storage.lock().await;
                    storage.chat_messages.push(chat_message);
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
            let storage = storage.lock().await;
        }
    });

    tx
}
