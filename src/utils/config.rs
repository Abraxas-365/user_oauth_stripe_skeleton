use std::env;

pub struct Config {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub database_url: String,
    pub strip_secret: String,
    pub stripe_checkout_cancel_url: String,
    pub stripe_checkout_success_url: String,
    pub stripe_webhook_secret: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            google_client_id: env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set"),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
                .expect("GOOGLE_CLIENT_SECRET not set"),
            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI")
                .expect("GOOGLE_REDIRECT_URI not set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            strip_secret: env::var("STRIPE_SECRET").expect("STRIPE_SECRET not set"),
            stripe_checkout_cancel_url: env::var("STRIPE_CHECKOUT_CANCEL_URL")
                .expect("STRIPE_CHECKOUT_CANCEL_URL not set"),
            stripe_checkout_success_url: env::var("STRIPE_CHECKOUT_SUCCESS_URL")
                .expect("STRIPE_CHECKOUT_SUCCESS_URL not set"),
            stripe_webhook_secret: env::var("STRIPE_WEBHOOK_SECRET")
                .expect("STRIPE_WEBHOOK_SECRET not set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not set"),
        }
    }
}
