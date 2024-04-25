use crate::{
    modules::subscription::{ports::Repository, SubscriptionError, UserSubscription},
    utils::PostgresRepository,
};
use async_trait::async_trait;

#[async_trait]
impl Repository for PostgresRepository {
    async fn get_subscription_by_user(
        &self,
        user_id: i32,
    ) -> Result<Option<UserSubscription>, SubscriptionError> {
        let query = "
            SELECT user_id, stripe_product_id, stripe_payment_id, subscription_date, is_active 
            FROM user_subscription 
            WHERE user_id = $1 and is_active = true";
        sqlx::query_as::<_, UserSubscription>(query)
            .bind(user_id)
            .fetch_optional(&*self.pg_pool)
            .await
            .map_err(SubscriptionError::from)
    }

    async fn create_subscription(
        &self,
        subscription: &UserSubscription,
    ) -> Result<UserSubscription, SubscriptionError> {
        let query = "
            INSERT INTO user_subscription (user_id, stripe_product_id, stripe_payment_id, subscription_date, is_active)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING user_id, stripe_product_id, stripe_payment_id, subscription_date, is_active";
        sqlx::query_as::<_, UserSubscription>(query)
            .bind(subscription.user_id)
            .bind(&subscription.stripe_product_id)
            .bind(&subscription.stripe_payment_id)
            .bind(subscription.subscription_date)
            .bind(subscription.is_active)
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(SubscriptionError::from)
    }

    async fn update_subscription(
        &self,
        subscription: &UserSubscription,
    ) -> Result<UserSubscription, SubscriptionError> {
        let query = "
            UPDATE user_subscription
            SET stripe_payment_id = $2, subscription_date = $3, is_active = $4
            WHERE user_id = $1 AND stripe_product_id = $5
            RETURNING user_id, stripe_product_id, stripe_payment_id, subscription_date, is_active";
        sqlx::query_as::<_, UserSubscription>(query)
            .bind(subscription.user_id)
            .bind(&subscription.stripe_payment_id)
            .bind(subscription.subscription_date)
            .bind(subscription.is_active)
            .bind(&subscription.stripe_product_id)
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(SubscriptionError::from)
    }
}
