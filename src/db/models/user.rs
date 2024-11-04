use deadpool_postgres::Client;
use serde::Serialize;
use tokio_postgres::NoTls;
use uuid::Uuid;

use super::Group;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub nickname: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub image: Option<String>,
}

impl TryFrom<crate::http::models::UserCreateRequest> for User {
    type Error = bcrypt::BcryptError;
    fn try_from(value: crate::http::models::UserCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            nickname: value.nickname,
            name: value.name,
            surname: value.surname,
            email: value.email,
            password: bcrypt::hash(value.password, bcrypt::DEFAULT_COST)?,
            image: value.image,
        })
    }
}

impl User {
    pub fn parse_row(row: &tokio_postgres::Row) -> User {
        User {
            id: row.get("id"),
            nickname: row.get("nickname"),
            name: row.get("name"),
            surname: row.get("surname"),
            email: row.get("email"),
            password: row.get("password"),
            image: row.get("image"),
        }
    }

    pub async fn get_by_email(
        email: &str,
        client: &Client<NoTls>,
    ) -> Result<Option<User>, tokio_postgres::Error> {
        let stmt = "SELECT * FROM users WHERE email = $1";
        let rows = client.query(stmt, &[&email]).await?;

        Ok(rows.first().map(User::parse_row))
    }

    pub async fn insert(&self, client: &Client<NoTls>) -> Result<(), tokio_postgres::Error> {
        let stmt =
            "INSERT into users(id,nickname,name,surname,email,password) VALUES($1,$2,$3,$4,$5,$6)";
        client
            .execute(
                stmt,
                &[
                    &self.id,
                    &self.nickname,
                    &self.name,
                    &self.surname,
                    &self.email,
                    &self.password,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn get_by_id(
        user_id: &uuid::Uuid,
        client: &Client<NoTls>,
    ) -> Result<Option<User>, tokio_postgres::Error> {
        let stmt = "SELECT * FROM users WHERE id = $1";
        let rows = client.query(stmt, &[user_id]).await?;

        Ok(rows.first().map(User::parse_row))
    }

    pub async fn get_group_ids_of_user(
        user_id: &uuid::Uuid,
        client: &Client<NoTls>,
    ) -> Result<Vec<uuid::Uuid>, tokio_postgres::Error> {
        let stmt = "SELECT group_id FROM users_groups WHERE user_id = $1";
        let rows = client.query(stmt, &[user_id]).await?;

        Ok(rows.iter().map(|row| row.get("group_id")).collect())
    }

    pub async fn get_unhandled_groups_requests(
        owner_id: &uuid::Uuid,
        client: &Client<NoTls>,
    ) -> Result<Vec<(User, Group)>, tokio_postgres::Error> {
        let stmt = "
        SELECT 
            u.id AS user_id, u.nickname, u.name, u.surname, u.email, u.image, u.password,
            g.id AS group_id, g.name, g.created_by_user
        FROM 
            user_group_join_requests ugjr
        JOIN 
            users u ON ugjr.user_id = u.id
        JOIN 
            groups g ON ugjr.group_id = g.id
        WHERE 
            g.created_by_user = $1
            AND ugjr.approved = 'unhandled'
    ";
        let rows = client.query(stmt, &[owner_id]).await?;

        let mut results = Vec::new();
        for row in rows {
            let user = User {
                id: row.get("user_id"),
                nickname: row.get("nickname"),
                name: row.get("name"),
                surname: row.get("surname"),
                email: row.get("email"),
                image: row.get("image"),
                password: row.get("password"),
            };

            let group = Group {
                id: row.get("group_id"),
                name: row.get("name"),
                created_by_user: row.get("created_by_user"),
            };

            results.push((user, group));
        }
        Ok(results)
    }
}
