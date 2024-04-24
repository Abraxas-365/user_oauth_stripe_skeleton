use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TierError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Tier not found")]
    NotFound,

    #[error("Invalid tier data: {0}")]
    InvalidTierData(String),
}

impl ResponseError for TierError {
    fn error_response(&self) -> HttpResponse {
        match self {
            TierError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json("Database error occurred")
            }
            TierError::NotFound => HttpResponse::NotFound().json("Tier not found"),
            TierError::InvalidTierData(msg) => {
                HttpResponse::BadRequest().json(format!("Invalid tier data: {}", msg))
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TierError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TierError::NotFound => StatusCode::NOT_FOUND,
            TierError::InvalidTierData(_) => StatusCode::BAD_REQUEST,
        }
    }
}
