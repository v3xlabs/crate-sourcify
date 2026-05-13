use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const DEFAULT_BASE_URL: &str = "https://api.4byte.sourcify.dev";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupResponse {
    pub ok: bool,
    pub result: SignatureResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureResponse {
    #[serde(rename = "function")]
    pub functions: Option<HashMap<String, Option<Vec<Signature>>>>,
    pub event: Option<HashMap<String, Vec<Signature>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub filtered: bool,
    #[serde(rename = "hasVerifiedContract")]
    pub has_verified_contract: bool,
}
