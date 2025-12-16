use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use time::{OffsetDateTime};
use reqwest::Client;

use crate::error::Error;

#[derive(Deserialize, Debug)] // Временно использую дебаг. А так его нужно убрать ради безопасности
pub struct ServiceAccount {
    // pub r#type: String,
    // pub project_id: String,
    // pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    // pub auth_uri: String,
    pub token_uri: String,
    // pub auth_provider_x509_cert_url: String,
    // pub client_x509_cert_url: String,
}

#[derive(Serialize, Debug)] // Временно использую дебаг. А так его нужно убрать ради безопасности
struct Claims<'a> {
    iss: &'a str,
    scope: &'a str,
    aud: &'a str,
    iat: i64,
    exp: i64
}


pub async fn get_access_token(
    sa_path: &str,
) -> Result<(String, i64), Error> {
    let data = std::fs::read_to_string(sa_path)?;
    let sa: ServiceAccount = serde_json::from_str(&data)?;

    let now = OffsetDateTime::now_utc().unix_timestamp();
    let exp = now + 3600; // токен действителен 1 час

    let claims = Claims {
        iss: &sa.client_email,
        scope: "https://www.googleapis.com/auth/drive",
        aud: &sa.token_uri,
        iat: now,
        exp: now + 3600,
    };

    let key = EncodingKey::from_rsa_pem(sa.private_key.as_bytes())?;
    let jwt = encode(&Header::new(Algorithm::RS256), &claims, &key)?;

    let client = Client::new();
    let resp = client
        .post(&sa.token_uri)
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("Token response: {:#?}", resp); // Временно для отладки

    let token = resp["access_token"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or(Error::Google("No access_token in response".to_string()))?;

    Ok((token, exp))
}