mod group;
mod user;

use super::jwt::{decode_jwt, Claims};
use actix_web::Result;

pub use group::group_routes;
pub use user::user_routes;

use crate::http::error::HttpError;

fn get_auth_claims(req: &actix_web::HttpRequest) -> Result<Claims, HttpError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.strip_prefix("Bearer "))
        .ok_or(HttpError::Unauthorized)?;
    Ok(decode_jwt(token)?)
}
