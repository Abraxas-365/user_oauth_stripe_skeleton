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
}
