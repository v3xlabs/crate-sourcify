use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const DEFAULT_BASE_URL: &str = "https://sourcify.dev/server";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchLevel {
    FullMatch,
    PartialMatch,
    #[serde(rename = "match")]
    Match,
    ExactMatch,
}

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
    pub fn is_verified(&self) -> bool {
        self.r#match.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    pub content: String,
}

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
