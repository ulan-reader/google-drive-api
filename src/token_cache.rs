use crate::auth;
use crate::error::Error;
use time::OffsetDateTime;
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct TokenCache {
    sa_path: String,
    token: Option<String>,
    expires_at: Option<i64>,
}

impl TokenCache {
    pub fn new(sa_path: String) -> Self {
        Self {
            sa_path,
            token: None,
            expires_at: None
        }
    }

    pub async fn get_token(&mut self) -> Result<String, Error> {
        let now = OffsetDateTime::now_utc().unix_timestamp();

        if let Some(exp) = self.expires_at {
            if let Some(token) = &self.token {
                if now < exp - 30 { // 30 секунд запас
                    return Ok(token.clone());
                }
            }
        }

        let (token, exp) = auth::get_access_token(&self.sa_path).await?;
        self.token = Some(token.clone());
        self.expires_at = Some(exp);
        Ok(token)
    }
}


#[derive(Clone)]
pub struct SharedTokenChache {
    inner: Arc<Mutex<TokenCache>>,
}

impl SharedTokenChache {
    pub fn new(sa_path: String) -> Self {
        Self {
            inner: Arc::new(Mutex::new(TokenCache::new(sa_path))),
        }
    }

    pub async fn get_token(&self) -> Result<String, Error> {
        let mut cache = self.inner.lock().await;
        cache.get_token().await
    }
}