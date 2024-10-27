use core::error;

use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, errors, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub email: String,
    pub iat: usize,
    pub exp: usize,
}

impl From<(&uuid::Uuid, &str)> for Claims {
    fn from(value: (&uuid::Uuid, &str)) -> Self {
        Claims {
            sub: value.0.clone(),
            email: value.1.to_string(),
            iat: Utc::now().timestamp() as usize,
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        }
    }
}

fn get_secret_key_from_env() -> String {
    dotenv().ok();
    std::env::var("SECRET").expect("SECRET should be set")
}

pub fn create_jwt(user_id: &uuid::Uuid, email: &str) -> Result<String, errors::Error> {
    dotenv().ok();

    let claims = Claims::from((user_id, email));

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(get_secret_key_from_env().as_bytes());

    let token = encode(&header, &claims, &encoding_key)?;

    Ok(token)
}

pub fn decode_jwt(token: &str) -> Result<Claims, errors::Error> {
    dotenv().ok();
    let decoding_key = DecodingKey::from_secret(get_secret_key_from_env().as_bytes());
    let validation = Validation::default();

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}
