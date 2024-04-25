use async_trait::async_trait;

use super::{SubscriptionError, UserSubscription};

#[async_trait]
pub trait Repository: Send + Sync {
    async fn get_subscription_by_user(
        &self,
        user_id: i32,
    ) -> Result<Option<UserSubscription>, SubscriptionError>;

    async fn create_subscription(
        &self,
        subscription: &UserSubscription,
    ) -> Result<UserSubscription, SubscriptionError>;

    async fn update_subscription(
        &self,
        subscription: &UserSubscription,
    ) -> Result<UserSubscription, SubscriptionError>;
}
