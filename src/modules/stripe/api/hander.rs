use std::{borrow::Borrow, sync::Arc};

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use stripe::{EventObject, EventType, Webhook};

use crate::modules::{
    auth::Claims,
    stripe::{PaymentError, Service},
};

pub async fn get_products(service: web::Data<Arc<Service>>) -> Result<HttpResponse, PaymentError> {
    let products = service.get_all_products().await?;
    Ok(HttpResponse::Ok().json(products))
}

pub async fn get_checkout(
    req: HttpRequest,
    service: web::Data<Arc<Service>>,
) -> Result<HttpResponse, PaymentError> {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.sub;

        let url = service
            .create_checkout(user_id, "prod_Pz8TwS1quXKuWq")
            .await?;

        Ok(HttpResponse::Ok().json(url))
    } else {
        Err(PaymentError::AuthorizationFailed)
    }
}
