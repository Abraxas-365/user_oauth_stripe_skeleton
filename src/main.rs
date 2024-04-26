#![allow(dead_code)]

mod error;
mod modules;
mod utils;

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{
    modules::{
        auth::{
            self,
            provider::{google, OAuthProvider},
        },
        stripe_payments, subscription,
        user::{self},
    },
    utils::PostgresRepository,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,debug");
    env_logger::init();

    let repo = Arc::new(PostgresRepository::new().await);

    let user_service = Arc::new(user::Service::new(repo.clone()));
    let subscription_service = Arc::new(modules::subscription::Service::new(repo.clone()));
    let payment_service = Arc::new(stripe_payments::Service::new(
        repo.clone(),
        user_service.clone(),
        subscription_service.clone(),
    ));

    let oauth_google = Arc::new(google::Provider::new(user_service.clone()));

    log::info!("Starting HTTP server on 0.0.0.0:80...");
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .configure(auth::api::config)
            .configure(user::api::config)
            .configure(stripe_payments::api::config)
            .configure(subscription::api::config)
            .app_data(web::Data::new(payment_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(
                oauth_google.clone() as Arc<dyn OAuthProvider>
            ))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
