//! # Sourcify Rust Client
//!
//! A lightweight read-only wrapper for the Sourcify V2 API and Sourcify 4byte API.
//!
//! ## Quick Start
//!
//! ```rust,no_run
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

pub struct Sourcify {
    v2_client: v2::Client,
    four_byte_client: four_byte::Client,
}

impl Sourcify {
    pub fn new() -> Self {
        Self::with_client(HttpClient::new())
    }

    pub fn with_client(client: HttpClient) -> Self {
        let http = Arc::new(client);
        Self {
            v2_client: v2::Client::new(Arc::clone(&http)),
            four_byte_client: four_byte::Client::new(Arc::clone(&http)),
        }
    }

    pub fn v2(&self) -> &v2::Client {
        &self.v2_client
    }

    pub fn four_byte(&self) -> &four_byte::Client {
        &self.four_byte_client
    }
}

impl Default for Sourcify {
    fn default() -> Self {
        Self::new()
    }
}
