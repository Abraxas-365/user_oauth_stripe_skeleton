use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::modules::auth::OAuthData;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub image_url: Option<String>,
    pub oauth_provider: String,
    pub oauth_id: String,
    pub oauth_refresh_token: String,
    pub created_at: DateTime<Utc>,
}
impl User {
    pub fn new(oauth_data: OAuthData) -> Self {
        User {
            id: 0,
            email: oauth_data.email,
            oauth_provider: oauth_data.provider.into(),
            oauth_id: oauth_data.user_identifier,
            oauth_refresh_token: oauth_data.refresh_token,
            image_url: oauth_data.image_url,
            created_at: Utc::now(),
        }
    }
}
