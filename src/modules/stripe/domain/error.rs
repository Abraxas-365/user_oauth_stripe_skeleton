use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use sqlx::Error as SqlxError;
use stripe::StripeError;
use thiserror::Error;

use crate::modules::user::UserError;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Payment not found")]
    NotFound,

    #[error("Invalid payment status: {0}")]
    InvalidPaymentStatus(String),

    #[error("Authorization failed")]
    AuthorizationFailed,

    #[error("User Error: {0}")]
    UserError(#[from] UserError),

    #[error("Stripe error: {0}")]
    StripeError(#[from] StripeError),
}

impl ResponseError for PaymentError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PaymentError::StripeError(_) => {
                HttpResponse::InternalServerError().json("Stripe error")
            }
            PaymentError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json("Database error occurred")
            }
            PaymentError::NotFound => HttpResponse::NotFound().json("Payment not found"),
            PaymentError::AuthorizationFailed => {
                HttpResponse::Unauthorized().json("Authorization failed")
            }
            PaymentError::InvalidPaymentStatus(msg) => {
                HttpResponse::BadRequest().json(format!("Invalid payment status: {}", msg))
            }
            PaymentError::UserError(usr_err) => usr_err.error_response(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            PaymentError::StripeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PaymentError::AuthorizationFailed => StatusCode::UNAUTHORIZED,
            PaymentError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PaymentError::NotFound => StatusCode::NOT_FOUND,
            PaymentError::InvalidPaymentStatus(_) => StatusCode::BAD_REQUEST,
            PaymentError::UserError(usr_err) => usr_err.status_code(),
        }
    }
}
