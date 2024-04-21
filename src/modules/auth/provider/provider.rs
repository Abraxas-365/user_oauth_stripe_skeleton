use async_trait::async_trait;
use oauth2::CsrfToken;

use crate::modules::auth::{AuthError, OAuthResponse};

#[async_trait]
pub trait OAuthProvider: Send + Sync {
    async fn get_authorization_url(&self) -> (String, CsrfToken);

    async fn handle_oauth_callback(&self, code: String) -> Result<OAuthResponse, AuthError>;
}
