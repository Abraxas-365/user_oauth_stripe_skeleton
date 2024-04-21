use async_trait::async_trait;

use super::{User, UserError};

#[async_trait]
pub trait DBRepository: Send + Sync {
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, UserError>;
    async fn update_user(&self, user: &User) -> Result<User, UserError>;
    async fn create_user(&self, user: &User) -> Result<User, UserError>;
}
