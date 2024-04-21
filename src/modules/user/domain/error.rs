use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("database error")]
    DatabaseError(#[from] SqlxError),

    #[error("user already exists")]
    UserAlreadyExists,

    #[error("user not found")]
    UserNotFound,

    #[error("invalid input: {0}")]
    InvalidInput(String),
}
