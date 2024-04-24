use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tier {
    pub id: i32,
    pub price: i64,
}
impl Tier {
    pub fn new(price: i64) -> Self {
        Tier { id: 0, price }
    }
}
