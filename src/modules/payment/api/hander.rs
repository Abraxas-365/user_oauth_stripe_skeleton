use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::modules::payment::{PaymentStatus, Service};

#[derive(Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    pub user_id: i32,
    pub stripe_payment_id: String,
}
pub async fn create_payment(
    data: web::Json<CreatePaymentRequest>,
    service: web::Data<Arc<Service>>,
) -> impl Responder {
    let result = service
        .create_payment(data.user_id, &data.stripe_payment_id)
        .await;
    match result {
        Ok(payment) => HttpResponse::Ok().json(payment),
        Err(err) => HttpResponse::InternalServerError().body(format!("{}", err)),
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
) -> impl Responder {
    let result = service
        .update_payment_status(&data.stripe_payment_id, data.new_status.clone())
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(format!("{}", err)),
    }
}
