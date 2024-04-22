use crate::utils::middleware::jwt_validator;
use actix_web::web;
use actix_web_lab::middleware::from_fn;

use super::hander::{create_payment, update_payment_status};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/payments/create")
            .route(web::post().to(create_payment))
            .wrap(from_fn(jwt_validator)),
    )
    .service(web::resource("/payments/status").route(web::post().to(update_payment_status)));
}
