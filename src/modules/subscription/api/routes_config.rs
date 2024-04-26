use actix_web::web;

use super::handler::get_subscription;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/subscription/{user_id}").route(web::get().to(get_subscription)));
}
