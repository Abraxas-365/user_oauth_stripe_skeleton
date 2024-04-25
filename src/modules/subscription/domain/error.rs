use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SubscriptionError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Subscription not found")]
    SubscriptionNotFound,
}
