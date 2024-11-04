use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db;

#[derive(Debug, Validate, Deserialize)]
pub struct UserCreateRequest {
    #[validate(length(min = 2))]
    pub nickname: String,
    #[validate(length(min = 2))]
    pub name: String,
    #[validate(length(min = 2))]
    pub surname: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    pub image: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub auth: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub nickname: String,
    pub name: String,
    pub surname: String,
    pub email: String,
}

impl From<db::models::User> for User {
    fn from(value: db::models::User) -> Self {
        Self {
            id: value.id,
            nickname: value.nickname,
            name: value.name,
            surname: value.surname,
            email: value.email,
        }
    }
}
