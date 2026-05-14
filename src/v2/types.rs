use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Public Sourcify server base URL used by the v2 client.
pub const DEFAULT_BASE_URL: &str = "https://sourcify.dev/server";

/// Sourcify's verification match quality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchLevel {
    /// Source code fully matches the on-chain bytecode.
    FullMatch,
    /// Source code partially matches the on-chain bytecode.
    PartialMatch,
    /// Sourcify v2 generic match value.
    #[serde(rename = "match")]
    Match,
    /// Sourcify v2 exact match value.
    ExactMatch,
}

/// Verified contract data returned by `GET /v2/contract/{chainId}/{address}`.
///
/// The v2 endpoint can return many fields depending on the `fields` query
/// parameter. Frequently used fields are typed, while unknown or less-common
/// fields are preserved in [`Contract::extra`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    /// Overall match quality.
    pub r#match: Option<MatchLevel>,
    /// Creation bytecode match quality.
    pub creation_match: Option<MatchLevel>,
    /// Runtime bytecode match quality.
    pub runtime_match: Option<MatchLevel>,
    /// EVM chain ID as returned by Sourcify.
    pub chain_id: String,
    /// Contract address.
    pub address: String,
    /// Verification timestamp when available.
    pub verified_at: Option<String>,
    /// Sourcify match identifier when available.
    pub match_id: Option<String>,
    /// Source files keyed by path.
    pub sources: Option<HashMap<String, SourceFile>>,
    /// Contract ABI JSON.
    pub abi: Option<serde_json::Value>,
    /// Compiler and contract identity information.
    pub compilation: Option<Compilation>,
    /// Solidity or Vyper metadata JSON.
    pub metadata: Option<serde_json::Value>,
    /// NatSpec user documentation.
    pub userdoc: Option<serde_json::Value>,
    /// NatSpec developer documentation.
    pub devdoc: Option<serde_json::Value>,
    /// Storage layout JSON.
    pub storage_layout: Option<serde_json::Value>,
    /// Deployment transaction information.
    pub deployment: Option<Deployment>,
    /// Proxy resolution data.
    pub proxy_resolution: Option<serde_json::Value>,
    /// Extracted function, event, and error signatures.
    pub signatures: Option<serde_json::Value>,

    /// Additional Sourcify fields not explicitly modeled by this crate.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Contract {
    /// Returns true when Sourcify reported any match for this contract.
    pub fn is_verified(&self) -> bool {
        self.r#match.is_some()
    }
}

/// A source file returned by Sourcify.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    /// File content.
    pub content: String,
}

/// Compiler and contract identity information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Compilation {
    /// Source language, usually `Solidity` or `Vyper`.
    pub language: Option<String>,
    /// Compiler executable name, for example `solc`.
    #[serde(rename = "compiler")]
    pub compiler_name: Option<String>,
    /// Compiler version.
    pub compiler_version: Option<String>,
    /// Contract name.
    pub name: Option<String>,
    /// Fully qualified contract name, such as `src/Token.sol:Token`.
    pub fully_qualified_name: Option<String>,
    /// Compiler settings JSON.
    pub settings: Option<serde_json::Value>,
}

/// Deployment transaction information when available.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    /// Deployment transaction hash.
    pub transaction_hash: Option<String>,
    /// Deployment block number.
    pub block_number: Option<String>,
    /// Deployer address.
    pub deployer: Option<String>,
}

/// Response envelope for all-chain contract lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllChainsResponse {
    /// Verification results for the address across chains.
    pub results: Vec<ContractSummary>,
}

/// Compact contract verification data returned by all-chain lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractSummary {
    /// Overall match quality.
    pub r#match: Option<MatchLevel>,
    /// Creation bytecode match quality.
    pub creation_match: Option<MatchLevel>,
    /// Runtime bytecode match quality.
    pub runtime_match: Option<MatchLevel>,
    /// EVM chain ID as returned by Sourcify.
    pub chain_id: String,
    /// Contract address.
    pub address: String,
    /// Verification timestamp when available.
    pub verified_at: Option<String>,
    /// Sourcify match identifier when available.
    pub match_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_contract_response() {
        let contract: Contract = serde_json::from_str(
            r#"{
                "match": "exact_match",
                "creationMatch": "match",
                "runtimeMatch": "match",
                "chainId": "1",
                "address": "0x0000000000000000000000000000000000000000",
                "verifiedAt": "2024-07-24T12:00:00Z",
                "matchId": "42",
                "sources": {
                    "src/Counter.sol": { "content": "contract Counter {}" }
                },
                "abi": [{ "type": "function", "name": "count" }],
                "compilation": {
                    "language": "Solidity",
                    "compiler": "solc",
                    "compilerVersion": "0.8.24",
                    "name": "Counter",
                    "fullyQualifiedName": "src/Counter.sol:Counter"
                },
                "unexpectedField": true
            }"#,
        )
        .unwrap();

        assert!(contract.is_verified());
        assert_eq!(contract.r#match, Some(MatchLevel::ExactMatch));
        assert_eq!(
            contract.compilation.unwrap().compiler_name.as_deref(),
            Some("solc")
        );
        assert_eq!(
            contract
                .sources
                .unwrap()
                .get("src/Counter.sol")
                .unwrap()
                .content,
            "contract Counter {}"
        );
        assert_eq!(
            contract.extra.get("unexpectedField"),
            Some(&serde_json::json!(true))
        );
    }

    #[test]
    fn deserializes_unverified_contract_response() {
        let contract: Contract = serde_json::from_str(
            r#"{
                "match": null,
                "creationMatch": null,
                "runtimeMatch": null,
                "chainId": "1",
                "address": "0x0000000000000000000000000000000000000000"
            }"#,
        )
        .unwrap();

        assert!(!contract.is_verified());
    }
}
