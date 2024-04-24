use crate::utils::middleware::jwt_validator;
use actix_web::web;
use actix_web_lab::middleware::from_fn;

use super::hander::{get_checkout, get_products};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/stripe/checkout")
            .route(web::post().to(get_checkout))
            .wrap(from_fn(jwt_validator)),
    )
    .service(web::resource("/stripe/products").route(web::get().to(get_products)));
}
