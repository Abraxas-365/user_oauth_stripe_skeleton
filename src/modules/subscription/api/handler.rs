use std::sync::Arc;

use actix_web::{web, HttpResponse};

use crate::{error::ApiError, modules::subscription::Service};

pub async fn get_subscription(
    service: web::Data<Arc<Service>>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let user_id = user_id.into_inner();
    let subscription = service.get_subscription_by_user(user_id).await?;
    if let Some(subscription) = subscription {
        Ok(HttpResponse::Ok().json(subscription))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
