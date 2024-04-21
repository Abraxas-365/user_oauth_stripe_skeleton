use jsonwebtoken::errors::Error as JwtError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use thiserror::Error;

use crate::modules::user::UserError;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Authorization failed")]
    AuthorizationFailed,

    #[error("Token request failed: {0}")]
    TokenRequestFailed(String),

    #[error("Invalid token error: {0}")]
    InvalidTokenError(String),

    #[error("JWT creation failed: {0}")]
    JwtCreationFailed(String),

    #[error("Invalid callback data provided")]
    InvalidCallbackData,

    #[error("OAuth2 request token error: {0}")]
    OAuth2RequestTokenError(String),

    #[error(transparent)]
    JwtError(#[from] JwtError),

    #[error("Network error: {0}")]
    NetworkError(#[from] ReqwestError), // From reqwest::Error

    #[error("JSON parsing error: {0}")]
    SerdeParseError(#[from] SerdeError), // From serde_json::Error
    //
    #[error("User Error")]
    UserError(#[from] UserError),
}
