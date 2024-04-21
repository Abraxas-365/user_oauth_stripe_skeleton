use async_trait::async_trait;

use crate::{
    modules::user::{ports::DBRepository, User, UserError},
    utils::PostgresRepository,
};

#[async_trait]
impl DBRepository for PostgresRepository {
    async fn update_user(&self, user: &User) -> Result<User, UserError> {
        let query = "
            UPDATE users 
            SET email = $1, 
                image_url = $2, 
                oauth_provider = $3, 
                oauth_id = $4, 
                oauth_refresh_token = $5, 
                created_at = $6
            WHERE id = $7
            RETURNING id, email, image_url, oauth_provider, oauth_id, oauth_refresh_token, created_at;
        ";
        sqlx::query_as::<_, User>(query)
            .bind(&user.email)
            .bind(&user.image_url)
            .bind(&user.oauth_provider)
            .bind(&user.oauth_id)
            .bind(&user.oauth_refresh_token)
            .bind(user.created_at)
            .bind(user.id)
            .fetch_one(&*self.pg_pool)
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
            INSERT INTO users (email, image_url, oauth_provider, oauth_id, oauth_refresh_token, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, image_url, oauth_provider, oauth_id, oauth_refresh_token, created_at;
        ";
        sqlx::query_as::<_, User>(query)
            .bind(&user.email)
            .bind(&user.image_url)
            .bind(&user.oauth_provider)
            .bind(&user.oauth_id)
            .bind(&user.oauth_refresh_token)
            .bind(user.created_at)
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(UserError::from)
    }
}
