use std::{borrow::Borrow, sync::Arc};

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use stripe::{EventObject, EventType, Webhook};

use crate::{
    error::ApiError,
    modules::{auth::Claims, stripe::Service},
    utils::Config,
};

pub async fn get_products(service: web::Data<Arc<Service>>) -> Result<HttpResponse, ApiError> {
    let products = service.get_all_products().await?;
    Ok(HttpResponse::Ok().json(products))
}

pub async fn get_checkout(
    req: HttpRequest,
    service: web::Data<Arc<Service>>,
) -> Result<HttpResponse, ApiError> {
    //The middleware will take care if the claim is not present
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.sub;

        let url = service
            .create_checkout(user_id, "prod_Pz8TwS1quXKuWq")
            .await?;

        Ok(HttpResponse::Ok().json(url))
    } else {
        Err(ApiError::InternalServerError)
    }
}

pub async fn webhook_handler(
    req: HttpRequest,
    service: web::Data<Arc<Service>>,
    payload: web::Bytes,
) -> HttpResponse {
    handle_webhook(req, service, payload).await.unwrap();
    HttpResponse::Ok().finish()
}

pub async fn handle_webhook(
    req: HttpRequest,
    service: web::Data<Arc<Service>>,
    payload: web::Bytes,
) -> Result<(), ApiError> {
    let config = Config::from_env();
    let payload_str = std::str::from_utf8(payload.borrow()).unwrap();

    let stripe_signature = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    if let Ok(event) =
        Webhook::construct_event(payload_str, stripe_signature, &config.stripe_webhook_secret)
    {
        match event.type_ {
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    let _ = service.create_payment(session.id.as_str()).await?;
                }
            }

            // EventType::PaymentIntentSucceeded => {
            //     log::debug!("PaymentIntentSucceeded");
            //     if let EventObject::PaymentIntent(intent) = event.data.object {
            //         let _ = service
            //             .update_payment_status(intent.id.as_str(), true)
            //             .await?;
            //     }
            // }
            //
            // EventType::CheckoutSessionAsyncPaymentSucceeded => {
            //     log::debug!("CheckoutSessionAsyncPaymentSucceeded");
            //     if let EventObject::CheckoutSession(session) = event.data.object {}
            // }
            _ => {
                println!("Unknown event encountered in webhook: {:?}", event.type_);
            }
        }
    } else {
        println!("Failed to construct webhook event, ensure your webhook secret is correct.");
    }

    Ok(())
}

fn get_header_value<'b>(req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
    req.headers().get(key)?.to_str().ok()
}
