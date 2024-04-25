use crate::utils::middleware::jwt_validator;
use actix_web::web;
use actix_web_lab::middleware::from_fn;

use super::handler::{get_checkout, get_products, webhook_handler};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/stripe/checkout")
            .route(web::post().to(get_checkout))
            .wrap(from_fn(jwt_validator)),
    )
    .service(web::resource("/stripe/products").route(web::get().to(get_products)))
    .service(web::resource("/stripe/webhook").route(web::post().to(webhook_handler)));
}
