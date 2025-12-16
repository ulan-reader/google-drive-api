
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0} ")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Googple API error: {0}")]
    Google(String),

    // #[error("Invalid service account file")]
    // InvalidServiceAccount,

    #[error("Invalid file name")]
    InvalidFileName
}