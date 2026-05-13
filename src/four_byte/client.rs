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
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self::with_base_url(http, DEFAULT_BASE_URL)
    }

    pub fn with_base_url(http: Arc<HttpClient>, base_url: impl Into<String>) -> Self {
        Self {
            http,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

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
