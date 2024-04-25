use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserSubscription {
    pub user_id: i32,
    pub stripe_product_id: String,
    pub stripe_payment_id: String,
    pub subscription_date: DateTime<Utc>,
    pub is_active: bool,
}

impl UserSubscription {
    pub fn new(
        user_id: i32,
        stripe_product_id: String,
        stripe_payment_id: String,
        subscription_date: DateTime<Utc>,
    ) -> UserSubscription {
        UserSubscription {
            user_id,
            stripe_product_id,
            stripe_payment_id,
            subscription_date,
            is_active: true,
        }
    }
}
