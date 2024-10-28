use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub auth: String,
}

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
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}
