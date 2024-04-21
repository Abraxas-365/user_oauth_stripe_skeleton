use std::sync::Arc;

use super::{ports::DBRepository, Payment, PaymentError, PaymentStatus};

pub struct Service {
    repository: Arc<dyn DBRepository>,
}
impl Service {
    pub fn new(repository: Arc<dyn DBRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_payment(
        &self,
        user_id: i32,
        stripe_payment_id: &str,
    ) -> Result<Payment, PaymentError> {
        let payment = Payment::new(user_id, stripe_payment_id);
        self.repository.create_payment(&payment).await
    }

    pub async fn payment_status(&self, user_id: i32) -> Result<PaymentStatus, PaymentError> {
        match self.repository.get_payment_by_user(user_id).await {
            Ok(Some(payment)) => Ok(payment.get_payment_status()),
            Ok(None) => Err(PaymentError::NotFound),
            Err(e) => Err(e),
        }
    }

    pub async fn update_payment_status(
        &self,
        stripe_payment_id: &str,
        new_status: PaymentStatus,
    ) -> Result<(), PaymentError> {
        self.repository
            .update_payment_status(stripe_payment_id, new_status)
            .await
    }
}
