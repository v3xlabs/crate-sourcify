use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const DEFAULT_BASE_URL: &str = "https://api.4byte.sourcify.dev";

/// Envelope returned by lookup and search endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupResponse {
    pub ok: bool,
    pub result: SignatureResponse,
}

/// Function and event signatures grouped by selector/topic hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureResponse {
    #[serde(rename = "function")]
    pub functions: Option<HashMap<String, Option<Vec<Signature>>>>,
    pub event: Option<HashMap<String, Vec<Signature>>>,
}

/// A decoded function or event signature candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub filtered: bool,
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
