use reqwest::Client;
use serde::{Deserialize, Serialize};


pub struct ExchangeCodePayload {
    pub code: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
    pub get_token_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuth2Tokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

pub async fn exchange_code_for_tokens(
    payload: ExchangeCodePayload,
) -> anyhow::Result<OAuth2Tokens> {
    let client = Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("code", payload.code.as_str()),
        ("redirect_uri", payload.redirect_uri.as_str()),
        ("client_id", payload.client_id.as_str()),
        ("client_secret", payload.client_secret.as_str()),
    ];

    let response = client
        .post(payload.get_token_url.as_str())
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let access_token = json.get("access_token").unwrap().as_str().unwrap();
        let refresh_token = json.get("refresh_token").unwrap().as_str().unwrap();
        let expires_in = json.get("expires_in").unwrap().as_u64().unwrap();

        Ok(OAuth2Tokens {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            expires_in: expires_in,
        })
    } else {
        let error_text = response.text().await?;
        println!("Failed to exchange code for tokens: {}", error_text);
        Err(anyhow::anyhow!("Failed to exchange code for tokens: {}", error_text))
    }
}

pub struct RefreshTokenPayload {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub get_token_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshedTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
}

pub async fn refresh_access_token(
    payload: RefreshTokenPayload,
) -> anyhow::Result<RefreshedTokenResponse> {
    let client = Client::new();
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", payload.refresh_token.as_str()),
        ("client_id", payload.client_id.as_str()),
        ("client_secret", payload.client_secret.as_str()),
    ];

   
    let response = client
        .post(payload.get_token_url.as_str())
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let access_token = json.get("access_token").unwrap().as_str().unwrap();
        let expires_in = json.get("expires_in").unwrap().as_u64().unwrap();

        Ok(RefreshedTokenResponse {
            access_token: access_token.to_string(),
            expires_in: expires_in,
        })
    } else {
        Err(anyhow::anyhow!("Failed to refresh access token"))
    }
}