use super::{LookupResponse, Signature, SignatureResponse, DEFAULT_BASE_URL};
use crate::{Error, Result};
use reqwest::Client as HttpClient;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Client {
    http: Arc<HttpClient>,
    base_url: String,
}

impl Client {
    /// Creates a 4byte client pointed at the public Sourcify API.
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self::with_base_url(http, DEFAULT_BASE_URL)
    }

    /// Creates a 4byte client pointed at a custom Sourcify-compatible API base URL.
    pub fn with_base_url(http: Arc<HttpClient>, base_url: impl Into<String>) -> Self {
        Self {
            http,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    /// Resolve a 4-byte function selector, for example `0xa9059cbb`.
    ///
    /// Returns an empty vector when the selector is valid but unknown.
    pub async fn lookup_function(&self, selector: impl AsRef<str>) -> Result<Vec<Signature>> {
        let selector = normalize_hash(selector.as_ref(), 4)?;
        let response = self
            .http
            .get(format!("{}/signature-database/v1/lookup", self.base_url))
            .query(&[("function", selector.as_str())])
            .send()
            .await?;
        let lookup = parse_lookup_response(response).await?;

        Ok(lookup
            .result
            .functions
            .and_then(|mut functions| functions.remove(&selector))
            .flatten()
            .unwrap_or_default())
    }

    /// Resolve a 32-byte event topic hash.
    ///
    /// Returns an empty vector when the topic is valid but unknown.
    pub async fn lookup_event(&self, topic: impl AsRef<str>) -> Result<Vec<Signature>> {
        let topic = normalize_hash(topic.as_ref(), 32)?;
        let response = self
            .http
            .get(format!("{}/signature-database/v1/lookup", self.base_url))
            .query(&[("event", topic.as_str())])
            .send()
            .await?;
        let lookup = parse_lookup_response(response).await?;

        Ok(lookup
            .result
            .event
            .and_then(|mut events| events.remove(&topic))
            .unwrap_or_default())
    }

    /// Search function and event signatures by name.
    ///
    /// The upstream API supports wildcard searches using `*` and `?`.
    pub async fn search(&self, query: impl AsRef<str>) -> Result<SignatureResponse> {
        let response = self
            .http
            .get(format!("{}/signature-database/v1/search", self.base_url))
            .query(&[("query", query.as_ref())])
            .send()
            .await?;
        Ok(parse_lookup_response(response).await?.result)
    }
}

async fn parse_lookup_response(response: reqwest::Response) -> Result<LookupResponse> {
    if !response.status().is_success() {
        return Err(Error::ApiError(format!(
            "Sourcify 4byte lookup failed with {}",
            response.status()
        )));
    }

    Ok(response.json::<LookupResponse>().await?)
}

fn normalize_hash(hash: &str, bytes: usize) -> Result<String> {
    let hash = hash.trim();
    let hex = hash.strip_prefix("0x").or_else(|| hash.strip_prefix("0X"));
    let expected_len = bytes * 2;

    match hex {
        Some(hex) if hex.len() == expected_len && hex.chars().all(|c| c.is_ascii_hexdigit()) => {
            Ok(format!("0x{}", hex.to_ascii_lowercase()))
        }
        _ => Err(Error::InvalidHash(hash.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_function_selector() {
        assert_eq!(normalize_hash("0XA9059CBB", 4).unwrap(), "0xa9059cbb");
    }

    #[test]
    fn accepts_event_topic() {
        assert_eq!(
            normalize_hash(
                "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                32
            )
            .unwrap(),
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        );
    }

    #[test]
    fn rejects_invalid_hash() {
        assert!(normalize_hash("0x1234", 4).is_err());
        assert!(normalize_hash("a9059cbb", 4).is_err());
        assert!(normalize_hash("0xa9059cbg", 4).is_err());
    }
}
