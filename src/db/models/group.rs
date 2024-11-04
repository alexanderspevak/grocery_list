use deadpool_postgres::Client;
use serde::Serialize;
use tokio_postgres::NoTls;

use crate::http::models::CreateGroupRequest;

use super::user::User;

#[derive(Debug, Serialize)]
pub struct Group {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_by_user: uuid::Uuid,
}

impl From<CreateGroupRequest> for Group {
    fn from(value: CreateGroupRequest) -> Self {
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

    pub async fn insert(&self, client: &mut Client<NoTls>) -> Result<(), tokio_postgres::Error> {
        let transaction = client.transaction().await?;
        let stmt = "INSERT into groups(id,name,created_by_user) VALUES($1,$2,$3)";
        transaction
            .execute(stmt, &[&self.id, &self.name, &self.created_by_user])
            .await?;

        let user_group_id = uuid::Uuid::new_v4();
        let stmt = "INSERT into users_groups(id,group_id,user_id) VALUES($1,$2,$3)";

        transaction
            .execute(stmt, &[&user_group_id, &self.id, &self.created_by_user])
            .await?;
        transaction.commit().await?;
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

    pub async fn get_users(
        group_id: &uuid::Uuid,
        client: &Client<NoTls>,
    ) -> Result<Vec<User>, tokio_postgres::Error> {
        let stmt = "SELECT u.* FROM users u
             JOIN users_groups ug ON u.id = ug.user_id
             WHERE ug.group_id = $1";

        Ok(client
            .query(stmt, &[group_id])
            .await?
            .iter()
            .map(User::parse_row)
            .collect())
    }

    pub async fn create_group_request(
        group_id: &uuid::Uuid,
        user_id: &uuid::Uuid,
        client: &Client<NoTls>,
    ) -> Result<(), tokio_postgres::Error> {
        let stmt = "INSERT into user_group_join_requests(group_id,user_id) VALUES($1,$2)";

        client.execute(stmt, &[group_id, user_id]).await?;

        Ok(())
    }
}
