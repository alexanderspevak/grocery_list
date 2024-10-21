use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use std::fmt;

#[derive(Debug)]
pub enum HttpError {
    BadRequest(String),
    ServerError(String),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::BadRequest(message) => write!(f, "Bad Request: {}", message),
            HttpError::ServerError(message) => write!(f, "Internal Server Error: {}", message),
        }
    }
}

impl error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            HttpError::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<validator::ValidationErrors> for HttpError {
    fn from(value: validator::ValidationErrors) -> HttpError {
        HttpError::BadRequest(value.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for HttpError {
    fn from(value: jsonwebtoken::errors::Error) -> HttpError {
        HttpError::BadRequest(value.to_string())
    }
}

impl From<serde_json::Error> for HttpError {
    fn from(_: serde_json::Error) -> HttpError {
        HttpError::ServerError("Serialization error".to_string())
    }
}

impl From<deadpool_postgres::PoolError> for HttpError {
    fn from(value: deadpool_postgres::PoolError) -> HttpError {
        HttpError::ServerError(value.to_string())
    }
}

impl From<bcrypt::BcryptError> for HttpError {
    fn from(value: bcrypt::BcryptError) -> HttpError {
        HttpError::BadRequest(value.to_string())
    }
}

impl From<tokio_postgres::Error> for HttpError {
    fn from(value: tokio_postgres::Error) -> HttpError {
        HttpError::ServerError(value.to_string())
    }
}
