use std::env;

pub struct Config {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub database_url: String,
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
        }
    }
}
