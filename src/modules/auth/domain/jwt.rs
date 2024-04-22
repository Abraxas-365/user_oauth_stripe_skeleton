use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::modules::user::User;

use super::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: Option<i64>,
}

pub fn create_jwt(user: &User) -> Result<String, AuthError> {
    let expiration_seconds = 3600; // Define the expiration to be in 1 hour
    let claims = Claims {
        sub: user.id,
        exp: Some((Utc::now() + chrono::Duration::seconds(expiration_seconds)).timestamp()), // Create an unix timestamp
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    // Define a test user
    static TEST_USER_ID: i32 = 1;
    fn get_test_user() -> User {
        User {
            id: 1,
            email: "test".into(),
            oauth_provider: "google".into(),
            oauth_id: "1234".into(),
            oauth_refresh_token: "1234".into(),
            image_url: None,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_create_and_verify_jwt() {
        let user = get_test_user();
        let token = create_jwt(&user).expect("Failed to create JWT");

        // Verify the token
        let claims = verify_jwt(&token).expect("Failed to verify JWT");

        // Check if the 'sub' field in the token matches the user ID
        assert_eq!(claims.sub, TEST_USER_ID, "JWT 'sub' field mismatch");

        // Ensure that the 'exp' field is correctly set and not None
        assert!(claims.exp.is_some(), "JWT 'exp' field should not be None");
    }
}
