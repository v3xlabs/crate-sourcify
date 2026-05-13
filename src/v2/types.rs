use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const DEFAULT_BASE_URL: &str = "https://sourcify.dev/server";

/// Sourcify's verification match quality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchLevel {
    FullMatch,
    PartialMatch,
    #[serde(rename = "match")]
    Match,
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
    pub r#match: Option<MatchLevel>,
    pub creation_match: Option<MatchLevel>,
    pub runtime_match: Option<MatchLevel>,
    pub chain_id: String,
    pub address: String,
    pub verified_at: Option<String>,
    pub match_id: Option<String>,
    pub sources: Option<HashMap<String, SourceFile>>,
    pub abi: Option<serde_json::Value>,
    pub compilation: Option<Compilation>,
    pub metadata: Option<serde_json::Value>,
    pub userdoc: Option<serde_json::Value>,
    pub devdoc: Option<serde_json::Value>,
    pub storage_layout: Option<serde_json::Value>,
    pub deployment: Option<Deployment>,
    pub proxy_resolution: Option<serde_json::Value>,
    pub signatures: Option<serde_json::Value>,

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
    pub content: String,
}

/// Compiler and contract identity information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Compilation {
    pub language: Option<String>,
    #[serde(rename = "compiler")]
    pub compiler_name: Option<String>,
    pub compiler_version: Option<String>,
    pub name: Option<String>,
    pub fully_qualified_name: Option<String>,
    pub settings: Option<serde_json::Value>,
}

/// Deployment transaction information when available.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    pub transaction_hash: Option<String>,
    pub block_number: Option<String>,
    pub deployer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllChainsResponse {
    pub results: Vec<ContractSummary>,
}

/// Compact contract verification data returned by all-chain lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractSummary {
    pub r#match: Option<MatchLevel>,
    pub creation_match: Option<MatchLevel>,
    pub runtime_match: Option<MatchLevel>,
    pub chain_id: String,
    pub address: String,
    pub verified_at: Option<String>,
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
