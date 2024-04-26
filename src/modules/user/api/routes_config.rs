use actix_web::web;
use actix_web_lab::middleware::from_fn;

use crate::utils::middleware::jwt_validator;

use super::get_user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .route(web::get().to(get_user))
            .wrap(from_fn(jwt_validator)),
    );
}
