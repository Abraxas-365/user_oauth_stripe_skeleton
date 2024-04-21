use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::PaymentError;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Payment {
    pub stripe_payment_id: String,
    pub user_id: i32,
    pub payment_date: DateTime<Utc>,
    pub payment_status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub fn new(user_id: i32, stripe_payment_id: &str) -> Payment {
        Payment {
            stripe_payment_id: stripe_payment_id.to_string(),
            user_id,
            payment_date: Utc::now(),
            payment_status: PaymentStatus::Pending.into(),
        }
    }

    pub fn get_payment_status(&self) -> PaymentStatus {
        match self.payment_status.as_ref() {
            "pending" => PaymentStatus::Pending,
            "successful" => PaymentStatus::Successful,
            "failed" => PaymentStatus::Failed,
            "denied" => PaymentStatus::Denied,
            _ => {
                log::error!("Invalid payment status: {}", self.payment_status);
                PaymentStatus::Denied
            }
        }
    }
}
