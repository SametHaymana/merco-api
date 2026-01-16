use oauth2::{AuthorizationCode, ClientId, ClientSecret, TokenResponse};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthUrl, RedirectUrl, TokenUrl};
use uuid::Uuid;

use crate::error::AuthError;

pub enum OAuthProvider {
    Google,
    GitHub,
    Apple,
    Facebook,
    Twitter,
    Discord,
    Microsoft,
    LinkedIn,
    CustomOidc(String),
}

pub struct OAuthService;

impl OAuthService {
    pub fn get_authorization_url(
        provider: &OAuthProvider,
        client_id: &str,
        client_secret: Option<&str>,
        redirect_uri: &str,
    ) -> Result<String, AuthError> {
        let (auth_url, token_url) = Self::get_provider_urls(provider)?;

        let client = BasicClient::new(
            ClientId::new(client_id.to_string()),
            client_secret.map(|s| ClientSecret::new(s.to_string())),
            AuthUrl::new(auth_url).map_err(|e| AuthError::OAuth(e.to_string()))?,
            Some(TokenUrl::new(token_url).map_err(|e| AuthError::OAuth(e.to_string()))?),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri.to_string()).map_err(|e| AuthError::OAuth(e.to_string()))?);

        let (auth_url, _csrf_token) = client
            .authorize_url(oauth2::CsrfToken::new_random)
            .url();

        Ok(auth_url.to_string())
    }

    pub async fn exchange_code(
        provider: &OAuthProvider,
        client_id: &str,
        client_secret: Option<&str>,
        redirect_uri: &str,
        code: &str,
    ) -> Result<OAuthUserInfo, AuthError> {
        let (auth_url, token_url) = Self::get_provider_urls(provider)?;

        let client = BasicClient::new(
            ClientId::new(client_id.to_string()),
            client_secret.map(|s| ClientSecret::new(s.to_string())),
            AuthUrl::new(auth_url).map_err(|e| AuthError::OAuth(e.to_string()))?,
            Some(TokenUrl::new(token_url).map_err(|e| AuthError::OAuth(e.to_string()))?),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri.to_string()).map_err(|e| AuthError::OAuth(e.to_string()))?);

        let token_result = client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|e| AuthError::OAuth(e.to_string()))?;

        // TODO: Fetch user info from provider's userinfo endpoint
        // For now, return placeholder
        Ok(OAuthUserInfo {
            provider_id: "".to_string(),
            email: None,
            name: None,
        })
    }

    fn get_provider_urls(provider: &OAuthProvider) -> Result<(String, String), AuthError> {
        match provider {
            OAuthProvider::Google => Ok((
                "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
                "https://oauth2.googleapis.com/token".to_string(),
            )),
            OAuthProvider::GitHub => Ok((
                "https://github.com/login/oauth/authorize".to_string(),
                "https://github.com/login/oauth/access_token".to_string(),
            )),
            OAuthProvider::Apple => Ok((
                "https://appleid.apple.com/auth/authorize".to_string(),
                "https://appleid.apple.com/auth/token".to_string(),
            )),
            OAuthProvider::Facebook => Ok((
                "https://www.facebook.com/v18.0/dialog/oauth".to_string(),
                "https://graph.facebook.com/v18.0/oauth/access_token".to_string(),
            )),
            OAuthProvider::Twitter => Ok((
                "https://twitter.com/i/oauth2/authorize".to_string(),
                "https://api.twitter.com/2/oauth2/token".to_string(),
            )),
            OAuthProvider::Discord => Ok((
                "https://discord.com/api/oauth2/authorize".to_string(),
                "https://discord.com/api/oauth2/token".to_string(),
            )),
            OAuthProvider::Microsoft => Ok((
                "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
                "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string(),
            )),
            OAuthProvider::LinkedIn => Ok((
                "https://www.linkedin.com/oauth/v2/authorization".to_string(),
                "https://www.linkedin.com/oauth/v2/accessToken".to_string(),
            )),
            OAuthProvider::CustomOidc(base_url) => Ok((
                format!("{}/authorize", base_url),
                format!("{}/token", base_url),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OAuthUserInfo {
    pub provider_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
}
