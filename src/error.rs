use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use stripe::{ParseIdError, StripeError};
use thiserror::Error;

use crate::modules::{
    auth::AuthError, stripe_payments::PaymentError, subscription::SubscriptionError,
    user::UserError,
}; // Import sqlx::Error if you're using SQLx.

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Unprocessable Entity")]
    UnprocessableEntity,

    #[error("Network Error: {0}")]
    NetworkError(String),

    #[error("Access Denied")]
    AccessDenied,

    #[error("Resource Not Found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Service Unavailable")]
    ServiceUnavailable,

    #[error("Database Connectivity Error")]
    DatabaseConnectivityError,

    #[error(transparent)]
    UserError(#[from] UserError),

    #[error(transparent)]
    SubscriptionError(#[from] SubscriptionError),

    #[error("Stripe error: {0}")]
    StripeError(#[from] StripeError),

    #[error("Stripe Id parse error")]
    ParseIdError(#[from] ParseIdError),

    #[error(transparent)]
    PaymentError(#[from] PaymentError),

    #[error(transparent)]
    AuthError(#[from] AuthError),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::StripeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::NetworkError(_) => StatusCode::BAD_GATEWAY,
            ApiError::AccessDenied => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::DatabaseConnectivityError => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::ParseIdError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UserError(ref e) => match e {
                UserError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                UserError::UserAlreadyExists => StatusCode::CONFLICT,
                UserError::UserNotFound => StatusCode::NOT_FOUND,
            },
            ApiError::PaymentError(ref e) => match e {
                PaymentError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                PaymentError::ItemNotFound => StatusCode::NOT_FOUND,
                PaymentError::PaymentNotFound => StatusCode::NOT_FOUND,
                PaymentError::CreateCheckoutError => StatusCode::INTERNAL_SERVER_ERROR,
                PaymentError::InvalidPaymentStatus(_) => StatusCode::BAD_REQUEST,
            },
            ApiError::AuthError(ref e) => match e {
                AuthError::AuthorizationFailed
                | AuthError::InvalidTokenError(_)
                | AuthError::JwtError(_)
                | AuthError::JwtCreationFailed(_) => StatusCode::UNAUTHORIZED,

                AuthError::TokenRequestFailed(_)
                | AuthError::OAuth2RequestTokenError(_)
                | AuthError::NetworkError(_)
                | AuthError::SerdeParseError(_) => StatusCode::BAD_GATEWAY,

                AuthError::InvalidCallbackData => StatusCode::BAD_REQUEST,
            },
            ApiError::SubscriptionError(ref e) => match e {
                SubscriptionError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                SubscriptionError::SubscriptionNotFound => StatusCode::NOT_FOUND,
            },
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ApiErrorResponse {
            error: self.to_string(),
        })
    }
}

#[derive(Serialize)]
struct ApiErrorResponse {
    error: String,
}
