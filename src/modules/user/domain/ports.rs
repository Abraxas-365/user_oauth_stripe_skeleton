use async_trait::async_trait;

use super::{User, UserError};

#[async_trait]
pub trait Repository: Send + Sync {
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, UserError>;
    async fn get_user_by_customer_id(&self, customer_id: &str) -> Result<Option<User>, UserError>;
    async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, UserError>;
    async fn update_user(&self, user: &User) -> Result<User, UserError>;
    async fn create_user(&self, user: &User) -> Result<User, UserError>;
}
