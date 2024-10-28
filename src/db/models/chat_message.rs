use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error, Row};
use uuid::Uuid;

use crate::messages::websocket::DirectChatMessageResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectChatMessage {
    pub id: Uuid,
    pub message: String,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub read: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<DirectChatMessageResponse> for DirectChatMessage {
    fn from(value: DirectChatMessageResponse) -> Self {
        Self {
            id: value.id,
            message: value.message,
            sender_id: value.sender_id,
            receiver_id: value.receiver_id,
            read: false,
            created_at: value.created_at,
        }
    }
}

impl DirectChatMessage {
    fn from_row(row: Row) -> Self {
        DirectChatMessage {
            id: row.get("id"),
            message: row.get("message"),
            sender_id: row.get("sender"),
            receiver_id: row.get("receiver"),
            read: row.get("read"),
            created_at: row.get("created_at"),
        }
    }
}

impl DirectChatMessage {
    pub async fn insert(&self, client: &Client) -> Result<(), Error> {
        let query = "
            INSERT INTO messages (id, message, sender, receiver, read, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)";

        client
            .execute(
                query,
                &[
                    &self.id,
                    &self.message,
                    &self.sender_id,
                    &self.receiver_id,
                    &self.read,
                    &self.created_at,
                ],
            )
            .await?;

        Ok(())
    }

    pub async fn get_by_id(
        client: &Client,
        message_id: Uuid,
    ) -> Result<Option<DirectChatMessage>, Error> {
        let query = "SELECT * FROM messages WHERE id = $1";

        if let Some(row) = client.query_opt(query, &[&message_id]).await? {
            Ok(Some(DirectChatMessage::from_row(row)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_paginated(
        client: &Client,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DirectChatMessage>, Error> {
        let query = "
            SELECT * FROM messages
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2";

        let rows = client.query(query, &[&limit, &offset]).await?;

        let messages = rows.into_iter().map(DirectChatMessage::from_row).collect();

        Ok(messages)
    }

    pub async fn insert_bulk(client: &Client, messages: &[DirectChatMessage]) -> Result<(), Error> {
        if messages.is_empty() {
            return Ok(()); // Nothing to insert
        }

        let mut query = String::from(
            "INSERT INTO messages (id, message, sender, receiver, read, created_at) VALUES ",
        );
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

        for (i, message) in messages.iter().enumerate() {
            if i > 0 {
                query.push_str(", ");
            }
            let base = i * 6;
            query.push_str(&format!(
                "(${}, ${}, ${}, ${}, ${}, ${})",
                base + 1,
                base + 2,
                base + 3,
                base + 4,
                base + 5,
                base + 6
            ));
            params.push(&message.id);
            params.push(&message.message);
            params.push(&message.sender_id);
            params.push(&message.receiver_id);
            params.push(&message.read);
            params.push(&message.created_at);
        }

        client.execute(query.as_str(), &params[..]).await?;
        Ok(())
    }
}
