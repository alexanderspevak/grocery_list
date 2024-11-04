use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use std::fmt;
use tokio_postgres::error::SqlState;

#[derive(Debug)]
pub enum HttpError {
    BadRequest(String),
    NotFound,
    ServerError(String),
    Unauthorized,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::BadRequest(message) => write!(f, "Bad Request: {}", message),
            HttpError::Unauthorized => write!(f, "Unauthorized"),
            HttpError::NotFound => write!(f, "Not Found"),

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
            HttpError::NotFound => StatusCode::NOT_FOUND,
            HttpError::Unauthorized => StatusCode::UNAUTHORIZED,
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
        if let Some(db_error) = value.as_db_error() {
            if db_error.code() == &SqlState::UNIQUE_VIOLATION {
                return HttpError::BadRequest(value.to_string());
            }
        }
        HttpError::ServerError(value.to_string())
    }
}
