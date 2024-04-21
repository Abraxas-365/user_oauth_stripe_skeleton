use actix_web::web;

use crate::modules::auth::api::{oauth_callback, redirect_to_oauth};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/redirect", web::get().to(redirect_to_oauth))
            .route("/callback", web::get().to(oauth_callback)),
    );
}
