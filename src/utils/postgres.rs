use std::sync::Arc;

use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct PostgresRepository {
    pub pg_pool: Arc<PgPool>,
}

impl PostgresRepository {
    pub async fn new() -> Self {
        let conn_url = std::env::var("DATABASE_URL").expect("Need a DB_URL");

        // Append SSL parameters to the connection URL
        let conn_url = format!("{}", conn_url);

        let pool = PgPool::connect(&conn_url).await.unwrap();
        Self {
            pg_pool: Arc::new(pool),
        }
    }
}
