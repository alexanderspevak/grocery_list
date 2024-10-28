use deadpool_postgres::Client;
use serde::Serialize;
use tokio_postgres::NoTls;

use crate::messages::websocket::CreateGroupResponse;

#[derive(Debug, Serialize)]
pub struct Group {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_by_user: uuid::Uuid,
}

impl From<CreateGroupResponse> for Group {
    fn from(value: CreateGroupResponse) -> Self {
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

    pub async fn insert_bulk(
        client: &Client<NoTls>,
        groups: &[Group],
    ) -> Result<(), tokio_postgres::Error> {
        let mut query = String::from("INSERT INTO groups(id, name, created_by_user) VALUES ");
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

        for (i, group) in groups.iter().enumerate() {
            if i > 0 {
                query.push_str(", ");
            }
            let param_base = i * 3;
            query.push_str(&format!(
                "(${}, ${}, ${})",
                param_base + 1,
                param_base + 2,
                param_base + 3
            ));

            params.push(&group.id);
            params.push(&group.name);
            params.push(&group.created_by_user);
        }

        client.execute(query.as_str(), &params[..]).await?;
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
