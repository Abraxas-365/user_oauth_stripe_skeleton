use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Database error")]
    DatabaseError(#[from] SqlxError),

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json("Database error")
            }
            UserError::UserAlreadyExists => HttpResponse::Conflict().json("User already exists"),
            UserError::UserNotFound => HttpResponse::NotFound().json("User not found"),
            UserError::InvalidInput(msg) => {
                HttpResponse::BadRequest().json(format!("Invalid input: {}", msg))
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::UserAlreadyExists => StatusCode::CONFLICT,
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        }
    }
}
