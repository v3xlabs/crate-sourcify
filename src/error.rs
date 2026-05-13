use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Invalid hash: {0}")]
    InvalidHash(String),

    #[error("Contract not verified")]
    NotVerified,

    #[error("Verification failed: {0}")]
    VerificationFailed(String),
}
