use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Payment not found")]
    NotFound,

    #[error("Invalid payment status: {0}")]
    InvalidPaymentStatus(String),
}
