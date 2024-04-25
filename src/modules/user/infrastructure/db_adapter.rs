use async_trait::async_trait;

use crate::{
    modules::user::{ports::Repository, User, UserError},
    utils::PostgresRepository,
};

#[async_trait]
impl Repository for PostgresRepository {
    async fn update_user(&self, user: &User) -> Result<User, UserError> {
        let query = "
        UPDATE users
        SET name = $1,
            email = $2,
            image_url = $3,
            oauth_provider = $4,
            oauth_id = $5,
            stripe_customer_id = $6,
            oauth_refresh_token = $7,
            created_at = $8
        WHERE id = $9
        RETURNING id, name, email, image_url, oauth_provider, oauth_id, stripe_customer_id, oauth_refresh_token, created_at;
    ";
        sqlx::query_as::<_, User>(query)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.image_url)
            .bind(&user.oauth_provider)
            .bind(&user.oauth_id)
            .bind(&user.stripe_customer_id)
            .bind(&user.oauth_refresh_token)
            .bind(&user.created_at)
            .bind(&user.id)
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(UserError::from)
    }

    async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, UserError> {
        let query = "SELECT * FROM users WHERE id = $1";
        sqlx::query_as::<_, User>(query)
            .bind(id)
            .fetch_optional(&*self.pg_pool)
            .await
            .map_err(UserError::from)
    }

    async fn get_user_by_customer_id(&self, customer_id: &str) -> Result<Option<User>, UserError> {
        let query = "SELECT * FROM users WHERE stripe_customer_id = $1";
        sqlx::query_as::<_, User>(query)
            .bind(customer_id)
            .fetch_optional(&*self.pg_pool)
            .await
            .map_err(UserError::from)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let query = "SELECT * FROM users WHERE email = $1";
        sqlx::query_as::<_, User>(query)
            .bind(email)
            .fetch_optional(&*self.pg_pool)
            .await
            .map_err(UserError::from)
    }

    async fn create_user(&self, user: &User) -> Result<User, UserError> {
        let query = "
        INSERT INTO users (name, email, image_url, oauth_provider, oauth_id, stripe_customer_id, oauth_refresh_token, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, name, email, image_url, oauth_provider, oauth_id, stripe_customer_id, oauth_refresh_token, created_at;
    ";
        sqlx::query_as::<_, User>(query)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.image_url)
            .bind(&user.oauth_provider)
            .bind(&user.oauth_id)
            .bind(&user.stripe_customer_id)
            .bind(&user.oauth_refresh_token)
            .bind(&user.created_at)
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(UserError::from)
    }
}
