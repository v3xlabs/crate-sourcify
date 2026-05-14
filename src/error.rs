//! Error and result types returned by this crate.

use thiserror::Error;

/// The crate-wide result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by the Sourcify clients.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP transport or response decoding failed.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON parsing failed.
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    /// The upstream API returned an unsuccessful HTTP status.
    #[error("API error: {0}")]
    ApiError(String),

    /// A contract address was not `0x`-prefixed 20-byte hex.
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    /// A selector or topic hash had the wrong format or length.
    #[error("Invalid hash: {0}")]
    InvalidHash(String),
}
