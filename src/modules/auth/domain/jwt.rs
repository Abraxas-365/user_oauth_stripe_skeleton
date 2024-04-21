use std::time::Duration;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::modules::user::User;

use super::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: Option<Duration>,
}

pub fn create_jwt(user: &User) -> Result<String, AuthError> {
    let claims = Claims {
        sub: user.id,
        exp: None,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("your_secret_key".as_ref()),
    )
    .map_err(|err| AuthError::JwtCreationFailed(err.to_string()))
}

pub fn verify_jwt(token: &str) -> Result<Claims, AuthError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("your_secret_key".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|err| AuthError::JwtError(err))
}
