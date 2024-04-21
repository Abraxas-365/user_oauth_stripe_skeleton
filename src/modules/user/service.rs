use std::sync::Arc;

use crate::modules::auth::OAuthData;

use super::{ports::DBRepository, User, UserError};

pub struct Service {
    repository: Arc<dyn DBRepository>,
}
impl Service {
    pub fn new(repository: Arc<dyn DBRepository>) -> Self {
        Self { repository }
    }

    pub async fn sign_up_or_login(&self, oauth_data: OAuthData) -> Result<User, UserError> {
        match self.repository.get_user_by_email(&oauth_data.email).await? {
            Some(user) => {
                if user.oauth_refresh_token != oauth_data.refresh_token {
                    let updated_user = User {
                        oauth_refresh_token: oauth_data.refresh_token,
                        ..user
                    };
                    Ok(self.repository.update_user(&updated_user).await?)
                } else {
                    Ok(user)
                }
            }
            None => {
                let new_user = User::new(oauth_data);
                Ok(self.repository.create_user(&new_user).await?)
            }
        }
    }
}
