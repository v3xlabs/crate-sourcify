//! # sourcify
//!
//! A lightweight read-only wrapper for [Sourcify](https://sourcify.dev),
//! including the Sourcify v2 API and Sourcify 4byte signature API.
//!
//! ## Supported APIs
//!
//! - [`v2::Client`] wraps the Sourcify v2 contract data API.
//!   - [API Docs](https://docs.sourcify.dev/docs/api/#server-api-documentation)
//!   - [OpenAPI](https://sourcify.dev/server/api-docs/swagger.json)
//! - [`four_byte::Client`] wraps the Sourcify 4byte signature API.
//!   - [API Docs](https://docs.sourcify.dev/docs/api/#4byte-signature-service-api-documentation)
//!   - [OpenAPI](https://api.4byte.sourcify.dev/api-docs/swagger.json)
//!
//! ## Limitations
//!
//! The crate is intentionally small: use [`Sourcify::v2`] to retrieve verified
//! contract data, source files, ABI, and metadata by chain ID and address, and
//! use [`Sourcify::four_byte`] to resolve function selectors or event topics.
//!
//! ## Contract Source Lookup
//!
//! ```rust,no_run
//! use sourcify::{v2, Sourcify};
//!
//! #[tokio::main]
//! async fn main() -> sourcify::Result<()> {
//!     let client = Sourcify::new();
//!     let contract = client
//!         .v2()
//!         .get_contract_with_fields(
//!             1,
//!             "0xdAC17F958D2ee523a2206206994597C13D831ec7",
//!             &[v2::field::SOURCES, v2::field::ABI, v2::field::METADATA],
//!         )
//!         .await?;
//!
//!     if let Some(contract) = contract {
//!         println!("verified on chain {}", contract.chain_id);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 4byte Lookup
//!
//! ```rust,no_run
//! use sourcify::Sourcify;
//!
//! #[tokio::main]
//! async fn main() -> sourcify::Result<()> {
//!     let client = Sourcify::new();
//!     let signatures = client.four_byte().lookup_function("0xa9059cbb").await?;
//!
//!     for signature in signatures {
//!         println!("{}", signature.name);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!

pub mod error;
pub mod four_byte;
pub mod v2;

pub use error::{Error, Result};
use reqwest::Client as HttpClient;
use std::sync::Arc;

/// Shared entry point for the Sourcify v2 and 4byte clients.
pub struct Sourcify {
    v2_client: v2::Client,
    four_byte_client: four_byte::Client,
}

impl Sourcify {
    /// Creates a client using a default [`reqwest::Client`].
    pub fn new() -> Self {
        Self::with_client(HttpClient::new())
    }

    /// Creates a client using a caller-provided [`reqwest::Client`].
    ///
    /// This is useful when you already have timeout, proxy, TLS, or middleware
    /// settings configured on a `reqwest` client.
    pub fn with_client(client: HttpClient) -> Self {
        let http = Arc::new(client);
        Self {
            v2_client: v2::Client::new(Arc::clone(&http)),
            four_byte_client: four_byte::Client::new(Arc::clone(&http)),
        }
    }

    /// Access the Sourcify v2 contract data API.
    pub fn v2(&self) -> &v2::Client {
        &self.v2_client
    }

    /// Access the Sourcify 4byte signature API.
    pub fn four_byte(&self) -> &four_byte::Client {
        &self.four_byte_client
    }
}

impl Default for Sourcify {
    fn default() -> Self {
        Self::new()
    }
}
