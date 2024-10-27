use deadpool_postgres::Client;
use serde::Serialize;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::messages::websocket::CreateGroup;

#[derive(Debug, Serialize)]
pub struct Group {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_by_user: uuid::Uuid,
}

impl From<CreateGroup> for Group {
    fn from(value: CreateGroup) -> Self {
        Self {
            created_by_user: value.group_owner_id,
            name: value.name,
            id: uuid::Uuid::new_v4(),
        }
    }
}

impl Group {
    pub fn parse_row(row: &tokio_postgres::Row) -> Group {
        Group {
            id: row.get("id"),
            name: row.get("name"),
            created_by_user: row.get("created_by_user"),
        }
    }

    pub async fn insert(&self, client: &Client<NoTls>) -> Result<(), tokio_postgres::Error> {
        let stmt = "INSERT into groups(id,name,created_by_user) VALUES($1,$2,$3)";
        client
            .execute(stmt, &[&self.id, &self.name, &self.created_by_user])
            .await?;
        Ok(())
    }

    pub async fn get_by_id(
        group_id: &uuid::Uuid,
        client: &Client<NoTls>,
    ) -> Result<Option<Group>, tokio_postgres::Error> {
        let stmt = "SELECT * FROM groups WHERE id = $1";
        let rows = client.query(stmt, &[group_id]).await?;

        Ok(rows.first().map(Group::parse_row))
    }
}
