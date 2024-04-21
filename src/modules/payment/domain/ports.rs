use async_trait::async_trait;

use super::{Payment, PaymentError, PaymentStatus};

#[async_trait]
pub trait DBRepository: Send + Sync {
    async fn get_payment_by_user(&self, user_id: i32) -> Result<Option<Payment>, PaymentError>;
    async fn create_payment(&self, payment: &Payment) -> Result<Payment, PaymentError>;
    async fn update_payment_status(
        &self,
        stripe_payment_id: &str,
        new_status: PaymentStatus,
    ) -> Result<(), PaymentError>;
}
