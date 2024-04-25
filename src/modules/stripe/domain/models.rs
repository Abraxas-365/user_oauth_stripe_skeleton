use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::PaymentError;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Payment {
    pub stripe_payment_id: String,
    pub user_id: i32,
    pub stripe_product_id: String,
    pub payment_date: DateTime<Utc>,
    pub payment_status: PaymentStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "payment_status", rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,
    Successful,
    Failed,
    Denied,
}

impl TryFrom<String> for PaymentStatus {
    type Error = PaymentError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_ref() {
            "pending" => Ok(PaymentStatus::Pending),
            "successful" => Ok(PaymentStatus::Successful),
            "failed" => Ok(PaymentStatus::Failed),
            "denied" => Ok(PaymentStatus::Denied),
            _ => Err(PaymentError::InvalidPaymentStatus(s)),
        }
    }
}

impl ToString for PaymentStatus {
    fn to_string(&self) -> String {
        match self {
            PaymentStatus::Pending => "pending",
            PaymentStatus::Successful => "successful",
            PaymentStatus::Failed => "failed",
            PaymentStatus::Denied => "denied",
        }
        .to_string()
    }
}

impl Into<String> for PaymentStatus {
    fn into(self) -> String {
        match self {
            PaymentStatus::Pending => "pending".to_string(),
            PaymentStatus::Successful => "successful".to_string(),
            PaymentStatus::Failed => "failed".to_string(),
            PaymentStatus::Denied => "denied".to_string(),
        }
    }
}

impl Payment {
    pub fn new(user_id: i32, stripe_payment_id: &str, stripe_product_id: &str) -> Payment {
        Payment {
            stripe_payment_id: stripe_payment_id.to_string(),
            user_id,
            payment_date: Utc::now(),
            stripe_product_id: stripe_product_id.to_string(),
            payment_status: PaymentStatus::Pending.into(),
        }
    }
    pub fn with_status(mut self, status: PaymentStatus) -> Self {
        self.payment_status = status;
        self
    }
}
