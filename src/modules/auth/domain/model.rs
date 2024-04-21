use serde::{Deserialize, Serialize};

use crate::modules::user::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OAuthProviderType {
    Google,
}
impl Into<String> for OAuthProviderType {
    fn into(self) -> String {
        match self {
            OAuthProviderType::Google => "google".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthData {
    pub provider: OAuthProviderType,
    pub user_identifier: String,
    pub name: String,
    pub email: String,
    pub refresh_token: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthResponse {
    pub user: User,
    pub token: String,
}
