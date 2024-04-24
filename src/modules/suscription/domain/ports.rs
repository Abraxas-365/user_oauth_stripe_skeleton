use async_trait::async_trait;

use super::{Subscription, SubscriptionError};

#[async_trait]
pub trait Repository {
    async fn get_subscription_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<Subscription>, SubscriptionError>;

    async fn create_subscription(
        &self,
        subscription: &Subscription,
    ) -> Result<Subscription, SubscriptionError>;
}
