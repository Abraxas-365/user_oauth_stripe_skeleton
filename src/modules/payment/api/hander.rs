use std::sync::Arc;

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::modules::{
    auth::Claims,
    payment::{PaymentStatus, Service},
};

#[derive(Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    pub stripe_payment_id: String,
}
pub async fn create_payment(
    req: HttpRequest,
    data: web::Json<CreatePaymentRequest>,
    service: web::Data<Arc<Service>>,
) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.sub;
        println!("User ID: {}", user_id);

        let result = service
            .create_payment(user_id, &data.stripe_payment_id)
            .await;
        match result {
            Ok(payment) => HttpResponse::Ok().json(payment),
            Err(err) => HttpResponse::InternalServerError().body(format!("{}", err)),
        }
    } else {
        HttpResponse::Unauthorized().body("No valid JWT token found")
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
