use std::sync::Arc;

use crate::error::ApiError;

use super::{ports::Repository, UserSubscription};

pub struct Service {
    repository: Arc<dyn Repository>,
}

impl Service {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }

    pub async fn get_subscription_by_user(
        &self,
        user_id: i32,
    ) -> Result<Option<UserSubscription>, ApiError> {
        Ok(self.repository.get_subscription_by_user(user_id).await?)
    }

    pub async fn create_subscription(
        &self,
        subscription: &UserSubscription,
    ) -> Result<UserSubscription, ApiError> {
        let subscription = self.repository.create_subscription(subscription).await?;
        Ok(subscription)
    }

    pub async fn update_subscription(
        &self,
        subscription: &UserSubscription,
    ) -> Result<UserSubscription, ApiError> {
        let subscription = self.repository.update_subscription(subscription).await?;
        Ok(subscription)
    }
}
