use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
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

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::AuthorizationFailed => {
                HttpResponse::Unauthorized().json("Authorization failed")
            }
            AuthError::TokenRequestFailed(details)
            | AuthError::InvalidTokenError(details)
            | AuthError::JwtCreationFailed(details)
            | AuthError::OAuth2RequestTokenError(details) => {
                HttpResponse::BadRequest().json(details)
            }
            AuthError::InvalidCallbackData => {
                HttpResponse::BadRequest().json("Invalid callback data provided")
            }
            AuthError::JwtError(_) | AuthError::NetworkError(_) | AuthError::SerdeParseError(_) => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
            AuthError::UserError(usr_err) => usr_err.error_response(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::UserError(usr_err) => usr_err.status_code(),
            AuthError::AuthorizationFailed => StatusCode::UNAUTHORIZED,
            AuthError::TokenRequestFailed(_)
            | AuthError::InvalidTokenError(_)
            | AuthError::JwtCreationFailed(_)
            | AuthError::OAuth2RequestTokenError(_)
            | AuthError::InvalidCallbackData => StatusCode::BAD_REQUEST,
            AuthError::JwtError(_) | AuthError::NetworkError(_) | AuthError::SerdeParseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
