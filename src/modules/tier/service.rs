use std::sync::Arc;

use super::{ports::Repository, Tier, TierError};

pub struct Service {
    repository: Arc<dyn Repository>,
}

impl Service {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }

    async fn get_tier_by_id(&self, id: i32) -> Result<Option<Tier>, TierError> {
        self.repository.get_tier_by_id(id).await
    }
}
