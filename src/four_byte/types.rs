use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Public Sourcify 4byte API base URL used by the 4byte client.
pub const DEFAULT_BASE_URL: &str = "https://api.4byte.sourcify.dev";

/// Envelope returned by lookup and search endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupResponse {
    /// Whether the upstream API considered the request successful.
    pub ok: bool,
    /// Grouped signature candidates.
    pub result: SignatureResponse,
}

/// Function and event signatures grouped by selector/topic hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureResponse {
    /// Function signatures keyed by 4-byte selector. Unknown functions are `None`.
    #[serde(rename = "function")]
    pub functions: Option<HashMap<String, Option<Vec<Signature>>>>,
    /// Event signatures keyed by 32-byte topic hash.
    pub event: Option<HashMap<String, Vec<Signature>>>,
}

/// A decoded function or event signature candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    /// Human-readable Solidity signature, for example `transfer(address,uint256)`.
    pub name: String,
    /// Whether the upstream API marked this result as filtered.
    pub filtered: bool,
    /// Whether this signature is associated with a verified contract.
    #[serde(rename = "hasVerifiedContract")]
    pub has_verified_contract: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_lookup_response_with_function_match() {
        let response: LookupResponse = serde_json::from_str(
            r#"{
                "ok": true,
                "result": {
                    "function": {
                        "0xa9059cbb": [{
                            "name": "transfer(address,uint256)",
                            "filtered": false,
                            "hasVerifiedContract": true
                        }]
                    }
                }
            }"#,
        )
        .unwrap();

        let signatures = response
            .result
            .functions
            .unwrap()
            .remove("0xa9059cbb")
            .unwrap()
            .unwrap();
        assert_eq!(signatures[0].name, "transfer(address,uint256)");
    }

    #[test]
    fn deserializes_lookup_response_with_missing_function_match() {
        let response: LookupResponse = serde_json::from_str(
            r#"{
                "ok": true,
                "result": {
                    "function": {
                        "0x12345678": null
                    }
                }
            }"#,
        )
        .unwrap();

        assert!(response
            .result
            .functions
            .unwrap()
            .get("0x12345678")
            .unwrap()
            .is_none());
    }
}
