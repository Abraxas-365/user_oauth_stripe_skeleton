use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Item in payment not found - likely due to missing line item or product information.")]
    ItemNotFound,

    #[error("Payment not found")]
    PaymentNotFound,

    #[error("Couldn't create checkout")]
    CreateCheckoutError,

    #[error("Invalid payment status: {0}")]
    InvalidPaymentStatus(String),

    #[error("Allready have this product")]
    AllreadyHaveProduct,
}
