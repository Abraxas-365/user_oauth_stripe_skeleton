use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    Error, HttpMessage,
};
use actix_web_lab::middleware::Next;

use crate::modules::auth::verify_jwt;

// Middleware implementation
pub async fn jwt_validator(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Extract JWT from the Authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(token_str) = auth_header.to_str() {
            if let Some(token) = token_str.strip_prefix("Bearer ") {
                // Verify the token and proceed if valid
                match verify_jwt(token) {
                    Ok(claims) => {
                        // Insert claims into the request extensions so it can be accessed in handlers
                        req.extensions_mut().insert(claims);
                        return next.call(req).await;
                    }
                    Err(e) => {
                        return Err(ErrorUnauthorized(format!("Invalid token: {}", e)));
                    }
                }
            }
        }
    }

    Err(ErrorUnauthorized("No valid Bearer token found"))
}
