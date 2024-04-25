use async_trait::async_trait;
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    RedirectUrl, Scope, StandardTokenResponse, TokenResponse, TokenUrl,
};
use serde_json::Value;
use std::sync::Arc;

use crate::{
    error::ApiError,
    modules::{
        auth::{
            create_jwt, provider::OAuthProvider, AuthError, OAuthData, OAuthProviderType,
            OAuthResponse,
        },
        user,
    },
    utils::Config,
};

pub struct Provider {
    oauth_client: Arc<BasicClient>,
    user_service: Arc<user::Service>,
}

impl Provider {
    pub fn new(user_service: Arc<user::Service>) -> Self {
        let config = Config::from_env();

        let google_client_id = config.google_client_id;
        let google_client_secret = config.google_client_secret;
        let google_redirect_uri = config.google_redirect_uri;
        let client = BasicClient::new(
            ClientId::new(google_client_id),
            Some(ClientSecret::new(google_client_secret)),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_owned())
                .expect("Invalid authorization endpoint URL"),
            Some(
                TokenUrl::new("https://oauth2.googleapis.com/token".to_owned())
                    .expect("Invalid token endpoint URL"),
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(google_redirect_uri.into()).expect("Invalid redirect URI"),
        );
        Self {
            oauth_client: Arc::new(client),
            user_service,
        }
    }

    async fn exchange_token(
        &self,
        code: String,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, AuthError> {
        self.oauth_client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|err| AuthError::OAuth2RequestTokenError(err.to_string()))
    }

    async fn fetch_google_user_info(&self, access_token: &str) -> Result<Value, AuthError> {
        let user_info_url = "https://www.googleapis.com/oauth2/v3/userinfo";
        let client = reqwest::Client::new();
        Ok(client
            .get(user_info_url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|err| AuthError::NetworkError(err))?
            .json::<Value>()
            .await?)
    }

    fn extract_oauth_data(
        &self,
        token_response: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        user_info: &Value,
    ) -> Result<OAuthData, AuthError> {
        let refresh_token = token_response
            .refresh_token()
            .map(|token| token.secret().to_string())
            .ok_or_else(|| AuthError::InvalidTokenError("No refresh token received".to_string()))?;

        let user_identifier = extract_field(user_info, "sub")?;
        let name = extract_field(user_info, "given_name")?;
        let email = extract_field(user_info, "email")?;
        let image_url = user_info.get("picture").and_then(|val| val.as_str());

        Ok(OAuthData {
            provider: OAuthProviderType::Google,
            user_identifier,
            name,
            email,
            refresh_token,
            image_url: image_url.map(|url| url.to_string()),
        })
    }
}

#[async_trait]
impl OAuthProvider for Provider {
    async fn get_authorization_url(&self) -> (String, CsrfToken) {
        let scopes = vec!["email", "profile", "openid"];

        let (auth_url, csrf_state) = scopes
            .iter()
            .fold(
                self.oauth_client.authorize_url(CsrfToken::new_random),
                |url, scope| url.add_scope(Scope::new(scope.to_string())),
            )
            .add_extra_param("access_type", "offline")
            .add_extra_param("prompt", "consent")
            .url();

        (auth_url.to_string(), csrf_state)
    }

    async fn handle_oauth_callback(&self, code: String) -> Result<OAuthResponse, ApiError> {
        let token_response = self.exchange_token(code).await?;
        let user_info = self
            .fetch_google_user_info(token_response.access_token().secret())
            .await?;
        let oauth_data = self.extract_oauth_data(&token_response, &user_info)?;
        let user = self.user_service.sign_up_or_login(oauth_data).await?;
        let token = create_jwt(&user)?;
        Ok(OAuthResponse { user, token })
    }
}

fn extract_field(value: &Value, field: &str) -> Result<String, AuthError> {
    value
        .get(field)
        .ok_or_else(|| {
            AuthError::InvalidTokenError(format!("Missing '{}' field in UserInfo response", field))
        })
        .and_then(|v| {
            v.as_str().ok_or_else(|| {
                AuthError::InvalidTokenError(format!(
                    "Invalid '{}' field in UserInfo response",
                    field
                ))
            })
        })
        .map(|s| s.to_string())
}
