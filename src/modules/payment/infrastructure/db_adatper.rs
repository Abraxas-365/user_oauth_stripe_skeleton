use crate::{
    modules::payment::{ports::DBRepository, Payment, PaymentError, PaymentStatus},
    utils::PostgresRepository,
};
use async_trait::async_trait;

#[async_trait]
impl DBRepository for PostgresRepository {
    async fn get_payment_by_user(&self, user_id: i32) -> Result<Option<Payment>, PaymentError> {
        let query = "SELECT * FROM payments WHERE user_id = $1";
        sqlx::query_as::<_, Payment>(query)
            .bind(user_id)
            .fetch_optional(&*self.pg_pool)
            .await
            .map_err(|e| e.into())
    }

    async fn create_payment(&self, payment: &Payment) -> Result<Payment, PaymentError> {
        let query = "
            INSERT INTO payments (stripe_payment_id,user_id, payment_date, payment_status)
            VALUES ($1, $2, $3, $4::payment_status )
            RETURNING *;
        ";
        sqlx::query_as::<_, Payment>(query)
            .bind(&payment.stripe_payment_id)
            .bind(payment.user_id)
            .bind(payment.payment_date)
            .bind(&payment.payment_status.to_string())
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(|e| e.into())
    }

    async fn update_payment_status(
        &self,
        stripe_payment_id: &str,
        new_status: PaymentStatus,
    ) -> Result<(), PaymentError> {
        let new_status_str: String = new_status.into();
        let query =
            "UPDATE payments SET payment_status = $1::payment_status  WHERE stripe_payment_id = $2";
        sqlx::query(query)
            .bind(new_status_str)
            .bind(stripe_payment_id)
            .execute(&*self.pg_pool)
            .await
            .map_err(|e| e.into())
            .map(|_| ())
    }
}
