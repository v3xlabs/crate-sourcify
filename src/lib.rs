//! # sourcify
//!
//! A lightweight read-only wrapper for the Sourcify v2 API and Sourcify 4byte
//! signature API.
//!
//! The crate is intentionally small: use [`Sourcify::v2`] to retrieve verified
//! contract data, source files, ABI, and metadata by chain ID and address, and
//! use [`Sourcify::four_byte`] to resolve function selectors or event topics.
//!
//! ## Quick Start
//!
//! ```rust
//! use sourcify::Sourcify;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Sourcify::new();
//!
//!     // Fetch contract metadata from a verified contract
//!     let contract = client.v2().get_contract(
//!         1,
//!         "0x250b3e8E23d24C8b12dF5d0c4F62B2D1543E13b2",
//!     ).await?;
//!
//!     if let Some(c) = contract {
//!         println!("Contract verified: {}", c.address);
//!     }
//!
//!     Ok(())
//! }
//! ```

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
