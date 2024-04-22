use std::sync::Arc;

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::modules::{
    auth::Claims,
    payment::{PaymentError, PaymentStatus, Service},
};

#[derive(Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    pub stripe_payment_id: String,
}
pub async fn create_payment(
    req: HttpRequest,
    data: web::Json<CreatePaymentRequest>,
    service: web::Data<Arc<Service>>,
) -> Result<HttpResponse, PaymentError> {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.sub;

        let payment = service
            .create_payment(user_id, &data.stripe_payment_id)
            .await?;

        Ok(HttpResponse::Ok().json(payment))
    } else {
        Err(PaymentError::AuthorizationFailed)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePaymentStatusRequest {
    pub stripe_payment_id: String,
    pub new_status: PaymentStatus,
}

pub async fn update_payment_status(
    data: web::Json<UpdatePaymentStatusRequest>,
    service: web::Data<Arc<Service>>,
) -> Result<HttpResponse, PaymentError> {
    service
        .update_payment_status(&data.stripe_payment_id, data.new_status.clone())
        .await?;

    Ok(HttpResponse::Ok().finish())
}
