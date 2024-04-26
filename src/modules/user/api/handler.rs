use std::sync::Arc;

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};

use crate::{
    error::ApiError,
    modules::{auth::Claims, user::Service},
};

pub async fn get_user(
    req: HttpRequest,
    service: web::Data<Arc<Service>>,
) -> Result<HttpResponse, ApiError> {
    //The middleware will take care if the claim is not present
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.sub;
        let user = service.get_user_by_id(user_id).await?;

        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::InternalServerError)
    }
}
