use std::{collections::HashMap, sync::Arc};

use actix_web::{web, HttpResponse, Responder};

use crate::modules::auth::{provider::OAuthProvider, AuthError};

pub async fn redirect_to_oauth(oauth_manager: web::Data<Arc<dyn OAuthProvider>>) -> impl Responder {
    let (url, _csrf_token) = oauth_manager.get_authorization_url().await;
    HttpResponse::Found()
        .append_header(("Location", url.as_str()))
        .finish()
}

pub async fn oauth_callback(
    oauth_manager: web::Data<Arc<dyn OAuthProvider>>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, AuthError> {
    if let Some(code) = query.get("code") {
        let token = oauth_manager
            .handle_oauth_callback(code.to_string())
            .await?;
        Ok(HttpResponse::Ok().json(token))
    } else {
        log::error!("Invalid callback data provided");
        Err(AuthError::InvalidCallbackData)
    }
}
