use std::sync::Arc;

use super::{ports::Repository, Subscription, SubscriptionError};

pub struct Service {
    repository: Arc<dyn Repository>,
}
impl Service {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }

    pub async fn create_subscription(
        &self,
        subscription: &Subscription,
    ) -> Result<Subscription, SubscriptionError> {
        self.repository.create_subscription(subscription).await
    }

    async fn get_subscription_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<Subscription>, SubscriptionError> {
        self.repository.get_subscription_by_user_id(user_id).await
    }
}
