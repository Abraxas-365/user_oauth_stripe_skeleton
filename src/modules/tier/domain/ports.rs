use async_trait::async_trait;

use super::{Tier, TierError};

#[async_trait]
pub trait Repository {
    async fn get_tier_by_id(&self, id: i32) -> Result<Option<Tier>, TierError>;
}
