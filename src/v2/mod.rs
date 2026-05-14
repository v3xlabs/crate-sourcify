//! Read-only Sourcify v2 API client.
//! See [`Client`] for the main entry point.

mod client;
mod types;

pub use client::Client;
pub use types::*;

/// Field names accepted by [`Client::get_contract_with_fields`].
pub mod field {
    /// Contract source files.
    pub const SOURCES: &str = "sources";
    /// Contract ABI.
    pub const ABI: &str = "abi";
    /// Solidity or Vyper metadata.
    pub const METADATA: &str = "metadata";
    /// Compiler and contract identity information.
    pub const COMPILATION: &str = "compilation";
    /// Deployment transaction information.
    pub const DEPLOYMENT: &str = "deployment";
    /// NatSpec user documentation.
    pub const USERDOC: &str = "userdoc";
    /// NatSpec developer documentation.
    pub const DEVDOC: &str = "devdoc";
    /// Storage layout information.
    pub const STORAGE_LAYOUT: &str = "storageLayout";
    /// Function, event, and error signatures extracted from the ABI.
    pub const SIGNATURES: &str = "signatures";
}
