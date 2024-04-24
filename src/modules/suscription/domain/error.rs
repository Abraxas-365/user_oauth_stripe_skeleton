use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SubscriptionError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Subscription not found")]
    NotFound,
}

impl ResponseError for SubscriptionError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SubscriptionError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json("Database error occurred")
            }
            SubscriptionError::NotFound => HttpResponse::NotFound().json("Subscription not found"),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SubscriptionError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SubscriptionError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
