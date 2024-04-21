use actix_web::web;

use super::hander::{create_payment, update_payment_status};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/payments/create").route(web::post().to(create_payment)))
        .service(web::resource("/payments/status").route(web::post().to(update_payment_status)));
}
