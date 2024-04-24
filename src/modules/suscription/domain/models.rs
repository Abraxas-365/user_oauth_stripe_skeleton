use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Subscription {
    pub id: i32,
    pub user_id: i32,
    pub tier_id: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub active: bool,
}

impl Subscription {
    pub fn new(user_id: i32, tier_id: i32) -> Self {
        Subscription {
            id: 0,
            user_id,
            tier_id,
            start_date: Utc::now(),
            end_date: None,
            active: true,
        }
    }

    pub fn with_end_date(mut self, end_date: DateTime<Utc>) -> Self {
        self.end_date = Some(end_date);
        self
    }
}
